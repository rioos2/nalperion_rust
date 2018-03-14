///// Configuration structure for validating license

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LicensesCfg {
    /// The standard object (.so)  path to use
    pub so_file: String,
    /// The activation license code bought by the customer (or) we will assume we are on trial mode.
    pub activation_code: Option<String>,
}

impl Default for LicensesCfg {
    fn default() -> Self {
        LicensesCfg {
            so_file: "ShaferFilechck.so".to_string(),
            activation_code: None,
        }
    }
}
