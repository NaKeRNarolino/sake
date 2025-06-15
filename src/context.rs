use std::env::VarError;
use std::path::PathBuf;
use std::process::exit;

pub struct SakeContext {
    pub project_dir: PathBuf,
    pub sake_dir: PathBuf,
    pub release_dir: PathBuf
}

impl SakeContext {
    pub(crate) fn new() -> Self {
        let sake_dir = match std::env::var("SAKE_DOTSAKE_DIR") {
            Ok(v) => { PathBuf::from(v) }
            Err(e) => {
                log::error!("Sake can't find the .sake directory, it should be defined in .env, did you modify it?");
                exit(0);
            }
        };
        let project_dir = match std::env::var("SAKE_PROJECT_DIR") {
            Ok(v) => { PathBuf::from(v) }
            Err(e) => {
                log::error!("Sake can't find the project directory, it should be defined in .env, Did you delete or remove the key?");
                exit(0);
            }
        };
        
        let release_dir = match std::env::var("SAKE_COMMOJANG_DIR") {
            Ok(v) => { PathBuf::from(v) }
            Err(e) => {
                log::error!("Sake can't find the release directory, it should be defined in .env, Did you delete or remove the key?");
                exit(0);
            }
        };

        Self {
            project_dir,
            sake_dir,
            release_dir
        }
    }
}