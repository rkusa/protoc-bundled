use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=PROTOC");
    println!("cargo:rerun-if-env-changed=PROTOC_INCLUDE");
    println!(
        "cargo:rustc-env=PROTOC={}",
        env::current_dir()
            .unwrap()
            .join("protoc")
            .join("bin")
            .join(exectuable())
            .display()
    );
    println!(
        "cargo:rustc-env=PROTOC_INCLUDE={}",
        env::current_dir()
            .unwrap()
            .join("protoc")
            .join("include")
            .display()
    );
}

fn exectuable() -> &'static str {
    match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => "protoc-3.21.6-linux-x86_64",
        ("linux", "aarch64") => "protoc-3.21.6-linux-aarch_64",
        ("macos", "x86_64") => "protoc-3.21.6-osx-x86_64",
        ("macos", "aarch64") => "protoc-3.21.6-osx-aarch_64",
        ("windows", "x86_64") => "protoc-3.21.6-win64.exe",
        (os, arch) => panic!(
            "protoc-bundled does not include executable for {}@{}",
            os, arch
        ),
    }
}
