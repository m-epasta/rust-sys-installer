use crate::{prelude::*, utils::cmd_builder::CommandBuilder};

pub fn execute_ubuntu() -> Result<(), ProcessError> {
    // Should only proceed if the os is ubuntu >= 22.04
    let os: String = os_discover::fetch_os()?;

    if os != "Ubuntu" {
        return Err(ProcessError::ScriptingError {
            error_msg: format!("This function requires Ubuntu, but detected: {}", os),
            exit_code: 1,
        });
    }

    // install basic system packages and vscode prerequisites
    CommandBuilder::apt_install("curl").execute()?;
    CommandBuilder::apt_install("git").execute()?;
    CommandBuilder::apt_install("npm").execute()?;
    CommandBuilder::apt_install("wget").execute()?;
    CommandBuilder::apt_install("gpg").execute()?;
    CommandBuilder::apt_install("apt-transport-https").execute()?;

    // Install node in the reccomended way with nvm
    install_nvm()?;
    install_nodejs_with_nvm()?;

    // install rust, curl rust then load cargo to PATH
    install_rust()?;

    // Install vscode
    install_vscode()?;
    configure_vscode()?;

    Ok(())
}

pub fn install_nvm() -> Result<(), ProcessError> {
    let command = "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash";

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(command)
        .execute()?;

    Ok(())
}

pub fn install_nodejs_with_nvm() -> Result<(), ProcessError> {
    let nvm_commands = r#"
        . "$HOME/.nvm/nvm.sh"
        nvm install 24
    "#;

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(nvm_commands)
        .execute()?;

    Ok(())
}

pub fn install_rust() -> Result<(), ProcessError> {
    let command = "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y";

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(command)
        .execute()?;

    Ok(())
}

pub fn install_vscode() -> Result<(), ProcessError> {
    let setup_commands = r#"
        wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
        sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt/keyrings/packages.microsoft.gpg
        echo "deb [arch=amd64,arm64,armhf signed-by=/etc/apt/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/code stable main" | sudo tee /etc/apt/sources.list.d/vscode.list > /dev/null
        rm packages.microsoft.gpg
    "#;

    CommandBuilder::new("bash")
        .arg("-c")
        .arg(setup_commands)
        .execute()?;

    CommandBuilder::apt_install("code").execute()?;

    Ok(())
}

pub fn configure_vscode() -> Result<(), ProcessError> {
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

    // Install each extension
    for ext in recommendations {
        if let Some(ext_id) = ext.as_str() {
            CommandBuilder::new("code")
                .arg("--install-extension")
                .arg(ext_id)
                .execute()?;
        }
    }

    // Copy settings to user config directory
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

    Ok(())
}
