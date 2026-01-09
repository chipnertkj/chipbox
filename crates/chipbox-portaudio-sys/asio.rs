const _ASSERT_WINDOWS: () = assert!(cfg!(windows));

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use chipbox_build::miette::{self, Context as _, IntoDiagnostic as _};

/// Get the `asio` target subdirectory.
pub fn target_subdir() -> miette::Result<PathBuf> {
    let dir = chipbox_build::fs::cargo_target_subdir("asio")
        .into_diagnostic()
        .wrap_err("get cargo target subdir")?;
    Ok(dir)
}

const fn sdk_archive_filename() -> &'static str {
    "asiosdk.zip"
}
pub const fn sdk_extracted_dir_name() -> &'static str {
    "ASIOSDK"
}

#[derive(PartialEq, Eq)]
enum SdkCached {
    Yes,
    NotDownloaded,
    NotExtracted,
}

/// Check if the `asiodir` subdirectory exists under the target directory.
fn sdk_cached() -> miette::Result<SdkCached> {
    let try_exists = |path: &Path| -> miette::Result<bool> {
        path.try_exists()
            .into_diagnostic()
            .wrap_err("check dir exists")
    };
    // Get target subdir.
    let subdir = target_subdir().wrap_err("get target subdir")?;
    // Check if asiosdk dir exists.
    if !try_exists(&subdir)? {
        return Ok(SdkCached::NotDownloaded);
    }
    // Check if downloaded and/or at least extracted
    let extracted_path = subdir.join(sdk_extracted_dir_name());
    let archive_path = subdir.join(sdk_archive_filename());
    let status = if !try_exists(&extracted_path)? {
        if !try_exists(&archive_path)? {
            SdkCached::NotDownloaded
        } else {
            SdkCached::NotExtracted
        }
    } else {
        SdkCached::Yes
    };
    Ok(status)
}

/// Download the ASIO SDK.
fn download_sdk_archive() -> miette::Result<()> {
    println!("cargo:warning=ASIO SDK not found, downloading...");
    // Ensure subdir exists.
    let subdir = target_subdir().wrap_err("get target subdir")?;
    fs::create_dir_all(&subdir)
        .into_diagnostic()
        .wrap_err("create subdir")?;
    // Download file from the url to file.
    let url = "https://www.steinberg.net/asiosdk";
    let path = subdir.join(sdk_archive_filename());
    download_file(url, &path).wrap_err("download sdk")?;
    Ok(())
}

fn download_file(url: &str, p: impl AsRef<Path>) -> miette::Result<()> {
    println!("cargo:warning=Downloading ASIO SDK...");
    let path = p.as_ref();
    let mut resp = ureq::get(url)
        .call()
        .into_diagnostic()
        .wrap_err("get request")?;
    let mut out = fs::File::create(path)
        .into_diagnostic()
        .wrap_err("create output file")?;
    let mut reader = resp.body_mut().as_reader();
    io::copy(&mut reader, &mut out)
        .into_diagnostic()
        .wrap_err("copy contents")?;
    Ok(())
}

fn extract_sdk_archive() -> miette::Result<()> {
    let subdir = target_subdir().wrap_err("get target subdir")?;
    let archive_path = subdir.join(sdk_archive_filename());
    let file = fs::File::open(archive_path)
        .into_diagnostic()
        .wrap_err("open archive file")?;
    let reader = io::BufReader::new(file);
    zip::ZipArchive::new(reader)
        .into_diagnostic()
        .wrap_err("open zip archive")?
        .extract(&subdir)
        .into_diagnostic()
        .wrap_err("extract sdk")?;
    Ok(())
}

/// Download the ASIO SDK if it's not available.
pub fn setup_sdk() -> miette::Result<()> {
    match sdk_cached().wrap_err("check cached asio")? {
        SdkCached::Yes => {
            println!("cargo:warning=ASIO SDK seems available, skipping download");
        }
        state @ SdkCached::NotDownloaded | state @ SdkCached::NotExtracted => {
            if state == SdkCached::NotDownloaded {
                download_sdk_archive().wrap_err("download asio")?;
            } else {
                println!("cargo:warning=Using previously downloaded ASIO SDK...");
            }
            println!("cargo:warning=Extracting ASIO SDK...");
            extract_sdk_archive().wrap_err("extract asio")?;
        }
    }
    Ok(())
}
