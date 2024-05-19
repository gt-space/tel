// Represents commands that can be sent to the SX1280
use super::spi_wrapper;
enum Command {
    SetSleep,
    SetStandby,
    SetFs,
    SetTx,
    SetRx,
    StopTimerOnPreamble,
    SetRxDutyCycle,
    SetCad,
    SetTxContinuousWave,
    SetTxInfinitePreamble,
    SetRegulatorMode,
    Calibrate,
    CalibrateImage,
    SetPaConfig,
    SetRxTxFallbackMode,

    WriteRegister,
    ReadRegister,
    WriteBuffer,
    ReadBuffer,

    SetDioIrqParams,
    GetIrqStatus,
    ClearIrqStatus,
    SetDIO2AsRfSwitchCtrl,
    SetDIO3AsTcxoCtrl,

    SetRfFrequency,
    SetPacketType,
    GetPacketType,
    SetTxParams,
    SetModulationParams,
    SetPacketParams,
    SetCadParams,
    SetBufferBaseAddress,
    SetLoRaSymbNumTimeout,
    GetStatus,
    GetRxBufferStatus,
    GetPacketStatus,
    GetRssiInst,
    GetStats,
    ResetStats,
    
    GetDeviceErrors,
    ClearDeviceErrors,
}

impl Command {
    fn opcode(&self) -> u8 {
        match self {
            // 11.1 Operational Modes Commands
            Command::SetSleep => 0x84,
            Command::SetStandby => 0x80,
            Command::SetFs => 0xC1,
            Command::SetTx => 0x83,
            Command::SetRx => 0x82,
            Command::StopTimerOnPreamble => 0x9F,
            Command::SetRxDutyCycle => 0x94,
            Command::SetCad => 0xC5,
            Command::SetTxContinuousWave => 0xD1,
            Command::SetTxInfinitePreamble => 0xD2,
            Command::SetRegulatorMode => 0x96,
            Command::Calibrate => 0x89,
            Command::CalibrateImage => 0x98,
            Command::SetPaConfig => 0x95,
            Command::SetRxTxFallbackMode => 0x93,
            // 11.2 Register and Buffer Access Commands
            Command::WriteRegister => 0x0D,
            Command::ReadRegister => 0x1D,
            Command::WriteBuffer => 0x0E,
            Command::ReadBuffer => 0x1E,
            // 11.3 DIO and IRQ Control
            Command::SetDioIrqParams => 0x08,
            Command::GetIrqStatus => 0x12,
            Command::ClearIrqStatus => 0x02,
            Command::SetDIO2AsRfSwitchCtrl => 0x9D,
            Command::SetDIO3AsTcxoCtrl => 0x97,
            // 11.4 RF, Modulation and Packet Commands
            Command::SetRfFrequency => 0x86,
            Command::SetPacketType => 0x8A,
            Command::GetPacketType => 0x11,
            Command::SetTxParams => 0x8E,
            Command::SetModulationParams => 0x8B,
            Command::SetPacketParams => 0x8C,
            Command::SetCadParams => 0x88,
            Command::SetBufferBaseAddress => 0x8F,
            Command::SetLoRaSymbNumTimeout => 0xA0,
            // 11.5 Status Commands
            Command::GetStatus => 0xC0,
            Command::GetRssiInst => 0x15,
            Command::GetRxBufferStatus => 0x13,
            Command::GetPacketStatus => 0x14,
            Command::GetDeviceErrors => 0x17,
            Command::ClearDeviceErrors => 0x07,
            Command::GetStats => 0x10,
            Command::ResetStats => 0x00,
        }
    }
}

enum Register {
    HoppingEnable,
    PacketLength,
    NbHoppingBlocks,
    NbSymbols,
    Freq,
    DIOxOutputEnable,
    DIOxInputEnable,
    DIOxPullUpControl,
    DIOxPullDownControl,
    WhiteningInitialValueMSB,
    WhiteningInitialValueLSB,
    CRCInitialValue,
    CRCPolynomialValue,
    SyncWord,
    NodeAddress,
    BroadcastAddress,
    IQPolaritySetup,
    LoRaSyncWord,
    RandomNumberGen,
    TxModulation,
    RxGain,
    TxClampConfig,
    OCPConfiguration,
    RTCControl,
    XTATrim,
    XTBTrim,
    DIO3OutputVoltageControl,
    EventMask
}

enum SigBit {
    MSB,
    LSB,
}

impl Register {
    fn address(&self, index: u16, sigBit: SigBit) -> [u16;2] {
        match self {
            Register::HoppingEnable => [0x0385, 1],
            Register::PacketLength => [0x0386, 1],
            Register::NbHoppingBlocks => [0x0387, 1],
            Register::NbSymbols => {
                // TODO: check that index is between 0 and 15
                let base: u16 = 0x0388;
                let offset: u16 = index * 4;
                return [base + offset, 2]
            },
            Register::Freq => {
                let base: u16 = 0x038A;
                let offset: u16 = index * 4;
                return [base + offset, 2]
            },
            Register::DIOxOutputEnable => [0x0580, 1],
            Register::DIOxInputEnable => [0x0583, 1],
            Register::DIOxPullUpControl => [0x0584, 1],
            Register::DIOxPullDownControl => [0x0585, 1],
            
            Register::WhiteningInitialValueMSB => [0x06B8, 1],
            Register::WhiteningInitialValueLSB => [0x06B9, 1],
            Register::CRCInitialValue => {
                let base: u16 = 0x06BC;
                let offset: u16 = index;
                return [base + offset, 1]
            },
            Register::CRCPolynomialValue => {
                let base: u16 = 0x06BE;
                let offset: u16 = index;
                return [base + offset, 1]
            },
            Register::SyncWord => {
                let base: u16 = 0x06C0;
                let offset: u16 = index;
                return [base + offset, 1]
            },
            Register::NodeAddress => [0x06CD, 1],
            Register::BroadcastAddress => [0x06CE, 1],
            Register::IQPolaritySetup => [0x0736, 1],
            Register::LoRaSyncWord => {
                return match sigBit {
                    SigBit::MSB => [0x0740, 1],
                    SigBit::LSB => [0x0741, 1]
                }
            },
            Register::RandomNumberGen => {
                let base: u16 = 0x0819;
                let offset: u16 = index;
                return [base + offset, 1]
            },
            Register::TxModulation => [0x0889, 1],
            Register::RxGain => [0x08AC, 1],
            Register::TxClampConfig => [0x08D8, 1],
            Register::OCPConfiguration => [0x08E7, 1],
            Register::RTCControl => [0x0902, 1],
            Register::XTATrim => [0x0911, 1],
            Register::XTBTrim => [0x0912, 1],
            Register::DIO3OutputVoltageControl => [0x920, 1],
            Register::EventMask => [0x944, 1]
        }
    }
}

struct NBSymbolFreq {
    nb_symbol: u16,
    freq: u32,
}

struct SX1261_registers {
    hopping_enable: u8,
    packet_length: u8,
    nb_hopping_blocks: u8,
    nb_symbols: [NBSymbolFreq; 16],

    diox_output_enable: u32,
    diox_input_enable: u8,
    diox_pull_up_control: u8,
    diox_pull_down_control: u8,

    //whitening,
}

pub struct SX1261 {
    //regs: SX1280_registers,
    spi_wrapper: spi_wrapper::SpiWrapper,
}

impl SX1261 {
    pub fn new(spi_wrapper: spi_wrapper::SpiWrapper) -> SX1261 {
        return SX1261 {
            spi_wrapper
        }
    }
}

pub enum StdbyConfig {
    STDBY_RC = 0,
    STDBY_XOSC = 1,
}

impl SX1261 {
    pub fn SetStandby(&mut self, config: StdbyConfig) {
        let mut rx_buf =  [0_u8; 2];
        let tx_buf = [Command::SetStandby.opcode(), config as u8];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        println!("rx_buf1: {:?}", rx_buf);
    }
}

pub enum PacketType {
    PACKET_TYPE_GFSK = 0x00,
    PACKET_TYPE_LORA = 0x01,
    PACKET_TYPE_LR_FHSS = 0x03,
}

impl SX1261 {
    pub fn SetPacketType(&mut self, packetType: PacketType) {
        let mut rx_buf =  [0_u8; 2];
        let tx_buf = [Command::SetPacketType.opcode(), packetType as u8];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
    }
}

impl SX1261 {
    pub fn SetRFFrequency(&mut self, RFFreq: u32) -> [u8; 5] {
        let mut rx_buf = [0_u8; 5];
        // TODO: verify that SX1261 uses big endian
        let bytes = RFFreq.to_be_bytes();
        let tx_buf = [Command::SetRfFrequency.opcode(), bytes[0], bytes[1], bytes[2], bytes[3]];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

// RampTime units are in microseconds
pub enum RampTime {
    SET_RAMP_10U = 0x00,
    SET_RAMP_20U = 0x01,
    SET_RAMP_40U = 0x02,
    SET_RAMP_80U = 0x03,
    SET_RAMP_200U = 0x04,
    SET_RAMP_800U = 0x05,
    SET_RAMP_1700U = 0x06,
    SET_RAMP_3400U = 0x07
}

impl SX1261 {
    pub fn SetTxParams(&mut self, power: u8, rampTime: RampTime) -> [u8; 3] {
        let mut rx_buf = [0_u8; 3];
        let tx_buf = [Command::SetTxParams.opcode(), power, rampTime as u8];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn SetPaConfig(&mut self, paDutyCycle: u8, hpMax: u8) -> [u8; 5] {
        let mut rx_buf = [0_u8; 5];
        const deviceSel: u8 = 1; // always selecting 1261
        const paLut: u8 = 0x01;
        let tx_buf = [Command::SetPaConfig.opcode(), paDutyCycle, hpMax, deviceSel, paLut];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn SetBufferBaseAddress(&mut self, TX_base_address: u8, RX_base_address: u8) -> [u8; 3] {
        let mut rx_buf = [0_u8; 3];
        let tx_buf = [Command::SetBufferBaseAddress.opcode(), TX_base_address, RX_base_address];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn WriteBuffer(&mut self, offset: u8, data: &[u8]) -> Vec<u8> {
        let mut tx_buf = vec![Command::WriteBuffer.opcode(), offset];
        tx_buf.extend_from_slice(&data);
        let mut rx_buf = vec![0_u8; tx_buf.len()];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn SetModulationParams(&mut self, offset: u8, ModParam: &[u8; 8]) -> Vec<u8> {
        let mut tx_buf = vec![Command::SetModulationParams.opcode()];
        tx_buf.extend_from_slice(ModParam);
        let mut rx_buf = vec![0_u8; tx_buf.len()];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn SetPacketParams(&mut self, offset: u8, packetParam: &[u8; 9]) -> Vec<u8> {
        let mut tx_buf = vec![Command::SetPacketParams.opcode()];
        tx_buf.extend_from_slice(packetParam);
        let mut rx_buf = vec![0_u8; tx_buf.len()];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn SetDioIrqParams(&mut self, IrqMask: u16, DIO1Mask: u16, DIO2Mask: u16, DIO3Mask: u16) -> [u8; 9] {
        let IrqMaskBytes = IrqMask.to_be_bytes();
        let DIO1MaskBytes = IrqMask.to_be_bytes();
        let DIO2MaskBytes = IrqMask.to_be_bytes();
        let DIO3MaskBytes = IrqMask.to_be_bytes();
        
        let mut tx_buf = [Command::SetDioIrqParams.opcode(), IrqMaskBytes[0], IrqMaskBytes[1], DIO1MaskBytes[0], DIO1MaskBytes[1], DIO2MaskBytes[0], DIO2MaskBytes[1], DIO3MaskBytes[0], DIO3MaskBytes[1]];
        let mut rx_buf = [0_u8; 9];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn WriteRegister(&mut self, register: Register, data: &[u8]) -> Vec<u8> {
        let registerBytes = (register as u16).to_be_bytes();
        let mut tx_buf = vec![Command::WriteRegister.opcode(), registerBytes[0], registerBytes[1]];
        tx_buf.extend_from_slice(data);
        let mut rx_buf = vec![0_u8; tx_buf.len()];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}

impl SX1261 {
    pub fn SetTx(&mut self, timeout: u32) -> [u8; 4] {
        let timeoutBytes = timeout.to_be_bytes();
        let tx_buf = [Command::SetTx.opcode(), timeoutBytes[1], timeoutBytes[2], timeoutBytes[3]];
        let mut rx_buf = [0_u8; 4];
        let _ = self.spi_wrapper.read_write(&mut rx_buf, &tx_buf);
        return rx_buf
    }
}