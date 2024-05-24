use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;

use ping_rs::{PingOptions, send_ping};

use crate::Target;

/// Maintains state of a thread and its signal channel
pub struct Pinger {
    /// The thread handle for this object
    thread_handle: JoinHandle<()>,

    /// The signalling channel for this object
    signal_tx: Sender<Signal>
}

enum Signal {
    Stop,
    Pause,
    Resume
}

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };

impl Pinger {
    pub fn new(target: Target, channel_tx: Sender<Box<Target>>) -> Pinger {
        let (signal_tx, signal_rx) = mpsc::channel();
        let thread_handle = thread::spawn(move || { Pinger::go(target, signal_rx, channel_tx) });
        Pinger {
            thread_handle,
            signal_tx
        }
    }

    fn go(target: Target, signal_rx: Receiver<Signal>, channel_tx: Sender<Box<Target>>) {
        let mut target = Box::new(target);
        let data = [8; 8];
        let timeout = target.timeout_duration();
        let sleep_time = target.sleep_duration();
        let mut paused = false;
        loop {
            //check for a signal
            match signal_rx.try_recv() {
                Ok(Signal::Stop) => break,
                Ok(Signal::Pause) => paused = true,
                Ok(Signal::Resume) => paused = false,
                Err(TryRecvError::Empty) => {},
                Err(TryRecvError::Disconnected) => break
            }
            // do the ping and send a clone of the updated target object onward
            if !paused {
                let res = send_ping(&target.addr(), timeout, &data, Some(&PING_OPTS));

                target.update_from_reply(res.into());

                if channel_tx.send(target.clone()).is_err() {
                    break;
                }
            }
            thread::sleep(sleep_time);
        }
    }

    pub fn get_handle(&self) -> &JoinHandle<()> {
        &self.thread_handle
    }

    pub fn stop(&self) {
        self.signal_tx.send(Signal::Stop).expect("Stop signal should not fail but did.");
    }

    pub fn pause(&self) {
        self.signal_tx.send(Signal::Pause).expect("Pause signal should not fail but did.");
    }

    pub fn resume(&self) {
        self.signal_tx.send(Signal::Resume).expect("Resume signal should not fail but did.");
    }
}
