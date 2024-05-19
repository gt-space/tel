// This file adapted from the example in the rust-spidev README:
//     https://github.com/rust-embedded/rust-spidev

extern crate spidev;

mod spi_wrapper;
mod gpio;
mod sx1261;

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
    let device1 = spi_wrapper::SpiWrapper::new(spi, gpio1.get_pin(71));
    // let mut rx_buf = [0_u8; 6];
    // let _ = device1.read_write(&mut rx_buf, &[0x19, 0x09, 0x2B, 0x00, 0x00, 0x00]).await;
    // println!("rx_buf1: {:?}", rx_buf);
    let mut sx1261 = sx1261::SX1261::new(device1);
    sx1261.SetStandby(sx1261::StdbyConfig::STDBY_RC);
    sx1261.SetPacketType(sx1261::PacketType::PACKET_TYPE_GFSK);
    let fxtal: u32 = 32 * 1000000; // 10^6 = mhz
    let target: u32 = 915;
    let RfFreq: u32 = target * u32::pow(2, 25) / fxtal;
    sx1261.SetRFFrequency(RfFreq);
    sx1261.SetPaConfig(paDutyCycle, hpMax);
    sx1261.SetTxParams(power, rampTime);
    sx1261.SetBufferBaseAddress(TX_base_address, RX_base_address);
    sx1261.WriteBuffer(offset, data);
    sx1261.SetModulationParams(offset, ModParam);
    sx1261.SetPacketParams(offset, packetParam);
    sx1261.SetDioIrqParams(IrqMask, DIO1Mask, DIO2Mask, DIO3Mask);
    sx1261.WriteRegister(register, data);
    sx1261.SetTx(timeout);
}

fn main() {
    block_on(async_main());
}
