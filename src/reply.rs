use ping_rs::{PingApiOutput, PingError};

/// The overall status of a reply
pub enum ReplyStatus {
    /// success, including an RTT value (in milliseconds)
    SuccessTimed(u32),
    /// Failure: no reply in time allowed
    TimedOut,
    /// Failure: unspecified
    OtherFailure
}


impl From<PingApiOutput> for ReplyStatus {
    fn from(value: PingApiOutput) -> Self {
        match value {
            Ok(p) => Self::SuccessTimed(p.rtt),
            Err(PingError::TimedOut) => Self::TimedOut,
            Err(_) => Self::OtherFailure
        }
    }
}
