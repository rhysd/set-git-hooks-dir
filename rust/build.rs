#[cfg(feature = "setup-on-build")]
#[path = "src/installer.rs"]
#[allow(dead_code)]
mod installer;

fn main() {
    #[cfg(all(feature = "setup-on-build", debug_assertions))]
    if let Err(err) = installer::setup(".git-hooks", std::env::var_os("OUT_DIR").unwrap()) {
        println!("cargo::warning=Git hooks directory was not set: {}", err);
    }
}
