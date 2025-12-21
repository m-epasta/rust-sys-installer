use crate::{prelude::*, utils::cmd_builder::CommandBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;

// Progress bar guard for error recovery
#[allow(dead_code)]
struct ProgressGuard<'a> {
    progress: &'a ProgressBar,
    initial_position: u64,
}

#[allow(dead_code)]
impl<'a> ProgressGuard<'a> {
    fn new(progress: &'a ProgressBar) -> Self {
        let initial_position = progress.position();
        Self {
            progress,
            initial_position,
        }
    }
}

impl<'a> Drop for ProgressGuard<'a> {
    fn drop(&mut self) {
        // Reset to initial position on error
        self.progress.set_position(self.initial_position);
    }
}

// Username validation to prevent command injection
fn validate_username(username: &str) -> Result<(), ProcessError> {
    // Only allow alphanumeric, underscore, and hyphen
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err(ProcessError::ScriptingError {
            error_msg: format!("Invalid username format: {}", username),
            exit_code: 1,
        });
    }

    // Check reasonable length
    if username.len() > 32 || username.is_empty() {
        return Err(ProcessError::ScriptingError {
            error_msg: format!("Username length invalid: {}", username),
            exit_code: 1,
        });
    }

    Ok(())
}

// Installation validation functions
fn validate_rust_installation(user: Option<&str>) -> Result<(), ProcessError> {
    let check_cmd = if let Some(original_user) = user {
        validate_username(original_user)?;
        format!("sudo -u {} -i bash -c 'cargo --version > /dev/null 2>&1'", original_user)
    } else {
        "cargo --version > /dev/null 2>&1".to_string()
    };

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(&check_cmd)
        .execute()
        .map(|_| ()) // Convert Result<Output, ProcessError> to Result<(), ProcessError>
        .map_err(|_| ProcessError::ScriptingError {
            error_msg: "Rust installation validation failed".to_string(),
            exit_code: 1,
        })
}

fn validate_nvm_installation(user: Option<&str>) -> Result<(), ProcessError> {
    let check_cmd = if let Some(original_user) = user {
        validate_username(original_user)?;
        format!("sudo -u {} -i bash -c '. ~/.nvm/nvm.sh && nvm --version > /dev/null 2>&1'", original_user)
    } else {
        ". ~/.nvm/nvm.sh && nvm --version > /dev/null 2>&1".to_string()
    };

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(&check_cmd)
        .execute()
        .map(|_| ()) // Convert Result<Output, ProcessError> to Result<(), ProcessError>
        .map_err(|_| ProcessError::ScriptingError {
            error_msg: "NVM installation validation failed".to_string(),
            exit_code: 1,
        })
}

pub fn execute_ubuntu() -> Result<(), ProcessError> {
    // Should only proceed if the os is ubuntu >= 22.04
    let os: String = os_discover::fetch_os()?;

    if os != "Ubuntu" {
        return Err(ProcessError::ScriptingError {
            error_msg: format!("This function requires Ubuntu, but detected: {}", os),
            exit_code: 1,
        });
    }

    // Create progress bar for installation steps
    let pb = ProgressBar::new(6);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb.set_message("Installing system packages...");
    // install basic system packages and vscode prerequisites
    CommandBuilder::apt_install("curl").execute()?;
    CommandBuilder::apt_install("git").execute()?;
    CommandBuilder::apt_install("npm").execute()?;
    CommandBuilder::apt_install("wget").execute()?;
    CommandBuilder::apt_install("gpg").execute()?;
    CommandBuilder::apt_install("apt-transport-https").execute()?;
    pb.inc(1);

    pb.set_message("Installing NVM...");
    // Install node in the recommended way with nvm
    install_nvm()?;
    validate_nvm_installation(env::var("SUDO_USER").ok().as_deref())?;
    pb.inc(1);

    pb.set_message("Installing Node.js with NVM...");
    install_nodejs_with_nvm()?;
    pb.inc(1);

    pb.set_message("Installing Rust...");
    // install rust, curl rust then load cargo to PATH
    install_rust()?;
    validate_rust_installation(env::var("SUDO_USER").ok().as_deref())?;
    pb.inc(1);

    pb.set_message("Installing VS Code...");
    // Install vscode
    install_vscode()?;
    pb.inc(1);

    pb.set_message("Configuring VS Code...");
    configure_vscode()?;
    pb.inc(1);

    pb.finish_with_message("Development environment setup complete!");

    Ok(())
}

pub fn install_nvm() -> Result<(), ProcessError> {
    // Check if running as sudo and get original user
    let sudo_user = env::var("SUDO_USER").ok();

    if let Some(original_user) = &sudo_user {
        validate_username(original_user)?;
        // Install NVM as the original user so it's available for VS Code extensions
        let install_cmd = format!("sudo -u {} -i bash -c 'curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash'", original_user);
        CommandBuilder::new("bash")
            .arg("-c")
            .arg(&install_cmd)
            .execute()?;
    } else {
        // Not running as sudo, install normally
        let command = "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash";
        CommandBuilder::new("bash")
            .arg("-c")
            .arg(command)
            .execute()?;
    }

    Ok(())
}

pub fn install_nodejs_with_nvm() -> Result<(), ProcessError> {
    // Check if running as sudo and get original user
    let sudo_user = env::var("SUDO_USER").ok();

    if let Some(original_user) = &sudo_user {
        validate_username(original_user)?;
        // Install Node.js using NVM as the original user
        let node_cmd = format!("sudo -u {} -i bash -c '. \"$HOME/.nvm/nvm.sh\" && nvm install 24'", original_user);
        CommandBuilder::new("bash")
            .arg("-c")
            .arg(&node_cmd)
            .execute()?;
    } else {
        // Not running as sudo, install normally
        let nvm_commands = r#"
            . "$HOME/.nvm/nvm.sh"
            nvm install 24
        "#;

        CommandBuilder::new("bash")
            .arg("-c")
            .arg(nvm_commands)
            .execute()?;
    }

    Ok(())
}

pub fn install_rust() -> Result<(), ProcessError> {
    // Check if running as sudo and get original user
    let sudo_user = env::var("SUDO_USER").ok();

    if let Some(original_user) = &sudo_user {
        validate_username(original_user)?;
        // Install Rust as the original user so it's available for VS Code configuration
        let install_cmd = format!("sudo -u {} -i bash -c 'curl --proto \"=https\" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'", original_user);
        CommandBuilder::new("bash")
            .arg("-c")
            .arg(&install_cmd)
            .execute()?;
    } else {
        // Not running as sudo, install normally
        let command = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";
        CommandBuilder::new("bash")
            .arg("-c")
            .arg(command)
            .execute()?;
    }

    Ok(())
}

pub fn install_vscode() -> Result<(), ProcessError> {
    let setup_commands = r#"
        sudo snap install code --classic
    "#;

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(setup_commands)
        .execute()?;


    Ok(())
}

pub fn configure_vscode() -> Result<(), ProcessError> {
    // Check if running as sudo and get original user
    let sudo_user = env::var("SUDO_USER").ok();

    // Embed the configuration files
    let extensions_json = include_str!("../../vscode-config/.vscode/extensions.json");
    let settings_json = include_str!("../../vscode-config/.vscode/settings.json");

    // Parse extensions
    let extensions: serde_json::Value = serde_json::from_str(extensions_json)?;
    let recommendations = extensions["recommendations"]
        .as_array()
        .ok_or_else(|| ProcessError::ScriptingError {
            error_msg: "Invalid extensions.json format".to_string(),
            exit_code: 1,
        })?;

    let ext_count = recommendations.len();

    // Create progress bar for extensions
    let ext_pb = ProgressBar::new(ext_count as u64);
    ext_pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.blue} [{elapsed_precise}] [{bar:30.cyan/blue}] {pos:>2}/{len:2} Installing {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    // If running as sudo, run VS Code commands as the original user
    if let Some(original_user) = &sudo_user {
        println!("Running VS Code configuration as user: {}", original_user);

        // Install each extension as the original user
        for ext in recommendations {
            if let Some(ext_id) = ext.as_str() {
                ext_pb.set_message(format!("{}...", ext_id));
                let install_cmd = format!("sudo -u {} -i code --install-extension {}", original_user, ext_id);
                CommandBuilder::new("bash")
                    .arg("-c")
                    .arg(&install_cmd)
                    .execute()?;
                ext_pb.inc(1);
            }
        }

        ext_pb.finish_with_message("Extensions installed");

        // Copy settings to user config directory as the original user
        println!("Configuring VS Code settings...");
        let settings_commands = format!(r#"
            sudo -u {} -i bash -c "
                mkdir -p ~/.config/Code/User
                cat > ~/.config/Code/User/settings.json << 'EOF'
{}
EOF
            "
        "#, original_user, settings_json);

        CommandBuilder::new("bash")
            .arg("-c")
            .arg(&settings_commands)
            .execute()?;
    } else {
        // Not running as sudo, run commands normally
        println!("Running VS Code configuration as current user");

        // Install each extension
        for ext in recommendations {
            if let Some(ext_id) = ext.as_str() {
                ext_pb.set_message(format!("{}...", ext_id));
                CommandBuilder::new("code")
                    .arg("--install-extension")
                    .arg(ext_id)
                    .execute()?;
                ext_pb.inc(1);
            }
        }

        ext_pb.finish_with_message("Extensions installed");

        // Copy settings to user config directory
        println!("Configuring VS Code settings...");
        let settings_commands = format!(r#"
            mkdir -p ~/.config/Code/User
            cat > ~/.config/Code/User/settings.json << 'EOF'
{}
EOF
        "#, settings_json);

        CommandBuilder::new("bash")
            .arg("-c")
            .arg(&settings_commands)
            .execute()?;
    }

    Ok(())
}
