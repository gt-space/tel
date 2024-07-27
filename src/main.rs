// This file adapted from the example in the rust-spidev README:
//     https://github.com/rust-embedded/rust-spidev

extern crate spidev;

use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};
use std::thread;
use std::time::Duration;
pub mod gpio;

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(1_000_000)
         .mode(SpiModeFlags::SPI_MODE_0)
         .build();
    spi.configure(&options)?;
    Ok(spi)
}

/// perform half duplex operations using Read and Write traits
fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
    let mut rx_buf = [0_u8; 10];
    spi.write(&[0x01, 0x02, 0x03])?;
    spi.read(&mut rx_buf)?;
    println!("{:?}", rx_buf);
    Ok(())
}

/// Perform full duplex operations using Ioctl
fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let tx_buf = [0x19, 0x08, 0x91, 0x00, 0x00];
    let mut rx_buf = [0; 5];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    println!("{:?}", rx_buf);
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();
    // println!("{:?}", half_duplex(&mut spi).unwrap());
    // println!("{:?}", full_duplex(&mut spi).unwrap());
    gpio::set_output("49");
    gpio::set_high("49"); // power

    gpio::set_output("9");
    gpio::set_low("9"); // NRESET
    thread::sleep(Duration::from_millis(100));
    gpio::set_high("9");

    gpio::set_output("72"); // LF-CS
    gpio::set_high("72");
    gpio::set_output("44"); // GPS-CS
    gpio::set_high("44");
    gpio::set_output("81"); // HF-CS
    loop {
        gpio::set_low("81");
        thread::sleep(Duration::from_micros(200));
        println!("{:?}", full_duplex(&mut spi).unwrap());
        gpio::set_high("81");
        thread::sleep(Duration::from_millis(100));
    }
}
