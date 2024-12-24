use windows::Win32::System::Registry::{
    HKEY, RegOpenKeyExW, RegCloseKey, RegCreateKeyExW, RegDeleteKeyW,
    KEY_READ, KEY_WRITE, REG_OPTION_NON_VOLATILE,
};
use windows::Win32::Foundation::{ERROR_SUCCESS, WIN32_ERROR};
use windows::core::{Error, HSTRING, HRESULT};
use windows::core::PCWSTR;
use std::iter;

const HKEY_LOCAL_MACHINE: HKEY = HKEY(0x80000002u64 as isize);

fn build_source_registry_path(log_name: &str, source_name: &str) -> String {
    format!("SYSTEM\\CurrentControlSet\\Services\\EventLog\\{log_name}\\{source_name}")
}

/// Check if an event source key exists
pub fn source_exists(log_name: &str, source_name: &str) -> bool {
    let path = build_source_registry_path(log_name, source_name);
    let wide_path: Vec<u16> = path.encode_utf16().chain(iter::once(0)).collect();

    let mut hkey_out = HKEY(0);
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(wide_path.as_ptr()),
            0,
            KEY_READ,
            &mut hkey_out,
        )
    };

    let exists = status == ERROR_SUCCESS;
    if exists {
        unsafe { RegCloseKey(hkey_out) };
    }

    exists
}

/// Create the registry key for the event source
pub fn create_event_source(log_name: &str, source_name: &str) -> Result<(), Error> {
    let path = build_source_registry_path(log_name, source_name);
    let wide_path: Vec<u16> = path.encode_utf16().chain(iter::once(0)).collect();

    let mut hkey_out = HKEY(0);
    let status: WIN32_ERROR = unsafe {
        RegCreateKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(wide_path.as_ptr()),
            0,
            None,
            REG_OPTION_NON_VOLATILE,
            KEY_WRITE,
            None,
            &mut hkey_out,
            None,
        )
    };

    if status == ERROR_SUCCESS {
        unsafe { RegCloseKey(hkey_out) };
        Ok(())
    } else {
        Err(win32_error("RegCreateKeyExW", status))
    }
}

/// Delete the registry key for the event source
pub fn delete_event_source(log_name: &str, source_name: &str) -> Result<(), Error> {
    let path = build_source_registry_path(log_name, source_name);
    let wide_path: Vec<u16> = path.encode_utf16().chain(iter::once(0)).collect();

    let status: WIN32_ERROR = unsafe {
        RegDeleteKeyW(HKEY_LOCAL_MACHINE, PCWSTR(wide_path.as_ptr()))
    };

    if status == ERROR_SUCCESS {
        Ok(())
    } else {
        Err(win32_error("RegDeleteKeyW", status))
    }
}

/// Convert a WIN32_ERROR to a windows::core::Error
fn win32_error(context: &str, code: WIN32_ERROR) -> Error {
    // Convert a Win32 error to an HRESULT (FACILITY_WIN32 = 7).
    // 0x8007_0000 is an i32 bitmask. We must cast to i32.
    let mask = 0x80070000u32 | (code.0 & 0xFFFF);
    let hr = HRESULT(mask as i32);
    Error::new(hr, HSTRING::from(format!("{context} failed with code: {}", code.0)))
}
