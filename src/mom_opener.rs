use std::fs::File;
use std::path::Path;
use log::{error, info};
use opener;

pub struct Moms {
    pub moms: Vec<String>,
}

impl Moms {
    pub fn add(&mut self, mom: String) {
        self.moms.push(mom);
    }

    pub fn open_all(&self) {
        for mom in &self.moms {
            match opener::open(mom) {
                Ok(_) => info!("Opened {:?}", mom),
                Err(e) => error!("Error opening {:?}: {:?}", mom, e),
            }
        }
    }
}