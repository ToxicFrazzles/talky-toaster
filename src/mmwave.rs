use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;
use esp_idf_hal::uart::*;

pub struct MMWave<'d> {
    // uart_tx: gpio::AnyOutputPin,
    // uart_rx: gpio::AnyInputPin,
    driver: Option<UartDriver<'d>>,
}

impl<'d> MMWave<'d> {
    pub fn new() -> MMWave<'d> {
        MMWave {
            // uart_tx: tx,
            // uart_rx: rx,
            driver: None,
        }
    }

    pub fn init(&mut self) {
        let peripherals = Peripherals::take().unwrap();
        let config = UartConfig::new().baudrate(Hertz(115_200));
        let r = UartDriver::new(
            peripherals.uart1,
            peripherals.pins.gpio11,
            peripherals.pins.gpio10,
            Option::<gpio::Gpio0>::None,
            Option::<gpio::Gpio0>::None,
            &config,
        );
        if let Err(why) = r {
            log::error!("Receive error {:?}", why);
        }
        self.driver = Some(r.unwrap());
    }

    pub fn read(&mut self) {
        if self.driver.is_none() {
            return;
        }

        let mut buffer = [0_u8; 4];
        let uart = self.driver.as_ref().unwrap();
        let result = uart.read(&mut buffer, BLOCK);
        if let Err(why) = result {
            log::error!("Receive error {:?}", why);
        }
        let result = result.unwrap();
        log::info!("{:?}", buffer);
    }
}

pub fn get_uart_driver() -> Option<UartDriver<'static>> {
    let peripherals = Peripherals::take().unwrap();

    let mmtx = peripherals.pins.gpio11;
    let mmrx = peripherals.pins.gpio10;

    let config = UartConfig::new().baudrate(Hertz(115_200));
    let mmwave = UartDriver::new(
        peripherals.uart1,
        mmtx,
        mmrx,
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio0>::None,
        &config,
    );

    if let Err(why) = mmwave {
        log::error!("{:?}", why);
        return None;
    }

    let mmwave = mmwave.unwrap();
    Some(mmwave)
}
