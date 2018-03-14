// Copyright 2018 The Rio Advancement Inc
//

//! A module containing the middleware of the HTTP server
use std::str;
use rand::{self, Rng};

use error::{Result, Error};
use handlebars::Handlebars;
use failure::SyncFailure;

use config;
use lib_load;

const ROOT_PATH: &'static str = "/home/vinov/rioos/home/license/";
const NALPERION_SHAFER_FILECHK_XML_TEMPLATE: &'static str = include_str!("../tools/shafer_filechk.hbs");

/// These are the security values stamped into your library.
/// They should be changed to match your vaues.
const AUTH_1: u32 = 6;
const AUTH_2: u32 = 502;
const AUTH_3: u32 = 234;

/// These values should be set to your customer ID and
/// product ID. They are used to verify that the library
/// being accessed corresponds to your product.
const CUSTOMER_ID: u32 = 4979;
const PRODUCT_ID: u32 = 0100;

const NALP_LIB_OPEN: &'static str = "NalpLibOpen";
const NALP_VALIDATE_LIBRARY: &'static str = "NSLValidateLibrary";
const NALP_CLOSED_LIBRARY: &'static str = "NalpLibClose";
const NALP_GET_LIBRARY: &'static str = "NSLGetLicense";


#[derive(Debug)]
pub struct Nalperion {
    fascade: API,
}

impl Nalperion {
    pub fn new(config: config::LicensesCfg) -> Self {
        Nalperion {
            fascade: API::new(
                config.so_file.to_string(),
                config.activation_code,
            ),
        }
    }

    // Returns the status of license verified with nalperion
    pub fn verify(&self) -> Result<()> {
        self.fascade.check_license()?;
        Ok(())
    }
}

#[derive(Debug)]
struct API {
    so_file: String,
    activation_code: Option<String>,
    secret_value: u32,
}

impl API {
    fn new(so_file: String, activation_code: Option<String>) -> Self {
        API {
            so_file: so_file,
            activation_code: activation_code,
            secret_value: rand::thread_rng().gen_range(0, 500),
        }
    }

    fn secret_offset(&self) -> (u32, u32) {
        (
            AUTH_1 + ((self.secret_value * AUTH_2) % AUTH_3),
            self.secret_value,
        )
    }

    fn check_license(&self) -> Result<()> {
        Self::call_dynamic(self.so_file.clone(), self.secret_offset(),self.activation_code.clone())?;
        Ok(())
    }


    fn call_dynamic(so_file: String, secret_offset: (u32, u32),activation_code:Option<String>) -> Result<()> {

        let lib = lib_load::Library::new(ROOT_PATH.to_string() + &so_file)?;

        unsafe {
            // open the nsl library and initialize the lib
            let open_fn = lib.get::<fn(&[u8]) -> i32>(NALP_LIB_OPEN.as_bytes())?;
            let ret_val = open_fn(shaferchk_xml_as_bytes(secret_offset.1)?.as_bytes());
            if ret_val < 0 {
                return NalperionResult::from_err(NALP_LIB_OPEN);
            }

            //validate the library with customer id and product id
            let validate_fn = lib.get::<fn(u32, u32) -> i32>(
                NALP_VALIDATE_LIBRARY.as_bytes(),
            )?;
            let response = validate_fn(CUSTOMER_ID, PRODUCT_ID);
            if response - secret_offset.0 as i32 != 0 {
                return NalperionResult::from_err(NALP_VALIDATE_LIBRARY);
            }

            //check the status of the license (license status has negative value return the error)
            let get_license_fn = lib.get::<fn(Option<String>, *mut i32, Option<String>) -> i32>(
                NALP_GET_LIBRARY.as_bytes(),
            )?;
            let x: &mut i32 = &mut 0;
            let ret_val = get_license_fn(activation_code, x, None);
            if ret_val - secret_offset.0 as i32 != 0 {
                return NalperionResult::from_err(NALP_GET_LIBRARY);
            }
            if *x < 0 {
                return NalperionResult::from_value(*x);
            }

            // lib must be close (if not close `core dump` error occurs)
            let free_fn = lib.get::<fn() -> i32>(NALP_CLOSED_LIBRARY.as_bytes())?;
            let ret_val = free_fn();
            if ret_val < 0 {
                return NalperionResult::from_err(NALP_CLOSED_LIBRARY);
            }
            Ok(())
        }
    }
}

fn shaferchk_xml_as_bytes(secret_value: u32) -> Result<String> {
    let json = json!({
        "secVal": secret_value,
        "WorkDir": ROOT_PATH
    });

    let r = Handlebars::new()
        .render_template(NALPERION_SHAFER_FILECHK_XML_TEMPLATE, &json)
        .map_err(SyncFailure::new);

    let write_content = r.unwrap()
        .lines()
        .filter(|l| *l != "")
        .collect::<Vec<_>>()
        .join("\n") + "\n";

    Ok(write_content.to_string())
}

enum NalperionResult {}

impl NalperionResult {
    pub fn from_value(v: i32) -> Result<()> {
        match v {
            // Error can Generated based on nalperion error code refer link: https://naldoc.atlassian.net/wiki/spaces/NND/pages/426049/Developers+API+Latest
            -1 => Err(Error::ProductExpired),
            -113 => Err(Error::TrialExpired),
            -116 => Err(Error::SubscriptionExpired),
            _ => Ok(()),
        }
    }

    pub fn from_err(name: &str) -> Result<()> {
        match name {
            NALP_LIB_OPEN => Err(Error::LicenseAPINotFound),
            NALP_VALIDATE_LIBRARY => Err(Error::LicenseAPIMustBeValid),
            NALP_GET_LIBRARY => Err(Error::LicenseCodeMustBeValid),
            NALP_CLOSED_LIBRARY => Err(Error::LicenseAPIMustBeInConsistentState),
            _ => Ok(()),
        }
    }
}
