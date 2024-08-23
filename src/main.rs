use esp_idf_svc::hal::{
    prelude::Peripherals,
    uart::{UartConfig, UartDriver},
};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mmtx = peripherals.pins.gpio10;
    let mmrx = peripherals.pins.gpio11;

    let config = UartConfig::new()
        .baudrate(115200)
        .parity_none()
        .stop_bits(1);
    let mmwave = UartDriver::new(peripherals.uart0, mmtx, mmrx, None, None, &config);

    if let Err(why) = mmwave {
        log::error!("{:?}", why);
    }

    log::info!("Hello, world!");
}
