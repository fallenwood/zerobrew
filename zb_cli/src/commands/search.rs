use console::style;

pub async fn execute(installer: &mut zb_io::Installer, query: String) -> Result<(), zb_core::Error> {
    let (formulas, casks) = tokio::try_join!(
        installer.search_formulas(&query),
        installer.search_casks(&query),
    )?;

    if formulas.is_empty() && casks.is_empty() {
        println!("No packages found matching '{}'.", query);
        return Ok(());
    }

    if !formulas.is_empty() {
        println!(
            "{}",
            style(format!(
                "==> Formulae ({})",
                formulas.len()
            ))
            .bold()
        );
        for name in &formulas {
            println!("{}", name);
        }
    }

    if !casks.is_empty() {
        if !formulas.is_empty() {
            println!();
        }
        println!(
            "{}",
            style(format!(
                "==> Casks ({})",
                casks.len()
            ))
            .bold()
        );
        for name in &casks {
            println!("{}", name);
        }
    }

    Ok(())
}
