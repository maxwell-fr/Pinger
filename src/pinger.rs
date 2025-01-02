use std::sync::{mpsc, Arc, RwLock};
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
    signal_tx: Sender<Signal>,

    /// The data block
    data: Arc<RwLock<Target>>
}

/// The signals a thread can understand
enum Signal {
    Stop,
    Pause,
    Resume
}

const PING_OPTS: PingOptions = PingOptions { ttl: 128, dont_fragment: true };

impl Pinger {
    pub fn new(target: Target) -> Pinger {
        let (signal_tx, signal_rx) = mpsc::channel();
        let data = Arc::new(RwLock::new(target));
        let target_data = data.clone();
        let thread_handle = thread::spawn(move || { Pinger::go(target_data, signal_rx ) });
        Pinger {
            thread_handle,
            signal_tx,
            data
        }
    }

    fn go(target_data: Arc<RwLock<Target>>, signal_rx: Receiver<Signal>) {
        let ping_data = [8; 8]; // TODO: make this cooler
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

            //get target config data
            let (timeout, sleep_time, addr) = {
                let target = target_data.read().unwrap();
                (target.timeout_duration(), target.sleep_duration(), target.addr())
            };

            //do the ping and update the data
            if !paused {
                let res = send_ping(&addr, timeout, &ping_data, Some(&PING_OPTS));
                let mut target = target_data.write().unwrap();

                target.update_from_reply(res.into());
            }
            thread::sleep(sleep_time);
        }
    }

    pub fn get_handle(&self) -> &JoinHandle<()> {
        &self.thread_handle
    }

    pub fn get_data(&self) -> Target {
        self.data.read().unwrap().clone()
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
