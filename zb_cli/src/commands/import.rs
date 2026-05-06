use std::path::Path;

use console::style;

use super::lockfile::PackageManifest;
use crate::ui::StdUi;

pub async fn execute(
    installer: &mut zb_io::Installer,
    file: &Path,
    ui: &mut StdUi,
) -> Result<(), zb_core::Error> {
    let content = std::fs::read_to_string(file).map_err(|e| zb_core::Error::FileError {
        message: format!("failed to read {}: {e}", file.display()),
    })?;

    let manifest: PackageManifest =
        toml::from_str(&content).map_err(|e| zb_core::Error::FileError {
            message: format!("failed to parse {}: {e}", file.display()),
        })?;

    let mut to_install: Vec<String> = Vec::new();
    let mut source_builds: Vec<String> = Vec::new();
    let mut skipped = 0usize;

    for entry in &manifest.formulas {
        if installer.get_installed(&entry.name).is_some() {
            skipped += 1;
            continue;
        }
        if entry.build_from_source {
            source_builds.push(entry.name.clone());
        } else {
            to_install.push(entry.name.clone());
        }
    }

    for entry in &manifest.casks {
        let install_name = format!("cask:{}", entry.name);
        if installer.get_installed(&install_name).is_some() {
            skipped += 1;
            continue;
        }
        to_install.push(install_name);
    }

    let total = to_install.len() + source_builds.len();

    if total == 0 {
        ui.heading(format!(
            "All {} packages from {} are already installed.",
            manifest.formulas.len() + manifest.casks.len(),
            file.display()
        ))
        .map_err(ui_error)?;
        return Ok(());
    }

    ui.heading(format!(
        "Installing {} {} from {} ({} already installed)",
        style(total).green().bold(),
        if total == 1 { "package" } else { "packages" },
        file.display(),
        skipped,
    ))
    .map_err(ui_error)?;

    if !to_install.is_empty() {
        super::install::execute(installer, to_install, false, false, false, ui).await?;
    }

    if !source_builds.is_empty() {
        super::install::execute(installer, source_builds, false, false, true, ui).await?;
    }

    Ok(())
}

fn ui_error(err: std::io::Error) -> zb_core::Error {
    zb_core::Error::FileError {
        message: format!("failed to write CLI output: {err}"),
    }
}
