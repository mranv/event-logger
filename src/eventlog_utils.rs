use windows::Win32::System::EventLog::{
    RegisterEventSourceW,
    DeregisterEventSource,
    ReportEventW,
    EventSourceHandle,
    REPORT_EVENT_TYPE,
};
use windows::core::PCWSTR;
use std::iter;

pub const EVENT_TYPE_WARNING: u16 = 2; // 1=ERROR, 2=WARNING, 4=INFORMATION

/// Thin wrapper around a Windows `EventSourceHandle`
pub struct EventLogger {
    source_handle: EventSourceHandle,
}

impl EventLogger {
    /// Register "IvsAgent" under "Infopercept" (or any log/source you pass)
    pub fn register(source: &str, log: &str) -> Option<Self> {
        let registry_path = format!("SYSTEM\\CurrentControlSet\\Services\\EventLog\\{log}\\{source}");
        let wide_path: Vec<u16> = registry_path.encode_utf16().chain(iter::once(0)).collect();

        match unsafe { RegisterEventSourceW(None, PCWSTR(wide_path.as_ptr())) } {
            Ok(handle) => Some(Self { source_handle: handle }),
            Err(_) => None,
        }
    }

    /// Write a single-string event
    ///
    /// `event_type` = 1=ERROR, 2=WARNING, 4=INFORMATION, etc.
    pub fn write_entry(&self, message: &str, event_type: u16) {
        // Convert message to wide
        let wide_msg: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
        let pcwstr_msg = PCWSTR(wide_msg.as_ptr());

        let string_slice = [pcwstr_msg];
        let report_type = REPORT_EVENT_TYPE(event_type);

        unsafe {
            // 8-parameter version on your system:
            //   ReportEventW(
            //       heventlog,
            //       wtype,
            //       wcategory,
            //       dweventid,
            //       lpusersid,
            //       dwdatasize,
            //       lpstrings,
            //       lprawdata
            //   )
            let _ok = ReportEventW(
                self.source_handle.clone(), // 1) event source
                report_type,                // 2) event type (WARNING, etc.)
                0,                          // 3) category
                0,                          // 4) event ID
                None,                       // 5) user SID (None)
                0,                          // 6) dwDataSize
                Some(&string_slice),        // 7) slice of strings
                None,                       // 8) raw data
            );
            // If `_ok == 0`, you could do GetLastError() to see what failed.
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
