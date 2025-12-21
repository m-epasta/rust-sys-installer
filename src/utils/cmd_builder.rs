use crate::prelude::*;
use std::{collections::HashMap, path::PathBuf, process::Output, time::Duration};

pub struct CommandBuilder {
    program: String,
    args: Vec<String>,
    env_vars: Option<HashMap<String, String>>,
    working_dir: Option<PathBuf>,
    timeout: Option<Duration>,
}

impl CommandBuilder {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: Vec::new(),
            env_vars: None,
            working_dir: None,
            timeout: None,
        }
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args.extend(args);
        self
    }

    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env_vars
            .get_or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
        self
    }

    pub fn current_dir(mut self, dir: PathBuf) -> Self {
        self.working_dir = Some(dir);
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    pub fn execute(&self) -> Result<Output, ProcessError> {
        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args);

        if let Some(env_vars) = &self.env_vars {
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code().unwrap_or(1) as u16;
            return Err(ProcessError::ScriptingError {
                error_msg,
                exit_code,
            });
        }

        Ok(output)
    }

    // OS-specific helpers
    pub fn apt_install(package: &str) -> Self {
        Self::new("apt").args(vec![
            "install".to_string(),
            "-y".to_string(),
            package.to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_builder_new() {
        let cmd = CommandBuilder::new("ls");
        assert_eq!(cmd.program, "ls");
        assert!(cmd.args.is_empty());
        assert!(cmd.env_vars.is_none());
        assert!(cmd.working_dir.is_none());
        assert!(cmd.timeout.is_none());
    }

    #[test]
    fn test_command_builder_with_args() {
        let cmd = CommandBuilder::new("ls")
            .arg("-la")
            .args(vec!["/tmp".to_string(), "/var".to_string()]);

        assert_eq!(cmd.program, "ls");
        assert_eq!(cmd.args, vec!["-la", "/tmp", "/var"]);
    }

    #[test]
    fn test_command_builder_with_env() {
        let cmd = CommandBuilder::new("echo")
            .env("HOME", "/tmp")
            .env("USER", "test");

        assert_eq!(
            cmd.env_vars.as_ref().unwrap().get("HOME"),
            Some(&"/tmp".to_string())
        );
        assert_eq!(
            cmd.env_vars.as_ref().unwrap().get("USER"),
            Some(&"test".to_string())
        );
    }

    #[test]
    fn test_ubuntu_apt_install() {
        let cmd = CommandBuilder::apt_install("curl");
        assert_eq!(cmd.program, "apt");
        assert_eq!(cmd.args, vec!["install", "-y", "curl"]);
    }
}
