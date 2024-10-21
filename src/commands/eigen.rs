use anyhow::{bail, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn update_eigen() -> Result<()> {
    let (os, arch) = detect_os_and_arch()?;
    let url = get_download_url(os, arch)?;
    let cargo_bin = get_cargo_bin()?;
    let eigen_path = cargo_bin.join("eigen");

    println!("Downloading Eigen binary for {}/{}", os, arch);
    let status = Command::new("curl")
        .args(["-L", "-o", eigen_path.to_str().unwrap(), &url])
        .status()?;

    if !status.success() {
        bail!("Failed to download Eigen binary");
    }

    println!("Making Eigen binary executable");
    let status = Command::new("chmod")
        .args(["+x", eigen_path.to_str().unwrap()])
        .status()?;

    if !status.success() {
        bail!("Failed to make Eigen binary executable");
    }

    println!(
        "Eigen update completed successfully. Installed to: {:?}",
        eigen_path
    );

    Ok(())
}

fn detect_os_and_arch() -> Result<(&'static str, &'static str)> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    match (os, arch) {
        ("macos", "aarch64") => Ok(("macos", "arm64")),
        ("macos", "x86_64") => Ok(("macos", "x86_64")),
        ("linux", "x86_64") => Ok(("linux", "x86_64")),
        _ => bail!("Unsupported OS/architecture combination: {}/{}", os, arch),
    }
}

fn get_download_url(os: &str, arch: &str) -> Result<String> {
    let base_url = "https://github.com/tensor-foundation/eigen/releases/latest/download";
    let filename = format!("eigen-{}-{}", os, arch);
    Ok(format!("{}/{}", base_url, filename))
}

fn get_cargo_bin() -> Result<PathBuf> {
    let home = env::var("HOME").expect("HOME environment variable not set");
    let cargo_bin = PathBuf::from(home).join(".cargo").join("bin");

    if !cargo_bin.exists() {
        bail!("Cargo bin directory not found: {:?}", cargo_bin);
    }

    Ok(cargo_bin)
}
