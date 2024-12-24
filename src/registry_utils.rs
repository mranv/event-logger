use windows::Win32::System::Registry::{
    HKEY, RegOpenKeyExW, RegCloseKey, RegCreateKeyExW, RegDeleteKeyW,
    KEY_READ, KEY_WRITE, REG_OPTION_NON_VOLATILE,
};
use windows::Win32::Foundation::{ERROR_SUCCESS, WIN32_ERROR};
use windows::core::{Error, HSTRING, HRESULT};
use windows::core::PCWSTR;
use std::iter;

/// HKLM handle is 0x80000002
const HKEY_LOCAL_MACHINE: HKEY = HKEY(0x80000002u64 as isize);

fn build_source_registry_path(log_name: &str, source_name: &str) -> String {
    format!("SYSTEM\\CurrentControlSet\\Services\\EventLog\\{log_name}\\{source_name}")
}

/// Check if the given event source subkey exists
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

/// Create the registry subkey for <log_name>\<source_name>, e.g. Infopercept\IvsAgent
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

/// Delete the registry subkey for <log_name>\<source_name>
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

/// Helper to convert a Win32 error code to a `windows::core::Error`
fn win32_error(context: &str, code: WIN32_ERROR) -> Error {
    // Combine code with 0x80070000 to build an HRESULT for FACILITY_WIN32
    let mask = 0x80070000u32 | (code.0 & 0xFFFF);
    let hr = HRESULT(mask as i32);
    Error::new(hr, HSTRING::from(format!("{context} failed with code: {}", code.0)))
}
