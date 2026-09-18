use std::path::PathBuf;

fn main() {
    // libmp3lame is linked dynamically (LGPL, see src/lame.rs): scripts/baue-lame.sh puts it
    // into src-tauri/frameworks, the bundler copies it to Contents/Frameworks.
    let frameworks = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("frameworks");
    if !frameworks.join("libmp3lame.dylib").exists() {
        panic!("libmp3lame.dylib fehlt: zuerst scripts/baue-lame.sh ausführen");
    }
    println!("cargo:rustc-link-search=native={}", frameworks.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Frameworks");
    // Outside a bundle (cargo test, tauri dev) the library is found next to the sources. Release
    // builds set PA_RELEASE=1 so that no path of this machine ends up in a shipped binary.
    println!("cargo:rerun-if-env-changed=PA_RELEASE");
    if std::env::var_os("PA_RELEASE").is_none() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", frameworks.display());
    }
    tauri_build::build()
}
