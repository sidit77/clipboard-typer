use windows_sys::Win32::Foundation::{GetLastError, WIN32_ERROR};

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $result:expr) => {
        if !($cond) {
            return Err($result);
        }
    };
}

pub type WinResult<T> = Result<T, WinError>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WinError {
    code: WIN32_ERROR
}

impl WinError {
    pub fn last_error() -> Self {
        unsafe {
            let code = GetLastError();
            Self {
                code,
            }
        }
    }
}