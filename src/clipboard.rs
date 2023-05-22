use std::slice;
use std::char::REPLACEMENT_CHARACTER;
use std::fmt::{Display, Formatter, Write};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::{FALSE, HANDLE};
use windows_sys::Win32::System::DataExchange::{CloseClipboard, GetClipboardData, OpenClipboard};
use windows_sys::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows_sys::Win32::System::Ole::CF_UNICODETEXT;
use crate::ensure;
use crate::error::{WinError, WinResult};

#[derive(Debug)]
pub struct Clipboard;

impl Clipboard {
    pub fn new() -> WinResult<Self> {
        unsafe {
            ensure!(OpenClipboard(0) != FALSE, WinError::last_error());
            Ok(Self)
        }
    }

    pub fn get_text(&self) -> WinResult<U16String> {
        unsafe {
            let sys_str = SystemString::new(GetClipboardData(CF_UNICODETEXT as _))?;
            let usr_str = U16String::from(&sys_str);
            Ok(usr_str)
        }
    }

}

impl Drop for Clipboard {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
        }
    }
}

struct SystemString {
    handle: HANDLE,
    ptr: PCWSTR
}

impl SystemString {
    unsafe fn new(handle: HANDLE) -> WinResult<Self> {
        ensure!(handle != 0, WinError::last_error());
        let ptr = GlobalLock(handle) as PCWSTR;
        ensure!(!ptr.is_null(), WinError::last_error());
        Ok(Self {
            handle,
            ptr,
        })
    }
    unsafe fn len(&self) -> usize {
        let mut ptr = self.ptr;
        let mut length = 0;
        while *ptr != 0 {
            length += 1;
            ptr = ptr.offset(1);
        }
        length
    }
    fn as_slice(&self) -> &[u16] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len())
        }
    }
}

impl Drop for SystemString {
    fn drop(&mut self) {
        unsafe{ GlobalUnlock(self.handle); }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct U16String(Vec<u16>);

impl From<&SystemString> for U16String {
    fn from(value: &SystemString) -> Self {
        Self(Vec::from(value.as_slice()))
    }
}

impl Display for U16String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in char::decode_utf16(self.0.iter().copied()).map(|r| r.unwrap_or(REPLACEMENT_CHARACTER)) {
            f.write_char(c)?;
        }
        Ok(())
    }
}