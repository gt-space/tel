use super::SX1261;
use super::Command;

enum StdbyConfig {
    STDBY_RC = 0,
    STDBY_XOSC = 1,
}

impl SX1261 {
    pub fn setStandby(&self, config: StdbyConfig) {
        let mut rx_buf = [0_u8; 2];
        let mut tx_buf = [Command::SetStandby.opcode(), config as u8];
        let _ = device1.read_write(&mut rx_buf, &tx_buf).await;
        println!("rx_buf1: {:?}", rx_buf);
    }
}