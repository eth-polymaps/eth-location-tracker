use connect::bluetooth::scan::Scanner;
use positioning_online::Locator;
use crossbeam_channel::unbounded;
use connect::wifi::Wifi;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::block_on;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sntp;
use log::{error, info, LevelFilter};
use std::thread;
use positioning::signal::{Processor, Signal};

fn main() {
    let wifi_ssid = env!("WIFI_SSID");
    let wifi_password = env!("WIFI_PASSWORD");
    let service_key = env!("LOCATION_SERVICE_KEY");
    let service_client_id = env!("LOCATION_SERVICE_CLIENT_ID");
    let service_endpoint = env!("LOCATION_SERVICE_ENDPOINT");

    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(LevelFilter::Info);

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = Wifi::new(peripherals, sys_loop, nvs, wifi_ssid, wifi_password);
    wifi.connect().expect("Unable to start WIFI");

    let sntp = sntp::EspSntp::new_default().expect("Unable to init time");
    loop {
        if sntp.get_sync_status() == sntp::SyncStatus::Completed {
            break;
        }
        thread::sleep(std::time::Duration::from_millis(500));
    }

    let (bluetooth_tx, bluetooth_rx) = unbounded();
    let (signal_tx, signal_rx) = unbounded::<Vec<Signal>>();

    let locator = Locator::new(service_key, service_client_id, service_endpoint);
    let signal_processor = Processor::new();

    let processor_thread = signal_processor.start(bluetooth_rx, signal_tx);

    let locator_thread = locator.start(signal_rx);

    block_on(async {
        let scanner = Scanner::new();
        scanner.scan_indefinit(bluetooth_tx).await;
    });

    match locator_thread.join() {
        Ok(_) => info!("Locator thread completed successfully"),
        Err(e) => error!("Locator thread panicked: {:?}", e),
    }

    match processor_thread.join() {
        Ok(_) => info!("Signal processor thread completed successfully"),
        Err(e) => error!("Signal processor thread panicked: {:?}", e),
    }

    info!("Scan done");
}
