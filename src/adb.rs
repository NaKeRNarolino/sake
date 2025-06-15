use std::fmt::format;
use std::fs::File;
use std::path::{Path, PathBuf};
use adb_client::{ADBDeviceExt, ADBServer, ADBServerDevice, ADBUSBDevice};
use subprocess::{Exec, Popen};
use crate::config::{ADBConfig, Config};
use crate::context::SakeContext;

fn delete_remote_folder<D: ADBDeviceExt>(
    device: &mut D,
    remote_folder_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let remote_path_str = remote_folder_path.to_str().ok_or("Invalid path")?;

    let command = format!("rm -rf {}", remote_path_str);

    println!("Executing shell command: {}", command);

    let mut stdout_buffer = Vec::new();
    let mut stderr_buffer = Vec::new();

    let result = device.shell_command(&[command.as_str()], &mut stdout_buffer);

    let stdout_str = String::from_utf8_lossy(&stdout_buffer);
    let stderr_str = String::from_utf8_lossy(&stderr_buffer);

    if !stdout_str.trim().is_empty() {
        println!("Shell command stdout: {}", stdout_str);
    }
    if !stderr_str.trim().is_empty() {
        eprintln!("Shell command stderr: {}", stderr_str);
    }

    match result {
        Ok(_) => {
            println!("Folder '{}' successfully deleted on Android device.", remote_path_str);
            Ok(())
        },
        Err(e) => {
            Err(e.into())
        }
    }
}


fn push_single_file<D: ADBDeviceExt>(
    device: &mut D,
    local_file: &Path,
    remote_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Pushing file: {:?} to {:?}", local_file, remote_path);
    let mut input_file = File::open(local_file)?;
    device.push(&mut input_file, &remote_path.to_str().unwrap())?;
    Ok(())
}

fn create_remote_dir<D: ADBDeviceExt>(
    device: &mut D,
    remote_dir_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!("mkdir -p {}", remote_dir_path.to_str().unwrap());
    println!("Executing shell command: {}", command);
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let result = device.shell_command(&[command.as_str()], &mut stdout);

    if let Err(e) = result {
        eprintln!("Error creating remote directory: {}", e);
        return Err(e.into());
    }

    let stderr_str = String::from_utf8_lossy(&stderr);
    if !stderr_str.trim().is_empty() {
        eprintln!("Shell command stderr: {}", stderr_str);
    }

    Ok(())
}

fn push_directory_recursive<D: ADBDeviceExt>(
    device: &mut D,
    local_source_dir: &Path,
    remote_base_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    create_remote_dir(device, remote_base_dir)?;

    for entry in std::fs::read_dir(local_source_dir)? {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path.strip_prefix(local_source_dir)?;

        let remote_target_path = remote_base_dir.join(relative_path);

        if path.is_dir() {
            push_directory_recursive(device, &path, &remote_target_path)?;
        } else if path.is_file() {
            push_single_file(device, &path, &remote_target_path)?;
        }
    }
    Ok(())
}

fn init_device() -> ADBServerDevice {
    let mut server = ADBServer::default();
    let device = server.get_device().unwrap();

    device
}

pub fn adb(sake_config: &Config, config: &ADBConfig, context: &SakeContext) {
    let mut device = init_device();

    if config.start_minecraft {
        start_minecraft(&mut device)
    }
    if config.push {
        push_to_adb(&sake_config, &context, &mut device)
    }
}
pub fn start_minecraft(device: &mut ADBServerDevice) {
    device.shell_command(&["am", "start", "-n", "com.mojang.minecraftpe/com.mojang.minecraftpe.MainActivity"], &mut std::io::stdout()).unwrap();
    // {
    //     Exec::shell(format!("{} shell am start -n com.mojang.minecraftpe/com.mojang.minecraftpe.MainActivity", find_adb()))
    // }.join().unwrap();
}

pub fn as_storage<'a, T>(path: T) -> PathBuf
where
    T: Into<PathBuf> + 'a
{
    let mut v = PathBuf::from("/storage/emulated/0");

    v.push(path.into());
    
    v
}

pub fn push_to_adb(config: &Config, ctx: &SakeContext, device: &mut ADBServerDevice) {
    delete_remote_folder(device, &as_storage("saketest/*")).unwrap();

    let release_bp = &ctx.release_dir.join("development_behavior_packs");
    let release_rp = &ctx.release_dir.join("development_resource_packs");

    delete_remote_folder(
        device,
        &as_storage(&format!("Android/data/com.mojang.minecraftpe/files/games/com.mojang/development_behavior_packs/{}_BP", config.meta.pack_name))
    ).unwrap();
    delete_remote_folder(
        device,
        &as_storage(&format!("Android/data/com.mojang.minecraftpe/files/games/com.mojang/development_resource_packs/{}_RP", config.meta.pack_name))
    ).unwrap();


    push_directory_recursive(
        device,
        release_bp,
        &as_storage("Android/data/com.mojang.minecraftpe/files/games/com.mojang/development_behavior_packs/")
    ).unwrap();

    push_directory_recursive(
        device,
        release_rp,
        &as_storage("Android/data/com.mojang.minecraftpe/files/games/com.mojang/development_resource_packs/")
    ).unwrap()
    // push_directory_recursive(device, &PathBuf::from("./release"), &PathBuf::from("/storage/emulated/0/saketest")).unwrap()
}
// Android/data/com.mojang.minecraftpe/files/games/com.mojang