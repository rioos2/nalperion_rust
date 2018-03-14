#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate libloading as lib_load;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate failure;
extern crate rand;

mod config;
mod licensor;
mod error;
mod nalperion;
use config::LicensesCfg;

fn main() {
    let client = licensor::Client::new(LicensesCfg::default());
    let result = client.create_trial_or_verify();
    match result {
        // return Ok when license is trial or acivated
        Ok(()) => println!("You have the valid License"),
        Err(err) => println!("{}", format!("{}", err)),
    }
}
