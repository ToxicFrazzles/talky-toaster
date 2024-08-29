// use core::slice::SlicePattern;
use core::str;
use std::result;
use std::thread::sleep;
use std::time;

use anyhow::anyhow;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::io::Read;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart;
use esp_idf_hal::uart::*;

// use crate::error::MMWaveOffError;

const RADAR_COMMAND_READ_FW_VERSION: u16 = 0x00;
const RADAR_COMMAND_SET_CONFIG: u16 = 0x07;
const RADAR_COMMAND_READ_CONFIG: u16 = 0x08;
const RADAR_COMMAND_READ_SERIAL_NUMBER: u16 = 0x11;
const RADAR_COMMAND_SET_MODE: u16 = 0x12;
const RADAR_COMMAND_READ_MODE: u16 = 0x13;

pub struct MMWave<'d> {
    // uart_tx: gpio::AnyOutputPin,
    // uart_rx: gpio::AnyInputPin,
    driver: Option<UartDriver<'d>>,
    partial_frame_buffer: Vec<u8>,
}

impl<'d> MMWave<'d> {
    pub fn new() -> MMWave<'d> {
        MMWave {
            // uart_tx: tx,
            // uart_rx: rx,
            driver: None,
            partial_frame_buffer: vec![],
        }
    }

    pub fn init(&mut self) {
        let peripherals = Peripherals::take().unwrap();
        let config = UartConfig::new().baudrate(Hertz(115_200));
        let r = UartDriver::new(
            peripherals.uart1,
            peripherals.pins.gpio10,
            peripherals.pins.gpio11,
            Option::<gpio::Gpio0>::None,
            Option::<gpio::Gpio0>::None,
            &config,
        );
        if let Err(why) = r {
            log::error!("Receive error {:?}", why);
        }
        self.driver = Some(r.unwrap());

        sleep(time::Duration::from_millis(200));

        // self.set_mode_report().ok();
    }

    fn send_command(
        &mut self,
        command_code: u16,
        command_data: Option<Vec<u8>>,
    ) -> anyhow::Result<()> {
        if self.driver.is_none() {
            return Err(anyhow!(
                "MMWave UART is not initialised. Call the init method!"
            ));
        }

        let uart = self.driver.as_ref().unwrap();

        let mut data = command_data.unwrap_or(vec![]);
        let len = (2 + data.len()) as u16;
        let mut txdatabuff: Vec<u8> = vec![
            0xFD,
            0xFC,
            0xFB,
            0xFA,
            (len & 0xFF) as u8,
            (len >> 8) as u8,
            (command_code & 0xFF) as u8,
            (command_code >> 8) as u8,
        ];
        txdatabuff.append(&mut data);
        txdatabuff.append(&mut vec![0x04, 0x03, 0x02, 0x01]);

        uart.write(txdatabuff.as_slice())?;

        Ok(())
    }

    pub fn read_chunk(&mut self) -> anyhow::Result<()> {
        if self.driver.is_none() {
            return Err(anyhow!(
                "MMWave UART is not initialised. Call the init method!"
            ));
        }
        let uart = self.driver.as_ref().unwrap();
        let mut buff = [0_u8; 256];
        let result = uart.read(&mut buff, 0)?;
        if result == 0 {
            return Ok(());
        }
        let newData = buff.get(0..result).unwrap();
        self.partial_frame_buffer.extend_from_slice(newData);
        // println!("{:0>2X?}", self.partial_frame_buffer);
        Ok(())
    }

    pub fn find_frame(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut start = 0;
        let mut command_frame = false;
        let mut started = false;
        if self.partial_frame_buffer.len() < 8 {
            return Ok(vec![]);
        }
        for i in 0..self.partial_frame_buffer.len() - 8 {
            if self.partial_frame_buffer[i..i + 4].starts_with(&[0xFD, 0xFC, 0xFB, 0xFA]) {
                command_frame = true;
                start = i;
                started = true;
            } else if command_frame
                && started
                && self.partial_frame_buffer[i..i + 4].ends_with(&[0x04, 0x03, 0x02, 0x01])
            {
                let result = self.partial_frame_buffer[start..i + 4].to_vec();
                self.partial_frame_buffer.drain(0..i + 4);
                return Ok(result);
            } else if self.partial_frame_buffer[i..i + 4].starts_with(&[0xF4, 0xF3, 0xF2, 0xF1]) {
                command_frame = false;
                started = true;
                start = i;
            } else if !command_frame
                && started
                && self.partial_frame_buffer[i..i + 4].ends_with(&[0xF8, 0xF7, 0xF6, 0xF5])
            {
                let result = self.partial_frame_buffer[start..i + 4].to_vec();
                self.partial_frame_buffer.drain(0..i + 4);
                return Ok(result);
            }
        }
        if start > 0 {
            self.partial_frame_buffer.drain(0..start);
        }
        if self.partial_frame_buffer.len() > 256 {
            self.partial_frame_buffer
                .drain(0..256 - self.partial_frame_buffer.len());
        }
        Ok(vec![])
    }

    pub fn read_frame(&mut self) -> anyhow::Result<Vec<u8>> {
        if self.driver.is_none() {
            return Err(anyhow!(
                "MMWave UART is not initialised. Call the init method!"
            ));
        }

        let mut frame = self.find_frame()?;
        if frame.is_empty() {
            self.read_chunk();
            frame = self.find_frame()?;
            if frame.is_empty() {
                return Ok(vec![]);
            }
        }

        log::info!("{:0>2X?}", frame);

        Ok(frame)
    }

    pub fn read_fw_version(&mut self) -> anyhow::Result<Vec<u8>> {
        self.send_command(RADAR_COMMAND_READ_FW_VERSION, None)?;

        Ok(vec![])
    }

    pub fn set_mode_report(&mut self) -> anyhow::Result<()> {
        // if self.driver.is_none() {
        //     return Err(anyhow!(
        //         "MMWave UART is not initialised. Call the init method!"
        //     ));
        // }

        // let uart = self.driver.as_ref().unwrap();
        // uart.write(b"fdfcfbfa0800120000006400000004030201")?;

        self.send_command(
            RADAR_COMMAND_SET_MODE,
            Some(vec![0x00, 0x00, 0x04, 0x00, 0x00, 0x00]),
        )?;

        Ok(())
    }
}
