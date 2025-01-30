use anyhow::Context;
use connect::bluetooth::scan::Scanner;
use connect::logging;
use crossbeam_channel::{select, unbounded};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::block_on;
use log::{LevelFilter, error, info};
use positioning::beacon::{BeaconId, Output};
use positioning::signal::{Processor, Signal};
use positioning_offline::Locator;
use std::thread;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(LevelFilter::Info);
    logging::disable_logs("NimBLE").expect("NimBLE disabled");

    let peripherals = Peripherals::take().unwrap();

    let (bluetooth_tx, bluetooth_rx) = unbounded();
    let (signal_tx, signal_rx) = unbounded::<Vec<Signal<BeaconId>>>();
    let (position_tx, position_rx) = unbounded::<Output>();

    let signal_processor = Processor::default();
    let signal_processor_handle = signal_processor.start(bluetooth_rx, signal_tx);

    let locator = Locator::default();
    let locator_thread = locator
        .start(signal_rx, position_tx)
        .expect("Unable to start locator");

    let display_updater = thread::Builder::new()
        .name("display updater".to_string())
        .stack_size(8 * 1024)
        .spawn(move || {
            let mut display = connect::display::ssd1306::Oled::new(
                peripherals.i2c0,
                peripherals.pins.gpio5.into(),
                peripherals.pins.gpio4.into(),
            )
            .context("Error creating display")
            .unwrap();

            if let Err(e) = display.lat_lon(Output::default()) {
                error!("Error writing display: {:?}", e);
                return;
            }

            loop {
                select! {
                    recv(position_rx) -> enc => match enc {
                        Ok(output) => {
                           if let Err(e) = display.lat_lon(output) {
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
}
