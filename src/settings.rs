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
use fast_config::Config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use const_format::concatcp;

const APP_PREFIX: &str = "rest-demo";
const CONFIG_FILE_PREFIX: &str = "./";
const CONFIG_FILE_PATH: &str = concatcp!(CONFIG_FILE_PREFIX, APP_PREFIX, "-config.toml");

#[derive(Debug, Deserialize, Serialize, Clone)]
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

lazy_static! {
    pub static ref CONFIG: Settings = Config::new(CONFIG_FILE_PATH, Settings::default())
        .expect("error loading config")
        .data
        .clone();
}
