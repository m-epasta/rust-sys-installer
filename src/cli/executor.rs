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

    // install git, curl, npm
    CommandBuilder::apt_install("curl").execute()?;
    CommandBuilder::apt_install("git").execute()?;
    CommandBuilder::apt_install("npm").execute()?;

    // Install node in the reccomended way with nvm
    install_nvm()?;
    install_nodejs_with_nvm()?;

    // install rust, curl rust then load cargo to PATH
    install_rust()?;

    // Now, installs vscode
    

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
