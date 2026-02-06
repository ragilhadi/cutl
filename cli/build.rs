use std::fs;
use std::path::Path;

fn main() {
    // Read version from version/version file
    let version_file = Path::new("../version/version");

    if version_file.exists() {
        let version = fs::read_to_string(version_file)
            .expect("Failed to read version file")
            .trim()
            .trim_start_matches('v') // Remove 'v' prefix if present
            .to_string();

        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", version);

        // Re-run build script if version file changes
        println!("cargo:rerun-if-changed=../version/version");
    } else {
        println!("cargo:warning=version/version file not found, using default version");
    }
}
