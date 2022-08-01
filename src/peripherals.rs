use std::{fmt, string};

use cfg_if::cfg_if;
use defaults::Defaults;
use parse_display_derive::Display;
use serde_json::json;

cfg_if! {
    if #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))] {
        compile_error!("Keyboard.rs only supports Mac-, Linux- and Windows-OS.");
    }
}

#[derive(Debug, Display, Defaults)]
#[def = "Unknown"]
pub enum ErrorKind {
    Unknown,
    OSSpecific,
    OSAgnostic,
}

#[derive(Debug, Display)]
pub struct ErrorCode(i32);

#[derive(Debug, Display, Defaults)]
#[display("{kind}Error: {description} (ErrorCode {code})")]
pub struct Error {
    #[def = "ErrorKind::default()"]
    pub kind: ErrorKind,
    #[def = "ErrorCode::UNKNOWN"]
    pub code: ErrorCode,
    #[def = "\"No description can be found.\".to_string()"]
    pub description: String,
}

impl ErrorCode {
    pub const UNKNOWN: ErrorCode = ErrorCode(1);
    pub const NO_EQUIVALENT_KEY: ErrorCode = ErrorCode(2);
    pub const INVALID_MODIFIRES_LIST: ErrorCode = ErrorCode(3);
}

impl From<i32> for ErrorCode {
    fn from(error_code: i32) -> Self {
        ErrorCode(error_code)
    }
}

impl Error {
    pub fn new(kind: ErrorKind, code: ErrorCode, description: impl Into<String>) -> Self {
        Self {
            kind,
            code,
            description: description.into(),
        }
    }

    pub fn to_json(&self) -> String {
        json!({
            "kind": self.kind.to_string(),
            "code": self.code.0,
            "description": self.description
        })
        .to_string()
    }
}
