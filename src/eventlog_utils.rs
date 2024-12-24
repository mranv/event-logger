use windows::Win32::System::EventLog::{
    RegisterEventSourceW,
    DeregisterEventSource,
    ReportEventW,
    EventSourceHandle,
    REPORT_EVENT_TYPE,
};
use windows::core::PCWSTR;
use std::iter; // for encode_utf16

/// If you ever need other event-type constants (Error=1, Info=4), just define and use them:
/// pub const EVENT_TYPE_ERROR: u16 = 1;
pub const EVENT_TYPE_WARNING: u16 = 2;
/// pub const EVENT_TYPE_INFORMATION: u16 = 4;

pub struct EventLogger {
    source_handle: EventSourceHandle,
}

impl EventLogger {
    /// Register a source under a given log, e.g. HKLM\SYSTEM\CurrentControlSet\Services\EventLog\<log>\<source>
    pub fn register(source: &str, log: &str) -> Option<Self> {
        let registry_path = format!("SYSTEM\\CurrentControlSet\\Services\\EventLog\\{log}\\{source}");
        let wide_path: Vec<u16> = registry_path.encode_utf16().chain(iter::once(0)).collect();

        match unsafe { RegisterEventSourceW(None, PCWSTR(wide_path.as_ptr())) } {
            Ok(handle) => Some(Self { source_handle: handle }),
            Err(_) => None,
        }
    }

    /// Write a single-string event to the Windows Event Log.
    ///
    /// - `message`: the text to log
    /// - `event_type`: typically 1=Error, 2=Warning, 4=Information (here default is 2=Warning)
    pub fn write_entry(&self, message: &str, event_type: u16) {
        let wide_msg: Vec<u16> = message.encode_utf16().chain(iter::once(0)).collect();
        let pcwstr_msg = PCWSTR(wide_msg.as_ptr());
        let string_slice = [pcwstr_msg];

        let report_type = REPORT_EVENT_TYPE(event_type);

        unsafe {
            // 8-parameter version of ReportEventW on your system:
            // (heventlog, wtype, wcategory, dweventid, lpusersid, dwdatasize, lpstrings, lprawdata)
            let _ok = ReportEventW(
                self.source_handle.clone(),
                report_type,
                0,    // category
                0,    // event ID
                None, // user SID
                0,    // data size
                Some(&string_slice),
                None,
            );
        }
    }
}

impl Drop for EventLogger {
    fn drop(&mut self) {
        unsafe {
            let _ = DeregisterEventSource(self.source_handle.clone());
        }
    }
}
