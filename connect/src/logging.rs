use std::ffi::CString;

unsafe extern "C" {
    fn esp_log_level_set(tag: *const u8, level: u32);
}

// Disables all logs for this module
const ESP_LOG_NONE: u32 = 0;

pub fn disable_logs(tag: &str) -> anyhow::Result<()> {
    // Convert Rust str to CString (adds 0 terminator)
    let c_tag = CString::new(tag).expect("CString conversion failed");
    unsafe {
        esp_log_level_set(c_tag.as_ptr(), ESP_LOG_NONE);
    }
    Ok(())
}
