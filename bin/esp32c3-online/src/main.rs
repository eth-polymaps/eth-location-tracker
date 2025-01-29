use anyhow::Context;
use connect::bluetooth::scan::Scanner;
use connect::timer;
use connect::wifi::Wifi;
use crossbeam_channel::{select, unbounded};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::block_on;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::{LevelFilter, error, info};
use positioning::beacon::Room;
use positioning::geographic::Position;
use positioning::signal::{Processor, Signal};
use positioning_online::Locator;
use std::thread;

pub mod display;

unsafe extern "C" {
    fn esp_log_level_set(tag: *const u8, level: u32);
}

// Disables all logs for this module
const ESP_LOG_NONE: u32 = 0;

fn disable_nimble_logs() {
    let tag = b"NimBLE\0"; // C-style null-terminated string
    unsafe {
        esp_log_level_set(tag.as_ptr(), ESP_LOG_NONE);
    }
}

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
    disable_nimble_logs();
    log::set_max_level(LevelFilter::Info);

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = Wifi::new(peripherals.modem, sys_loop, nvs, wifi_ssid, wifi_password)
        .expect("Error while creating wifi");
    wifi.connect().expect("Unable to start WIFI");

    timer::synchronize().expect("Unable to start timer");

    let (bluetooth_tx, bluetooth_rx) = unbounded();
    let (signal_tx, signal_rx) = unbounded::<Vec<Signal>>();
    let (position_tx, position_rx) = unbounded::<(Position, Room)>();

    let signal_processor = Processor::default();
    let signal_processor_handle = signal_processor.start(bluetooth_rx, signal_tx);

    let locator = Locator::new(service_key, service_client_id, service_endpoint);
    let locator_thread = locator
        .start(signal_rx, position_tx)
        .expect("Failed to start locator");

    let display_updater = thread::Builder::new()
        .name("display updater".to_string())
        .stack_size(8 * 1024)
        .spawn(move || {
            let mut display = display::ssd1306::OLED::new(
                peripherals.i2c0,
                peripherals.pins.gpio5.into(),
                peripherals.pins.gpio4.into(),
            )
            .context("Error creating display")
            .unwrap();

            if let Err(e) = display.lat_lon(Position::default(), Room::default()) {
                error!("Error writing display: {:?}", e);
                return;
            }

            loop {
                select! {
                    recv(position_rx) -> enc => match enc {
                        Ok((pos, room)) => {
                           info!("Sending position {:?} and room {:?} to display", pos, room);
                           if let Err(e) = display.lat_lon(pos, room) {
                                error!("Error writing display from chan: {:?}", e);
                                return;
                            }
                        },
                        Err(e) => {
                            error!("Failed to read location {:?}", e);
                        }
                    }
                }
            }
        })
        .expect("Failed to create thread");

    block_on(async {
        let scanner = Scanner::new(5000i32, 100, 50);
        scanner.scan_indefinit(bluetooth_tx).await;
    });

    match locator_thread.join() {
        Ok(_) => info!("Locator thread completed successfully"),
        Err(e) => error!("Locator thread panicked: {:?}", e),
    }

    match signal_processor_handle.join() {
        Ok(_) => info!("Signal processor thread completed successfully"),
        Err(e) => error!("Signal processor thread panicked: {:?}", e),
    }

    match display_updater.join() {
        Ok(_) => info!("Display updater thread joined"),
        Err(_) => error!("Display updater thread panicked"),
    }

    info!("Scan done");
}
