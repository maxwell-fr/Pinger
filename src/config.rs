use crate::Target;
use serde::{Serialize, Deserialize};

/// Stores configuration parameters
#[derive(Serialize, Deserialize)]
pub struct Config {
    scale: f32,
    targets: Vec<Target>
}

impl Config {
    /// add a Target to the list of configured ones
    pub fn add_target(&mut self, target: Target) {
        self.targets.push(target);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            scale: 1.0,
            targets: vec![]
        }
    }
}

