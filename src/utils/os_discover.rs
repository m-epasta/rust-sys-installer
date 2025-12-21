use crate::prelude::ProcessError;

pub fn fetch_os() -> Result<String, ProcessError> {
    let info: os_info::Info = os_info::get();
    let version_str: String = info.version().to_string();

    if version_str.contains("Ubuntu") {
        Ok("Ubuntu".to_string())
    } else {
        Ok(info.os_type().to_string())
    }
}
