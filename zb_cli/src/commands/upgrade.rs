use console::style;

use crate::ui::StdUi;

pub async fn execute(
    installer: &mut zb_io::Installer,
    formulas: Vec<String>,
    all: bool,
    ui: &mut StdUi,
) -> Result<(), zb_core::Error> {
    ui.heading("Checking for outdated packages...")
        .map_err(ui_error)?;

    let (outdated, warnings) = installer.check_outdated().await?;

    for warning in &warnings {
        eprintln!("{} {}", style("Warning:").yellow().bold(), warning);
    }

    let to_upgrade = if all {
        outdated
    } else {
        let mut selected = Vec::new();
        for name in &formulas {
            match outdated.iter().find(|pkg| pkg.name == *name) {
                Some(pkg) => selected.push(pkg.clone()),
                None => {
                    // Check if it's installed but already up to date
                    if installer.get_installed(name).is_some() {
                        ui.bullet(format!("{} is already up to date.", style(name).bold()))
                            .map_err(ui_error)?;
                    } else {
                        return Err(zb_core::Error::NotInstalled {
                            name: name.clone(),
                        });
                    }
                }
            }
        }
        selected
    };

    if to_upgrade.is_empty() {
        ui.heading("All packages are up to date.")
            .map_err(ui_error)?;
        return Ok(());
    }

    ui.heading(format!(
        "Upgrading {} {}:",
        style(to_upgrade.len()).bold(),
        if to_upgrade.len() == 1 {
            "package"
        } else {
            "packages"
        }
    ))
    .map_err(ui_error)?;

    for pkg in &to_upgrade {
        ui.bullet(format!(
            "{} {} {} {}",
            style(&pkg.name).green(),
            style(&pkg.installed_version).red(),
            style("→").dim(),
            style(&pkg.current_version).green(),
        ))
        .map_err(ui_error)?;
    }

    ui.blank_line().map_err(ui_error)?;

    let names: Vec<String> = to_upgrade.iter().map(|pkg| pkg.name.clone()).collect();
    let any_source = to_upgrade.iter().any(|pkg| pkg.is_source_build);

    super::install::execute(installer, names, false, true, any_source, ui).await
}

fn ui_error(err: std::io::Error) -> zb_core::Error {
    zb_core::Error::FileError {
        message: format!("failed to write CLI output: {err}"),
    }
}
