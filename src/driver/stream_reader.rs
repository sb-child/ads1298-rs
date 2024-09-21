use byteorder::{BigEndian, ByteOrder};
use embedded_hal::spi::SpiDevice;
use ux::u24;

use super::{
    registers::{
        self,
        data::{DataStatus1, DataStatus2, DataStatus3},
        DataRegister,
    },
    StreamError, ADS1298,
};

/// `StreamReader` is used to continuously read data from the ADS1298 by using streaming mode.
pub struct StreamReader<'a, Spi: SpiDevice> {
    pub driver: &'a mut ADS1298<Spi>,
    buffer: Vec<u8>,
}

struct FieldConfig {
    register: registers::DataRegister,
    width: usize,
}

const LOOP_READ_BACK_CONFIG_FIELDS: &'static [FieldConfig] = &[
    FieldConfig {
        register: DataRegister::DATA_STATUS_1(DataStatus1(0)),
        width: 1,
    },
    FieldConfig {
        register: DataRegister::DATA_STATUS_2(DataStatus2(0)),
        width: 1,
    },
    FieldConfig {
        register: DataRegister::DATA_STATUS_3(DataStatus3(0)),
        width: 1,
    },
    FieldConfig {
        register: DataRegister::DATA_CH1(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH2(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH3(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH4(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH5(u24::new(0)),
        width: 3,
    },
    FieldConfig {
        register: DataRegister::DATA_CH6(u24::new(0)),
        width: 3,
    },
];

impl<'a, Spi: SpiDevice> StreamReader<'a, Spi> {
    pub fn new(driver: &'a mut ADS1298<Spi>) -> Result<Self, StreamError<Spi::Error>> {
        let buffer_len = LOOP_READ_BACK_CONFIG_FIELDS
            .iter()
            .map(|field| field.width)
            .sum::<usize>()
            + 1;
        let buffer: Vec<u8> = vec![0xff; buffer_len];
        Ok(Self { driver, buffer })
    }

    /// before read, please set `START` = `high`, and wait for `DRDY` become `high`
    pub fn read(&mut self) -> Result<Vec<registers::DataRegister>, StreamError<Spi::Error>> {
        let buffer = self
            .driver
            .operator
            .read_single_data(&mut self.buffer)
            .map_err(StreamError::StreamingAbort)?;

        let mut cursor = (0, 0);

        let result = LOOP_READ_BACK_CONFIG_FIELDS
            .iter()
            .map(|enabled_field| {
                let &FieldConfig { register, width } = enabled_field;

                let mut register = register.clone();
                cursor = (cursor.1, cursor.1 + width);

                macro_rules! ecg {
                    ($data: ident, $raw: ident) => {{
                        debug_assert!($raw.len() == 3);
                        *$data = u24::new(BigEndian::read_u24(&$raw))
                    }};
                }

                let raw = &buffer[cursor.0..cursor.1];
                match &mut register {
                    DataRegister::DATA_STATUS_1(data) => {
                        debug_assert!(raw.len() == 1);
                        *data = DataStatus1(raw[0]);
                    }
                    DataRegister::DATA_STATUS_2(data) => {
                        debug_assert!(raw.len() == 1);
                        *data = DataStatus2(raw[0]);
                    }
                    DataRegister::DATA_STATUS_3(data) => {
                        debug_assert!(raw.len() == 1);
                        *data = DataStatus3(raw[0]);
                    }
                    DataRegister::DATA_CH1(data) => ecg!(data, raw),
                    DataRegister::DATA_CH2(data) => ecg!(data, raw),
                    DataRegister::DATA_CH3(data) => ecg!(data, raw),
                    DataRegister::DATA_CH4(data) => ecg!(data, raw),
                    DataRegister::DATA_CH5(data) => ecg!(data, raw),
                    DataRegister::DATA_CH6(data) => ecg!(data, raw),
                    DataRegister::DATA_CH7(data) => ecg!(data, raw),
                    DataRegister::DATA_CH8(data) => ecg!(data, raw),
                }
                register
            })
            .collect::<Vec<_>>();

        Ok(result)
    }
}
