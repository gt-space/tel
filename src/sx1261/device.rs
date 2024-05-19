// Represents commands that can be sent to the SX1280
enum Command {
    SetSleep,
    SetStandby,
    SetFs,
    SetTx,
    SetRx,
    StopTimerOnPreamble,
    SetRxDutyCycle,
    SetCAD,
    SetTxContinuousWave,
    SetTxInfinitePreamble,
    SetRegulatorMode,
    CalibrateFunction,
    CalibrateImage,
    SetPaConfig,
    SetRxTxFallbackMode,

    WriteRegister,
    ReadRegister,
    WriteBuffer,
    ReadBuffer,

    SetDioIrqParams,
    IrqMask,
    GetIrqStatus,
    ClearIrqStatus,
    SetDIO2AsRfSwitchCtrl,
    SetDIO3AsTCXOCtrl,

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
        match *self {
            // 11.1 Operational Modes Commands
            SetSleep => 0x84,
            SetStandby => 0x80,
            SetFs => 0xC1,
            SetTx => 0x83,
            SetRx => 0x82,
            StopTimerOnPreamble => 0x9F,
            SetRxDutyCycle => 0x94,
            SetCad => 0xC5,
            SetTxContinuousWave => 0xD1,
            SetTxInfinitePreamble => 0xD2,
            SetRegulatorMode => 0x96,
            Calibrate => 0x89,
            CalibrateImage => 0x98,
            SetPaConfig => 0x95,
            SetRxTxFallbackMode => 0x93,
            // 11.2 Register and Buffer Access Commands
            WriteRegister => 0x0D,
            ReadRegister => 0x1D,
            WriteBuffer => 0x0E,
            ReadBuffer => 0x1E,
            // 11.3 DIO and IRQ Control
            SetDioIrqParams => 0x08,
            GetIrqStatus => 0x12,
            ClearIrqStatus => 0x02,
            SetDIO2AsRfSwitchCtrl => 0x9D,
            SetDIO3AsTcxoCtrl => 0x97,
            // 11.4 RF, Modulation and Packet Commands
            SetRfFrequency => 0x86,
            SetPacketType => 0x8A,
            GetPacketType => 0x11,
            SetTxParams => 0x8E,
            SetModulationParams => 0x8B,
            SetPacketParams => 0x8C,
            SetCadParams => 0x88,
            SetBufferBaseAddress => 0x8F,
            SetLoRaSymbNumTimeout => 0xA0,
            // 11.5 Status Commands
            GetStatus => 0xC0,
            GetRssiInst => 0x15,
            GetRxBufferStatus => 0x13,
            GetPacketStatus => 0x14,
            GetDeviceErrors => 0x17,
            ClearDeviceErrors => 0x07,
            GetStats => 0x10,
            ResetStats => 0x00,
        }
    }
}

const REG_ID_HOPPINGENABLE: u32 = 0x0385;

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
    CRCInitialValue,
    CRCPolynomialValue,
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
    DIOS3OutputVoltageControl,
    EventMask
}

struct NBSymbolFreq {
    nb_symbol: u16,
    freq: u32,
}

struct SX1261_registers {
    hopping_enable: u8,
    packet_length: u8,
    nb_hopping_blocks: u8,
    
    diox_output_enable: u32,
    diox_input_enable: u8,
    diox_pull_up_control: u8,
    diox_pull_down_control: u8,

    whitening_initial_value_msb: u8,
    whitening_initial_value_lsb: u8,
    crc_msb_initial_value: u8,
    crc_lsb_initial_value: u8,
    crc_msb_polynomial_value: u8,
    crc_lsb_polynomial_value: u8,
    //whitening,
    sync_word: [u8; 8],
    node_address: u8,
    broadcast_address: u8,

    iq_polarity_setup: u8,
    
}

const REG_ID_RXGAIN: u16 = 0x981;

struct SX1280_registers {
    rx_gain: u8,
    manual_gain_setting: u8,
    lna_gain_value: u8,
    lna_gain_control: u8,
    sync_peak_attenuation: u8,
    payload_length: u8,
    lora_header_mode: u8,
    ranging_request_addr: [u8; 4],
    ranging_device_addr: [u8; 4],
    ranging_filter_window_size: u8,
    reset_ranging_filter: u8,
    ranging_result_mux: u8,
    sf_additional_configuration: u8,
    ranging_calibration_byte: [u8; 3],
    ranging_id_check_length: u8,
    frequency_error_correction: u8,
    lora_sync_word: [u8; 2],
    fei_byte: [u8; 3],
    ranging_result_byte: [u8; 3],
    ranging_rssi: u8,
    freeze_ranging_result: u8,
    packet_preamble_settings: u8,
    whitening_initial_value: u8,
    crc_polynomial_definition: u16,
    crc_polynomial_seed: u32,
    crc_initial_value: u16,
    sync_address_control: u8,
    sync_address_1: u64,
    sync_address_2: u64,
    sync_address_3: u64,
}

pub struct SX1261 {
    regs: SX1280_registers,
    spiWrapper: SpiWrapper,
}

impl SX1261 {
    pub fn new(spiWrapper: SpiWrapper) {
        return SX1261 {
            spiWrapper: spiWrapper
        }
    }
}