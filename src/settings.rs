//! Settings
//! (credit for most of the good ideas:
//! https://blog.logrocket.com/configuration-management-in-rust-web-services/
//! )
//!
//! Config file is xxx.toml
//! Structure is:
//!
//! ```
//! # global settings
//! [main]
//! level = "info"
//! # settings for the web server
//! [server]
//! addr = "0.0.0.0:3000"
//! ...
//! # settings for the database and views
//! [db]
//! ```

use crate::server;
#[allow(unused)]
use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;


const APP_PREFIX: &str = "rest-demo";
//todo: is there a way to catenate `const` strings and avoid repeating the literal?
const CONFIG_FILE_PATH: &str = "rest-demo-config.toml";
#[allow(unused)]
const CONFIG_FILE_PREFIX: &str = "./";

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub main: crate::Settings,
    pub server: server::Settings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            main: crate::Settings::default(),
            server: server::Settings::default(),
        }
    }
}

impl Settings {
    fn load() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH).required(false))
            .add_source(Environment::with_prefix(APP_PREFIX).prefix_separator("__"))
            .build()?;

        builder.try_deserialize()
    }
}


lazy_static! {
    pub static ref CONFIG: Settings = Settings::load().expect("error loading config");
}
