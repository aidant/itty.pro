fn main() {
    #[cfg(feature = "include_app")]
    println!("cargo::rerun-if-changed=../app");
    #[cfg(feature = "include_app")]
    assert!(
        std::process::Command::new("pnpm")
            .args(&["--filter", "url", "run", "build"])
            .current_dir("../app")
            .output()
            .expect("failed to build the app")
            .status
            .success(),
        "pnpm --filter url run build"
    )
}
