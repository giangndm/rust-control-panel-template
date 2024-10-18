use std::process::Command;

fn main() {
    // Call the function from the prisma_client_rust_cli crate
    prisma_client_rust_cli::run();

    // Build Vite project when compiling in release mode
    if !cfg!(debug_assertions) {
        Command::new("pnpm")
            .current_dir("./public/")
            .args(&["run", "build"])
            .status()
            .expect("Failed to build Vite project");

        // Optionally copy the dist folder to the static directory
        std::fs::rename("../vite-project/dist", "static").expect("Failed to move dist folder");
    }
}
