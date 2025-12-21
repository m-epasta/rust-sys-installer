

use rust_sys_installer::cli::executor::execute_ubuntu;

fn main() {
    if let Err(e) = execute_ubuntu() {
        eprintln!("Installation failed: {}", e);
        std::process::exit(1);
    }

    println!("Development environment setup complete!");
}
