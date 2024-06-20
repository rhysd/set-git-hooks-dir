#[cfg(feature = "setup-on-build")]
#[path = "src/installer.rs"]
mod installer;

fn main() {
    #[cfg(all(feature = "setup-on-build", debug_assertions))]
    if let Err(err) = installer::setup(".git-hooks") {
        println!("cargo::warning=Git hooks directory was not set: {}", err);
    }
}
