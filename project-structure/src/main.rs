mod cargo_and_project_structure;
mod cargo_toml;
mod crates_packages_modules;

fn main() {
    println!("--- CARGO & PROJECT STRUCTURE DEMO ---");
    cargo_and_project_structure::cargo_and_project_structure_demo();

    println!("\n--- CARGO.TOML & TOML DEMO ---");
    cargo_toml::cargo_toml_demo();

    println!("\n--- CRATES, PACKAGES, MODULES DEMO ---");
    crates_packages_modules::crates_packages_modules_demo();
}
