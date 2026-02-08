fn main() {
    // smbclient must come from pkg-config
    pkg_config::Config::new()
        .probe("smbclient")
        .expect("Could not find libsmbclient via pkg-config");

    // CUPS: pkg-config on Linux, system lib on macOS
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        println!("cargo:rustc-link-lib=cups");
    } else {
        pkg_config::Config::new()
            .probe("cups")
            .expect("Could not find libcups via pkg-config");
    }
}
