use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Scripting error: {error_msg}.\n Exited with {exit_code}")]
    ScriptingError { error_msg: String, exit_code: u16 },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}
