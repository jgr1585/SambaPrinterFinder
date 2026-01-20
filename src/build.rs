fn main() {
    pkg_config::Config::new()
        .probe("smbclient")
        .expect("Could not find libsmbclient via pkg-config");

    pkg_config::Config::new()
        .probe("cups")
        .expect("Could not find libcups via pkg-config");
}
