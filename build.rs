fn main() {
    // Try pkg-config first for better library detection
    if pkg_config::probe_library("rados").is_err() {
        println!("cargo:rustc-link-lib=dylib=rados");
    }

    if std::env::var("CARGO_FEATURE_RADOS_STRIPER").is_ok()
        && pkg_config::probe_library("radosstriper").is_err()
    {
        println!("cargo:rustc-link-lib=dylib=radosstriper");
    }
}
