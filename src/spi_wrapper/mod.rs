use spidev::{Spidev, SpidevTransfer};

use std::io;
use std::rc::Rc;
use std::sync::Mutex;

use crate::gpio::{Pin, PinMode::{Output}, PinValue::{High, Low}};

pub struct SpiWrapper {
    pub spidev: Rc<Spidev>,
    pub gpio_pin: Mutex<Pin> // only allow single read/write to a pin at a time
}

impl SpiWrapper {
    // Constructs a new instance of a SPI Device
    pub fn new(spidev: Rc<Spidev>, gpio_pin: Pin) -> SpiWrapper {
        gpio_pin.mode(Output);
        gpio_pin.digital_write(High);
        return SpiWrapper {
            spidev: spidev,
            gpio_pin: Mutex::new(gpio_pin)
        };
    }

    pub async fn read(&mut self, rx_buf: &mut [u8]) -> io::Result<()> {
        let mut transfer = SpidevTransfer::read(rx_buf);
        self.transfer(&mut transfer)
    }

    pub async fn write(&mut self, tx_buf: &[u8]) -> io::Result<()> {
        let mut transfer = SpidevTransfer::write(&tx_buf);
        self.transfer(&mut transfer)
    }

    pub async fn read_write(&mut self, rx_buf: &mut [u8], tx_buf: &[u8]) -> io::Result<()> {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, rx_buf);
        self.transfer(&mut transfer)
    }

    fn transfer(&mut self, transfer: &mut SpidevTransfer) -> io::Result<()> {
        // MutexGuard pin unlocked once out of scope
        let pin = self.gpio_pin.lock().unwrap();
        pin.digital_write(Low);
        let result = self.spidev.transfer(transfer);
        // need to set back to high even if error in transfer
        pin.digital_write(High);
        return result
    }
}