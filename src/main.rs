use std::{thread::sleep, time};

use esp_idf_hal::{delay::BLOCK, prelude::Peripherals};

mod error;
mod mmwave;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // let peripherals = Peripherals::take().unwrap();

    // let mmtx = peripherals.pins.gpio11;
    // let mmrx = peripherals.pins.gpio10;
    let mut radar = mmwave::MMWave::new();
    radar.init();
    println!("Setting report mode");
    radar.set_mode_report().ok();
    sleep(time::Duration::from_millis(50));
    radar.read_frame();

    println!("Sending command to retrieve FW");
    radar.read_fw_version().ok();
    println!("Sent command to retrieve FW");
    println!("Reading a lil bit o data");
    loop {
        match radar.read_frame() {
            Ok(frame) => {
                sleep(time::Duration::from_millis(50));
            }
            Err(why) => {
                log::error!("{}", why);
                sleep(time::Duration::from_millis(200));
            }
        }
    }

    // log::info!("Hello, world!");
}
