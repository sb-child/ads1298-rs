use byteorder::{BigEndian, ByteOrder};
use embedded_hal::spi::SpiDevice;
use ux::u24;

use super::{
    registers::{
        self,
        access::ReadFromRegister,
        addressable::Addressable,
        data::{DataStatus, LoopReadBackConfig},
        DataRegister,
    },
    StreamError, ADS1298,
};

/// `StreamReader` is used to continuously read data from the ADS1298 by using streaming mode.
pub struct StreamReader<'a, Spi: SpiDevice> {
    pub driver: &'a mut ADS1298<Spi>,
    _config: LoopReadBackConfig,
    enabled_fields: Vec<&'static FieldConfig>,
    buffer: Vec<u8>,
}

struct FieldConfig {
    register: registers::DataRegister,
    width: usize,
}

const LOOP_READ_BACK_CONFIG_FIELDS: &'static [FieldConfig] = &[
    FieldConfig {
        register: DataRegister::DATA_STATUS(DataStatus(0)),
        width: 1,
    },
    FieldConfig {
        register: DataRegister::DATA_CH1_PACE(0),
        width: 2,
    },
    FieldConfig {
        register: DataRegister::DATA_CH2_PACE(0),
        width: 2,
    },
    FieldConfig {
        register: DataRegister::DATA_CH3_PACE(0),
        width: 2,
    },
    FieldConfig {
        register: DataRegister::DATA_CH1_ECG(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH2_ECG(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH3_ECG(u24::new(0)),
        width: 3,
    },
];

impl<'a, Spi: SpiDevice> StreamReader<'a, Spi> {
    pub fn new(driver: &'a mut ADS1298<Spi>) -> Result<Self, StreamError<Spi::Error>> {
        let config = driver
            .read(registers::CH_CNFG)
            .map_err(StreamError::ReadConfigError)?;

        let mut config_raw_bytes = config.0;

        let enabled_fields = LOOP_READ_BACK_CONFIG_FIELDS
            .iter()
            .filter(|_| {
                let lowest_bit = config_raw_bytes & 1;
                config_raw_bytes >>= 1;
                lowest_bit == 1
            })
            .collect::<Vec<_>>();

        let buffer_len = enabled_fields
            .iter()
            .map(|field| field.width)
            .sum::<usize>()
            + 1;

        let buffer: Vec<u8> = vec![0xff; buffer_len];

        Ok(Self {
            driver,
            _config: config,
            enabled_fields,
            buffer,
        })
    }

    pub fn read(&mut self) -> Result<Vec<registers::DataRegister>, StreamError<Spi::Error>> {
        let buffer = self
            .driver
            .operator
            .stream(registers::DATA_LOOP.get_address(), &mut self.buffer)
            .map_err(StreamError::StreamingAbort)?;

        let mut cursor = (0, 0);

        let result = self
            .enabled_fields
            .iter()
            .map(|enabled_field| {
                let &FieldConfig { register, width } = enabled_field;

                let mut register = register.clone();
                cursor = (cursor.1, cursor.1 + width);

                macro_rules! pace {
                    ($data: ident, $raw: ident) => {{
                        debug_assert!($raw.len() == 2);
                        *$data = BigEndian::read_u16(&$raw)
                    }};
                }

                macro_rules! ecg {
                    ($data: ident, $raw: ident) => {{
                        debug_assert!($raw.len() == 3);
                        *$data = u24::new(BigEndian::read_u24(&$raw))
                    }};
                }

                let raw = &buffer[cursor.0..cursor.1];
                match &mut register {
                    DataRegister::DATA_STATUS(data) => {
                        debug_assert!(raw.len() == 1);
                        *data = DataStatus(raw[0]);
                    }
                    DataRegister::DATA_CH1_PACE(data) => pace!(data, raw),
                    DataRegister::DATA_CH2_PACE(data) => pace!(data, raw),
                    DataRegister::DATA_CH3_PACE(data) => pace!(data, raw),
                    DataRegister::DATA_CH1_ECG(data) => ecg!(data, raw),
                    DataRegister::DATA_CH2_ECG(data) => ecg!(data, raw),
                    DataRegister::DATA_CH3_ECG(data) => ecg!(data, raw),
                }

                register
            })
            .collect::<Vec<_>>();

        Ok(result)
    }
}
