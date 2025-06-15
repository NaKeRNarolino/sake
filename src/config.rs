use std::collections::HashMap;
use std::fmt::{format, Formatter};
use std::process::exit;
use serde::{Serialize, Deserialize, Deserializer, Serializer};
use serde::de::Visitor;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Windows,
    Mac,
    Linux
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub enforces_platform: Platform,
    pub default_mode: String,
    pub modes: HashMap<String, ModeConfig>,
    pub actions: HashMap<String, Action>,
    pub adb: Option<ADBConfig>,
    pub meta: ConfigMeta
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub source: ActionSource
}

pub enum ActionSource {
    Core(String),
    Web(String),
    Path(String)
}

impl<'de> Deserialize<'de> for ActionSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let x = String::deserialize(deserializer)?;

        if x.starts_with("core::") {
            Ok(ActionSource::Core(x.trim_start_matches("core::").to_string()))
        } else if x.starts_with("web::") {
            Ok(ActionSource::Web(x.trim_start_matches("web::").to_string()))
        } else if x.starts_with("path::") {
            Ok(ActionSource::Path(x.trim_start_matches("path::").to_string()))
        } else {
            log::error!("Cannot parse action source: {}", x);
            exit(0)
        }
    }
}

impl Serialize for ActionSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let x = match self {
            ActionSource::Core(v) => {
                format!("core::{v}")
            }
            ActionSource::Web(v) => {
                format!("web::{v}")
            }
            ActionSource::Path(v) => {
                format!("path::{v}")
            }
        };

        serializer.serialize_str(&x)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModeConfig {
    pub watch: bool,
    pub target: ModeTarget,
    pub include_actions: HashMap<String, Value>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeTarget {
    Release,
    Preview
}

#[derive(Serialize, Deserialize)]
pub struct ConfigMeta {
    pub pack_name: String
}

#[derive(Serialize, Deserialize)]
pub struct ADBConfig {
    pub start_minecraft: bool,
    pub push: bool
}