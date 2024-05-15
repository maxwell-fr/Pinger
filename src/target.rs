use std::cmp::{max, min};
use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::net::IpAddr;
use crate::ReplyStatus;

/// Contains configuration and historical details for a given target
#[derive(Debug, Clone)]
pub struct Target {
    friendly_name: String,
    address: IpAddr,

    options: TargetOptions,

    /// Most recent RTT (round trip time) in milliseconds
    last_rtt: u32,
    /// Average RTT
    avg_rtt: u32,
    /// highest RTT
    max_rtt: u32,
    /// lowest RTT
    min_rtt: u32,

    /// The most recent RTT values
    rtt_hist: VecDeque<u32>,

    /// Error count
    /// An error increments, while a success decrements (if > 0)
    error_count: u32,
}

/// Contains Configuration information for a Target
#[derive(Debug, Copy, Clone)]
pub struct TargetOptions {
    /// RTT above this value is considered "high"
    pub rtt_high_threshold: u32,

    /// How long to wait (in seconds) between checks
    pub sleep_period_sec: u32,
    /// How long to wait (in seconds) for a response
    pub timeout_period_sec: u32,

    /// How many failed responses in a row are considered a problem
    pub error_count_threshold: u32,
}





impl Target {
    const HIST_SIZE: usize = 120;

    /// Create a new Target object with the needed parameters.
    pub fn new(friendly_name: String, address: IpAddr, sleep_period_sec: u32, timeout_period_sec: u32,
               rtt_high_threshold: u32, error_count_threshold: u32) -> Target {
        Target {
            friendly_name,
            address,
            last_rtt: 0,
            avg_rtt: 0,
            max_rtt: 0,
            min_rtt: u32::MAX,
            rtt_hist: VecDeque::with_capacity(Self::HIST_SIZE),
            error_count: 0,
            options: TargetOptions {
                rtt_high_threshold,
                sleep_period_sec,
                timeout_period_sec,
                error_count_threshold
            }
        }
    }

    /// Update the Target data based on the ReplyStatus
    pub fn update_from_reply(&mut self, reply_status: ReplyStatus) {
        match reply_status {
            ReplyStatus::SuccessTimed(rtt) => {
                self.push_rtt(rtt);
                self.last_rtt = rtt;
                self.update_rtt_avg();
                if self.error_count > 0 {
                    self.error_count -= 1;
                }
            }
            ReplyStatus::TimedOut | ReplyStatus::OtherFailure => {
                self.error_count += 1;
            }
        }

    }

    /// Helper function to push to the Deque and trim it if full
    fn push_rtt(&mut self, rtt: u32) {
        if self.rtt_hist.len() >= Self::HIST_SIZE {
            self.rtt_hist.pop_front();
        }

        self.rtt_hist.push_back(rtt);
        self.max_rtt = max(self.max_rtt, rtt);
        self.min_rtt = min(self.min_rtt, rtt);
    }

    /// Helper function to compute and store the RTT average
    fn update_rtt_avg(&mut self) {
        let total: u32 = self.rtt_hist.iter().sum();
        self.avg_rtt = total / self.rtt_hist.len() as u32;
    }

    /// Get the target IpAddr
    pub fn addr(&self) -> IpAddr {
        self.address.clone()
    }

    /// Get a front-to-back (oldest to newest) iterator
    pub fn hist_iter(&self) -> Iter<u32> {
        self.rtt_hist.iter()
    }

    pub fn error_count(&self) -> u32 {
        self.error_count
    }

    pub fn last_rtt(&self) -> u32 {
        self.last_rtt
    }

    pub fn avg_rtt(&self) -> u32 {
        self.avg_rtt
    }

    pub fn min_rtt(&self) -> u32 {
        self.min_rtt
    }

    pub fn max_rtt(&self) -> u32 {
        self.max_rtt
    }

    pub fn get_options(&self) -> &TargetOptions {
        &self.options
    }


}

