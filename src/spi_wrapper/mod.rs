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

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::gpio::{Gpio};

    #[test]
    fn test_initialize() {
        let spi = match Spidev::open("/dev/spidev0.0") {
            Ok(spi) => spi,
            Err(_) => panic!("Unable to open SPI device")
        };
        let gpio1 = Gpio::open(1);
        let pin = gpio1.get_pin(16);
        let _ = SpiWrapper::new(Rc::new(spi), pin);

        // we don't want to let pin be cloned since we should only
        // be using a single SpiWrapper per pin
        let same_pin = gpio1.get_pin(16);

        assert_eq!(same_pin.get_mode(), Output);
        assert_eq!(same_pin.get_value(), High);
    }

    #[test]
    fn test_transfer() {
        let spi = match Spidev::open("/dev/spidev0.0") {
            Ok(spi) => spi,
            Err(_) => panic!("Unable to open SPI device")
        };
        let gpio1 = Gpio::open(1);
        let pin = gpio1.get_pin(16);

        let mut device1 = SpiWrapper::new(Rc::new(spi), pin);
        let mut rx_buf = [0_u8; 6];
        let _ = block_on(device1.read_write(&mut rx_buf, &[0x19, 0x09, 0x2B, 0x00, 0x00, 0x00]));

        let same_pin = gpio1.get_pin(16);

        // ensure that pin settings are reset to the same as before transfer
        assert_eq!(same_pin.get_mode(), Output);
        assert_eq!(same_pin.get_value(), High);
    }
}