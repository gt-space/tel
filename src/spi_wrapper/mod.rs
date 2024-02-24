use spidev::{Spidev, SpidevTransfer};

use std::io;
use std::rc::Rc;

use crate::gpio::{Pin, PinMode::{Output}, PinValue::{High, Low}};

pub struct SpiWrapper {
    pub spidev: Rc<Spidev>,
    pub gpio_pin: Pin
}

impl SpiWrapper {
    // Constructs a new instance of a SPI Device
    pub fn new(spidev: Rc<Spidev>, gpio_pin: Pin) -> SpiWrapper {
        gpio_pin.mode(Output);
        gpio_pin.digital_write(High);
        return SpiWrapper {
            spidev: spidev,
            gpio_pin: gpio_pin
        };
    }

    pub fn read(&mut self, rx_buf: &mut [u8]) -> io::Result<()> {
        let mut transfer = SpidevTransfer::read(rx_buf);
        self.transfer(&mut transfer)
    }

    pub fn write(&mut self, tx_buf: &[u8]) -> io::Result<()> {
        let mut transfer = SpidevTransfer::write(&tx_buf);
        self.transfer(&mut transfer)
    }

    pub fn read_write(&mut self, rx_buf: &mut [u8], tx_buf: &[u8]) -> io::Result<()> {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, rx_buf);
        self.transfer(&mut transfer)
    }

    fn transfer(&mut self, transfer: &mut SpidevTransfer) -> io::Result<()> {
        self.gpio_pin.digital_write(Low);
        let result = self.spidev.transfer(transfer);
        // need to set back to high even if error in transfer
        self.gpio_pin.digital_write(High);
        return result
    }
}