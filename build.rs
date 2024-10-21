use std::process::Command;

fn main() {
    // Build Vite project when compiling in release mode
    if !cfg!(debug_assertions) {
        Command::new("pnpm")
            .current_dir("./admin-panel/")
            .args(&["run", "build"])
            .status()
            .expect("Failed to build Vite project");
    }
}
