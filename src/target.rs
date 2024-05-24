use std::cmp::{max, min};
use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::net::IpAddr;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::ReplyStatus;

/// Contains configuration and historical details for a given target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    /// Friendly name appears the largest
    friendly_name: String,

    /// The IP Address of the target
    address: IpAddr,

    /// Most recent RTT (round trip time) in milliseconds
    last_rtt: Option<u32>,
    /// Average RTT
    avg_rtt: Option<u32>,
    /// highest RTT
    max_rtt: Option<u32>,
    /// lowest RTT
    min_rtt: Option<u32>,

    /// The most recent RTT values
    rtt_hist: VecDeque<u32>,

    /// Error count
    /// An error increments, while a success decrements (if > 0)
    error_count: u32,

    /// RTT above this value is considered "high"
    rtt_high_threshold: u32,

    /// How long to wait (in seconds) between checks
    sleep_duration: u32,
    /// How long to wait (in seconds) for a response
    timeout_duration: u32,

    /// How many failed responses in a row are considered a problem
    error_count_threshold: u32
}


impl Target {
    const HIST_SIZE: usize = 120;

    /// Create a new Target object with the needed parameters.
    pub fn new(friendly_name: String, address: IpAddr, sleep_period_ms: u32,
               timeout_period_ms: u32, rtt_high_threshold_ms: u32, error_count_threshold: u32) -> Target {
        Target {
            friendly_name,
            address,
            last_rtt: None,
            avg_rtt: None,
            max_rtt: None,
            min_rtt: None,
            rtt_hist: VecDeque::with_capacity(Self::HIST_SIZE),
            error_count: 0,
            rtt_high_threshold: rtt_high_threshold_ms,
            sleep_duration: sleep_period_ms,
            timeout_duration: timeout_period_ms,
            error_count_threshold
        }
    }

    /// Update the Target data based on the ReplyStatus
    pub fn update_from_reply(&mut self, reply_status: ReplyStatus) {
        match reply_status {
            ReplyStatus::SuccessTimed(rtt) => {
                self.push_rtt(rtt);
                self.last_rtt = Some(rtt);
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
        self.max_rtt = Some(max(self.max_rtt.unwrap_or(0), rtt));
        self.min_rtt = Some(min(self.min_rtt.unwrap_or(u32::MAX), rtt));
    }

    /// Helper function to compute and store the RTT average
    fn update_rtt_avg(&mut self) {
        let total: u32 = self.rtt_hist.iter().sum();
        self.avg_rtt = Some(total / self.rtt_hist.len() as u32);
    }

    /// Get the target IpAddr
    pub fn addr(&self) -> IpAddr {
        self.address
    }

    /// Get a front-to-back (oldest to newest) iterator
    pub fn hist_iter(&self) -> Iter<u32> {
        self.rtt_hist.iter()
    }

    pub fn error_count(&self) -> u32 {
        self.error_count
    }

    pub fn last_rtt(&self) -> Option<u32> {
        self.last_rtt
    }

    pub fn avg_rtt(&self) -> Option<u32> {
        self.avg_rtt
    }

    pub fn min_rtt(&self) -> Option<u32> {
        self.min_rtt
    }

    pub fn max_rtt(&self) -> Option<u32> {
        self.max_rtt
    }

    pub fn name(&self) -> &str {
        &self.friendly_name
    }

    pub fn rtt_high_threshold(&self) -> Duration {
        Duration::from_millis(self.rtt_high_threshold as u64)
    }

    pub fn sleep_duration(&self) -> Duration {
        Duration::from_millis(self.sleep_duration as u64)
    }

    pub fn timeout_duration(&self) -> Duration {
        Duration::from_millis(self.timeout_duration as u64)
    }

    pub fn error_count_threshold(&self) -> u32 {
        self.error_count_threshold
    }

}

//impl Default for Target {
//    fn default() -> Self {
//        Target {
//            friendly_name: "Default".to_string(),
//            address: IpAddr::from([127,0,0,1]),
//            last_rtt: None,
//            avg_rtt: None,
//            max_rtt: None,
//            min_rtt: None,
//            rtt_hist: Default::default(),
//            error_count: 0,
//            rtt_high_threshold: Default::default(),
//            sleep_duration: Default::default(),
//            timeout_duration: Default::default(),
//            error_count_threshold: 0,
//        }
//    }
//}