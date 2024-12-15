fn main() {
    #[cfg(feature = "app_internal")]
    println!("cargo::rerun-if-changed=../app");
    #[cfg(feature = "app_internal")]
    assert!(
        std::process::Command::new("pnpm")
            .args(&["--filter", "itty.pro", "run", "build"])
            .current_dir("../app")
            .output()
            .expect("failed to build the app")
            .status
            .success(),
        "pnpm --filter itty.pro run build"
    )
}
