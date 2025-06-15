use std::env::VarError;
use std::fs;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::process::exit;
use uuid::Uuid;
use crate::fs::path;

pub fn find_release_dir() -> PathBuf {
    // let local_app_data = match std::env::var("LOCALAPPDATA") {
    //     Ok(v) => { v }
    //     Err(e) => {
    //         match e {
    //             VarError::NotPresent => {
    //                 log::error!("The variable LOCALAPPDATA is not present in the environment.");
    //                 exit(0)
    //             }
    //             VarError::NotUnicode(_) => {
    //                 log::error!("The variable LOCALAPPDATA does not contain proper data in the environment.");
    //                 exit(0)
    //             }
    //         }
    //     }
    // };
    // 
    // let mut buf = PathBuf::from(
    //     local_app_data
    // );
    // 
    // buf.push("Packages");
    // buf.push("Microsoft.MinecraftUWP_8wekyb3d8bbwe");
    // buf.push("LocalState");
    // buf.push("games");
    // buf.push("com.mojang");
    // 
    let buf = PathBuf::from(std::env::var("SAKE_COMMOJANG_DIR").unwrap());
    
    buf
}

pub fn acquire_lock(current_uuid: Uuid) -> bool {
    let lock = fs::read_to_string(
        path("./.sake/lock.lock")
    ).unwrap_or("none".into());

    if lock == "none" {
        fs::write(
            path("./.sake/lock.lock"),
            current_uuid.to_string()
        ).unwrap();
        true
    } else {
        log::error!("Cannot acquire the lock file. Is another instance of Sake running?");
        exit(0)
    }
}

pub fn clear_lock() {
    fs::write(
        path("./.sake/lock.lock"),
        "none"
    ).unwrap();
}
