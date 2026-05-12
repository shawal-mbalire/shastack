use anyhow::Result;
use colored::*;
use inquire::Confirm;
use self_update::cargo_crate_version;

pub fn check_for_updates() -> Result<()> {
    // Skip update check if explicitly disabled or in CI
    if std::env::var("SHA_SKIP_UPDATE").is_ok() || std::env::var("CI").is_ok() {
        return Ok(());
    }

    let current_version = cargo_crate_version!();
    
    // We use self_update to check against GitHub releases
    let status = self_update::backends::github::Update::configure()
        .repo_owner("shawal-mbalire")
        .repo_name("shastack")
        .bin_name("sha")
        .show_download_progress(true)
        .current_version(current_version)
        .no_confirm(true) // We will handle confirmation with inquire
        .build()?;

    let latest_release = status.get_latest_release()?;
    
    if self_update::version::bump_is_greater(current_version, &latest_release.version)? {
        println!(
            "{}",
            format!(
                "A new version of sha is available: {} -> {}",
                current_version.yellow(),
                latest_release.version.green()
            )
            .bold()
        );

        let ans = Confirm::new("Would you like to upgrade now?")
            .with_default(true)
            .prompt()?;

        if ans {
            println!("{}", "Upgrading sha...".cyan());
            status.update()?;
            println!("{}", "Successfully upgraded to latest version!".green());
            std::process::exit(0);
        }
    }

    Ok(())
}
