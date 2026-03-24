//! Archive extraction and discovery of the packaged app root directory.

use flate2::read::GzDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;
use zip::ZipArchive;

use crate::error::LugosError;

fn is_zip(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("zip"))
        .unwrap_or(false)
}

/// Extracts `archive_path` into `dest` (directory), supporting `.zip` and `.tar.gz`/`.tgz`.
pub fn extract_to_tempdir(archive_path: &Path, dest: &Path) -> Result<(), LugosError> {
    std::fs::create_dir_all(dest)?;
    if is_zip(archive_path) {
        extract_zip(archive_path, dest)?;
    } else {
        extract_tar_gz(archive_path, dest)?;
    }
    Ok(())
}

fn extract_tar_gz(src: &Path, dest: &Path) -> Result<(), LugosError> {
    let file = File::open(src)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    archive.unpack(dest)?;
    Ok(())
}

fn extract_zip(src: &Path, dest: &Path) -> Result<(), LugosError> {
    let file = File::open(src)?;
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let outpath = match entry.enclosed_name() {
            Some(p) => dest.join(p),
            None => continue,
        };
        if entry.is_dir() {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out = File::create(&outpath)?;
            std::io::copy(&mut entry, &mut out)?;
        }
    }
    Ok(())
}

pub fn find_app_root(extracted: &Path) -> Result<PathBuf, LugosError> {
    if extracted.join("app.manifest.json").exists() {
        return Ok(extracted.to_path_buf());
    }
    find_manifest_recursive(extracted)
}

fn find_manifest_recursive(dir: &Path) -> Result<PathBuf, LugosError> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.join("app.manifest.json").exists() {
                return Ok(path);
            }
            if let Ok(found) = find_manifest_recursive(&path) {
                return Ok(found);
            }
        }
    }
    Err(LugosError::Msg(
        "app.manifest.json not found inside release archive".into(),
    ))
}

/// Extract archive to a temporary layout, locate app root, copy tree to `dest`.
pub fn extract_app(archive_path: &Path, dest: &Path) -> Result<(), LugosError> {
    let tmp = archive_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!("extract-{}", chrono::Utc::now().timestamp_millis()));
    extract_to_tempdir(archive_path, &tmp)?;
    let root = find_app_root(&tmp)?;
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }
    std::fs::create_dir_all(dest.parent().unwrap_or(Path::new(".")))?;
    copy_dir_all(&root, dest)?;
    let _ = std::fs::remove_dir_all(&tmp);
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), LugosError> {
    std::fs::create_dir_all(dst)?;
    for e in std::fs::read_dir(src)? {
        let e = e?;
        let from = e.path();
        let to = dst.join(e.file_name());
        let ft = e.file_type()?;
        if ft.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}
