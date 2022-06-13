use serde::Deserialize;
use serde::Serialize;
use std::boxed::Box;

#[derive(Serialize, Deserialize)]
pub struct Result<T> {
    pub code: i32,
    pub msg: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub struct Error {
    pub code: i32,
    pub message: &'static str,
}

impl Result<()> {
    // 0 操作成功
    pub const SUCCESS: Error = Error {
        code: 0,
        message: "Success",
    };

    pub const SYS_ERROR: Error = Error {
        code: 10001,
        message: "Error",
    };

    pub fn success() -> Self {
        let e = Result::SUCCESS;
        Result {
            code: e.code,
            msg: e.message,
            data: None,
        }
    }

    pub fn error(e: Error) -> Self {
        Result {
            code: e.code,
            msg: e.message,
            data: None,
        }
    }

    pub fn error_description(e: Error, msg: &str) -> Self {
        let msg = format!("{} {}", e.message, msg);
        Result {
            code: e.code,
            msg: Box::leak(msg.into_boxed_str()),
            data: None,
        }
    }
}

impl<T> Result<T> {
    pub fn success_return_data(data: T) -> Self {
        let e = Result::SUCCESS;
        Result {
            code: e.code,
            msg: e.message,
            data: Some(data),
        }
    }
}
