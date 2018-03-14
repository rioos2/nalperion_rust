// Copyright 2018 The Rio Advancement Inc
//

//! A module containing the middleware of the HTTP server

use error::Result;
use nalperion::Nalperion;
use config;

#[derive(Debug)]
pub struct Client {
    pub nalp: Nalperion,
}

impl Client {
    pub fn new(config: config::LicensesCfg) -> Self {
        Client { nalp: Nalperion::new(config) }
    }

    // Returns the status of license verified with nalperion
    // If there is a chance for starting a trial, then it does.
    // If there is the activation code then it used that to verify.
    pub fn create_trial_or_verify(&self) -> Result<()> {
        self.nalp.verify()
    }
}
