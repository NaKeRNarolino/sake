use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use fs_extra::dir::CopyOptions;
use subprocess::{Exec, Redirection};
use uuid::Uuid;
use crate::config::{Action, ActionSource, Config, ConfigMeta, ModeConfig};
use crate::{actions, windows};
use crate::context::SakeContext;

pub fn find_release_dir() -> PathBuf {
    let platform = std::env::consts::OS;

    if platform == "windows" {
        windows::find_release_dir()
    } else {
        log::error!("Platforms other than Windows are not supported.");
        exit(0)
    }
}

pub fn read_config(context: &SakeContext) -> String {
    let mut conf = context.project_dir.clone();

    conf.push("config.json");

    match fs::read_to_string(conf) {
        Ok(content) => content,
        Err(e) => {
            log::error!("Sake cannot read the config file: {e}");
            exit(0);
        }
    }
}

pub fn path(src: impl AsRef<OsStr>) -> PathBuf {
    if std::env::var("SAKE_DEV").unwrap_or("false".into()) == "true" {
        let mut buf = PathBuf::new();

        let v = PathBuf::from(&src);

        buf.push("tests");
        buf.push(v);

        buf
    } else {
        PathBuf::from(&src)
    }
}

pub fn acquire_lock(uuid: Uuid) -> bool {
    let platform = std::env::consts::OS;

    if platform == "windows" {
        windows::acquire_lock(uuid)
    } else {
        panic!("Platforms other than Windows are not supported.")
    }
}

pub fn clear_lock() {
    let platform = std::env::consts::OS;

    if platform == "windows" {
        windows::clear_lock()
    } else {
        panic!("Platforms other than Windows are not supported.")
    }
}

pub fn copy_packs(meta: &ConfigMeta, context: &SakeContext) {
    let mut bp_source = context.sake_dir.clone(); // path("./.sake/temp/src/BP")
    bp_source.push("temp");
    bp_source.push("src");
    bp_source.push("BP");
    let mut bp_res = find_release_dir();

    bp_res.push("development_behavior_packs");
    bp_res.push(format!("{}_BP", &meta.pack_name));

    let opt = CopyOptions::new().content_only(true);

    fs_extra::dir::remove(&bp_res).unwrap();

    match fs_extra::dir::copy(
        bp_source,
        bp_res,
        &opt
    ) {
        Ok(_) => {
            log::info!("Successfully compiled BP folder");
        }
        Err(v) => {
            log::error!("Couldn't copy BP folder, error: {:?}", v);
            exit(0);
        }
    }

    let mut rp_source = context.sake_dir.clone(); // path("./.sake/temp/src/BP")
    rp_source.push("temp");
    rp_source.push("src");
    rp_source.push("RP");
    let mut rp_res = find_release_dir();

    rp_res.push("development_resource_packs");
    rp_res.push(format!("{}_RP", &meta.pack_name));

    let opt = CopyOptions::new().content_only(true);

    fs_extra::dir::remove(&rp_res).unwrap();

    match fs_extra::dir::copy(
        rp_source,
        rp_res,
        &opt
    ) {
        Ok(_) => {
            log::info!("Successfully compiled RP folder");
        }
        Err(v) => {
            log::error!("Couldn't copy RP folder, error: {:?}", v);
            exit(0);
        }
    }
}

pub fn temp_operations(mode: &ModeConfig, config: &Config, context: &SakeContext) {
    let mut dir = context.project_dir.clone();
    dir.push("src");

    let mut res = context.sake_dir.clone();
    res.push("temp");
    res.push("src");

    match fs::create_dir_all(&res) {
        Ok(_) => {
            log::info!("Creating temporary environment for actions...");
        }
        Err(e) => {
            log::error!("Couldn't create temporary environment, error: {:?}", e);
            exit(0);
        }
    }

    println!("{}", res.display());

    match fs_extra::dir::remove(&res) {
        Ok(_) => {
            log::info!("Cleared the temporary environment for actions\n")
        }
        Err(e) => {
            log::error!("Couldn't remove the temporary environment, error: {:?}", e);
            exit(0);
        }
    };

    let opt = CopyOptions::new().content_only(true);

    match fs_extra::dir::copy(
        dir,
        &res,
        &opt
    ) {
        Ok(_) => {
            log::info!("Successfully created a temporary environment for actions");
        }
        Err(v) => {
            log::error!("Couldn't create a temporary environment, error: {:?}", v);
            exit(0);
        }
    }

    actions::run_actions(mode, config, &context);
}

pub fn clear_temp(context: &SakeContext) {
    let mut res = context.sake_dir.clone();
    res.push("temp");
    res.push("src");

    match fs_extra::dir::remove(res) {
        Ok(_) => {
            log::info!("Cleared the temporary environment for actions")
        }
        Err(e) => {
            log::error!("Couldn't remove the temporary environment, error: {:?}", e);
            exit(0);
        }
    };
}

pub fn canonical_path(p: impl AsRef<OsStr>) -> PathBuf {
    fs::canonicalize(path(p)).unwrap()
}

pub fn canonical_str(p: impl AsRef<OsStr>) -> String {
    if cfg!(target_os = "windows") {
        canonical_path(p).to_string_lossy().to_string().strip_prefix("\\\\?\\").unwrap().to_string()
    } else {
        canonical_path(p).to_string_lossy().to_string()
    }
}