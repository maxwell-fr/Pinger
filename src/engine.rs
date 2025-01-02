use std::{collections::HashMap, net::IpAddr};

use crate::{Pinger, Target};


pub struct Engine {
    pingers: HashMap<IpAddr, Pinger>
}

impl Engine {
    pub fn new() -> Self {
        let pingers = HashMap::new();

        Engine {
            pingers
        }
    }

    pub fn add(&mut self, target: Target) {
        self.pingers.entry(target.addr()).or_insert_with(|| {
            Pinger::new(target)
        });
    }

    pub fn get(&self, addr: &IpAddr) -> Option<Target> {
        self.pingers.get(addr).map(|p| p.get_data())
    }

    pub fn get_addresses(&self) -> Vec<IpAddr> {
        self.pingers.keys().copied().collect()
    }

}


impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
