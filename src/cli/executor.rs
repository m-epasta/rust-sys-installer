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
    CommandBuilder::apt_install("curl");
    CommandBuilder::apt_install("git");
    CommandBuilder::apt_install("npm");

    // Install node and rust
    

    Ok(())
}

