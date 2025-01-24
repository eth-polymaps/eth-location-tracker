use std::thread::sleep;
use std::time::Duration;
use esp_idf_svc::sntp;

pub fn synchronize() -> Result<(), anyhow::Error> {
    let sntp = sntp::EspSntp::new_default()?;
    loop {
        if sntp.get_sync_status() == sntp::SyncStatus::Completed {
            return Ok(());
        }
        sleep(Duration::from_millis(500));
    }
}
