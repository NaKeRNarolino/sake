use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use subprocess::{Exec, Redirection};
use crate::config::{ActionSource, Config, ModeConfig};
use crate::fs::path;

pub fn run_actions(mode: &ModeConfig, config: &Config) {
    for (name, props) in &mode.include_actions {
        let action_def = match config.actions.get(name) {
            None => {
                log::warn!("Couldn't find action `{}`, perhaps You forgot to define it in `actions`.", &name);
                continue
            }
            Some(v) => {
                log::info!("Running action `{}`\n", &name);
                v
            }
        };

        let action = if let ActionSource::Path(v) = &action_def.source {
            v
        } else {
            log::warn!("`web::` and `core::` actions are not supported yet.");
            continue
        };

        log::info!("{}", crate::fs::canonical_str("./.sake/temp"));

        let process = {
            Exec::shell(&format!("{}", action))
        }.stdout(Redirection::Pipe).cwd(crate::fs::canonical_str("./.sake/temp/src")).capture().unwrap();

        println!("{}", String::from_utf8(process.stdout).unwrap());
    }

}