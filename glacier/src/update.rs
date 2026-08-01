use self_update::{Status, backends::gitea};

/// Updates the CLI in place.
pub fn update() -> anyhow::Result<()> {
    let status = gitea::Update::configure()
        .with_host("https://git.lab.itsnova.sh")
        .repo_owner("nova")
        .repo_name("glacier")
        .bin_name("glacier")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    match status {
        Status::UpToDate(_) => {
            println!(
                "\n\nglacier is already up to date (v{}).",
                env!("CARGO_PKG_VERSION")
            )
        }
        Status::Updated(version) => {
            println!("\nglacier updated to v{}.", version)
        }
    }

    Ok(())
}
