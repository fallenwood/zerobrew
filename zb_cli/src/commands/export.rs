use std::path::Path;

use chrono::Local;
use console::style;

use super::lockfile::{self, Metadata, PackageEntry, PackageManifest};
use crate::ui::StdUi;

pub fn execute(
    installer: &mut zb_io::Installer,
    file: &Path,
    force: bool,
    ui: &mut StdUi,
) -> Result<(), zb_core::Error> {
    if file.exists() && !force {
        return Err(zb_core::Error::FileError {
            message: format!(
                "file {} already exists (use --force to overwrite)",
                file.display()
            ),
        });
    }

    let installed = installer.list_installed()?;

    if installed.is_empty() {
        ui.heading("No packages installed to export.")
            .map_err(ui_error)?;
        return Ok(());
    }

    let mut formulas = Vec::new();
    let mut casks = Vec::new();

    for keg in &installed {
        if let Some(token) = keg.name.strip_prefix("cask:") {
            casks.push(PackageEntry {
                name: token.to_string(),
                version: keg.version.clone(),
                build_from_source: false,
            });
        } else {
            formulas.push(PackageEntry {
                name: keg.name.clone(),
                version: keg.version.clone(),
                build_from_source: keg.store_key.starts_with("source:"),
            });
        }
    }

    formulas.sort_by(|a, b| a.name.cmp(&b.name));
    casks.sort_by(|a, b| a.name.cmp(&b.name));

    let manifest = PackageManifest {
        metadata: Metadata {
            schema_version: lockfile::SCHEMA_VERSION,
            exported_at: Local::now().to_rfc3339(),
            zerobrew_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        formulas,
        casks,
    };

    let content = toml::to_string_pretty(&manifest).map_err(|e| zb_core::Error::FileError {
        message: format!("failed to serialize manifest: {e}"),
    })?;

    std::fs::write(file, content).map_err(|e| zb_core::Error::FileError {
        message: format!("failed to write {}: {e}", file.display()),
    })?;

    let total = manifest.formulas.len() + manifest.casks.len();
    ui.heading(format!(
        "Exported {} {} to {}",
        style(total).green().bold(),
        if total == 1 { "package" } else { "packages" },
        file.display()
    ))
    .map_err(ui_error)?;

    Ok(())
}

fn ui_error(err: std::io::Error) -> zb_core::Error {
    zb_core::Error::FileError {
        message: format!("failed to write CLI output: {err}"),
    }
}
