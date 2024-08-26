use esp_idf_hal::{delay::BLOCK, prelude::Peripherals};

mod mmwave;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mmtx = peripherals.pins.gpio11;
    let mmrx = peripherals.pins.gpio10;
    let mut radar = mmwave::MMWave::new();
    radar.init();
    radar.read();

    log::info!("Hello, world!");
}
