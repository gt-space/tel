// This file adapted from the example in the rust-spidev README:
//     https://github.com/rust-embedded/rust-spidev

extern crate spidev;

mod spi_wrapper;
mod gpio;

use std::io;
use std::rc::Rc;
use futures::executor::block_on;
use spidev::{Spidev, SpidevOptions, SpiModeFlags};

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(20_000)
         .mode(SpiModeFlags::SPI_MODE_0)
         .build();
    spi.configure(&options)?;
    Ok(spi)
}

async fn async_main() {
    let spi = Rc::new(create_spi().unwrap());
    let gpio1 = gpio::Gpio::open(1);
    let mut device1 = spi_wrapper::SpiWrapper::new(spi, gpio1.get_pin(16));
    let mut rx_buf = [0_u8; 6];
    let _ = device1.read_write(&mut rx_buf, &[0x19, 0x09, 0x2B, 0x00, 0x00, 0x00]).await;
    println!("rx_buf1: {:?}", rx_buf);
}

fn main() {
    block_on(async_main());
}
