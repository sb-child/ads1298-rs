use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use embedded_hal::spi::SpiDevice;
use registers::data::{DataStatus1, DataStatus2, DataStatus3};
use ux::u24;

use crate::driver::initialization::{Application3Lead, InitializeError, Initializer};
use crate::driver::registers::access::{ReadError, ReadFromRegister, WriteToRegister};
use crate::driver::registers::addressable::Addressable;

use self::operator::Operator;
use self::registers::addressable::Address;
use self::registers::data::{DataStatus, ErrorStatus, LoopReadBackConfig};
use self::registers::DataRegister;
use self::stream_reader::StreamReader;

pub mod initialization;
pub mod operator;
pub mod registers;
pub mod stream_reader;

pub struct ADS1298<SPI: SpiDevice> {
    pub operator: Operator<SPI>,
}

const LOOP_READ_BACK_CONFIG_FIELDS: &'static [registers::DataRegister] = &[
    // DataRegister::DATA_STATUS(DataStatus {
    //     ds1: DataStatus1(0),
    //     ds2: DataStatus2(0),
    //     ds3: DataStatus3(0),
    // }),
    DataRegister::DATA_STATUS_1(DataStatus1(0)),
    DataRegister::DATA_STATUS_2(DataStatus2(0)),
    DataRegister::DATA_STATUS_3(DataStatus3(0)),
    DataRegister::DATA_CH1(u24::new(0)),
    DataRegister::DATA_CH2(u24::new(0)),
    DataRegister::DATA_CH3(u24::new(0)),
    DataRegister::DATA_CH4(u24::new(0)),
    DataRegister::DATA_CH5(u24::new(0)),
    DataRegister::DATA_CH6(u24::new(0)),
    DataRegister::DATA_CH7(u24::new(0)),
    DataRegister::DATA_CH8(u24::new(0)),
];

const LOOP_READ_BACK_DATA_WIDTH: &'static [usize] = &[1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3];

impl<SPI: SpiDevice> ADS1298<SPI> {
    pub fn new(spi: SPI) -> ADS1298<SPI> {
        ADS1298 {
            operator: Operator::new(spi),
        }
    }

    // #[deprecated(note = "Use `stream_reader` instead")]
    // pub fn stream_one(&mut self) -> Result<Vec<registers::DataRegister>, StreamError<SPI::Error>> {
    //     let config = self
    //         .read(registers::CH_CNFG)
    //         .map_err(StreamError::ReadConfigError)?;

    //     let mut config_raw = config.0;
    //     let len: usize = LOOP_READ_BACK_CONFIG_FIELDS
    //         .iter()
    //         .enumerate()
    //         .map(|(i, ..)| i * LOOP_READ_BACK_DATA_WIDTH[i])
    //         .filter(|_| {
    //             let lowest_bit = config_raw & 1;
    //             config_raw = config_raw >> 1;
    //             lowest_bit == 1
    //         })
    //         .sum();

    //     debug_assert!(config_raw == 0);

    //     let mut buffer: Vec<u8> = vec![0xff; len + 1];

    //     let buffer = self
    //         .operator
    //         .stream(registers::DATA_LOOP.get_address(), &mut buffer)
    //         .map_err(StreamError::StreamingAbort)?;

    //     let mut config_raw = config.0;
    //     let mut cursor = (0, 0);
    //     let result = LOOP_READ_BACK_CONFIG_FIELDS
    //         .iter()
    //         .enumerate()
    //         .filter(|_| {
    //             let lowest_bit = config_raw & 1;
    //             config_raw = config_raw >> 1;
    //             lowest_bit == 1
    //         })
    //         .map(|(i, v)| {
    //             let width = LOOP_READ_BACK_DATA_WIDTH[i];
    //             let mut value = *v;
    //             cursor = (cursor.1, cursor.1 + width);

    //             macro_rules! pace {
    //                 ($data: ident, $raw: ident) => {{
    //                     debug_assert!($raw.len() == 2);
    //                     *$data = BigEndian::read_u16(&$raw)
    //                 }};
    //             }

    //             macro_rules! ecg {
    //                 ($data: ident, $raw: ident) => {{
    //                     debug_assert!($raw.len() == 3);
    //                     *$data = u24::new(BigEndian::read_u24(&$raw))
    //                 }};
    //             }

    //             let raw = &buffer[cursor.0..cursor.1];
    //             match &mut value {
    //                 DataRegister::DATA_STATUS(data) => {
    //                     debug_assert!(raw.len() == 1);
    //                     *data = DataStatus(raw[0]);
    //                 }
    //                 DataRegister::DATA_CH1_PACE(data) => pace!(data, raw),
    //                 DataRegister::DATA_CH2_PACE(data) => pace!(data, raw),
    //                 DataRegister::DATA_CH3_PACE(data) => pace!(data, raw),
    //                 DataRegister::DATA_CH1_ECG(data) => ecg!(data, raw),
    //                 DataRegister::DATA_CH2_ECG(data) => ecg!(data, raw),
    //                 DataRegister::DATA_CH3_ECG(data) => ecg!(data, raw),
    //             }

    //             value
    //         })
    //         .collect::<Vec<_>>();
    //     debug_assert!(config_raw == 0);

    //     Ok(result)
    // }

    pub fn stream_reader(&mut self) -> Result<StreamReader<SPI>, StreamError<SPI::Error>> {
        StreamReader::new(self)
    }
}

#[derive(Debug)]
pub enum StreamError<SpiError> {
    ReadConfigError(ReadError<SpiError>),
    StreamingAbort(ReadError<SpiError>),
}

impl<SPI: SpiDevice> Initializer<Application3Lead> for ADS1298<SPI> {
    type SpiError = SPI::Error;

    fn init(
        &mut self,
        _application: Application3Lead,
    ) -> Result<(), InitializeError<Self::SpiError>> {
        struct AddressData(u8, u8);

        const INITIAL_ADDRESS_DATA_ARR: &'static [AddressData] = &[
            AddressData(0x01, 0x11),
            AddressData(0x02, 0x19),
            AddressData(0x0A, 0x07),
            AddressData(0x0C, 0x04),
            AddressData(0x12, 0x04),
            AddressData(0x12, 0x04),
            AddressData(0x14, 0x24),
            AddressData(0x21, 0x02),
            AddressData(0x22, 0x02),
            AddressData(0x23, 0x02),
            AddressData(0x27, 0x08),
            // Additionally enable data status retrieving.
            AddressData(0x2F, 0x31),
            AddressData(0x00, 0x01),
        ];

        for address_data in INITIAL_ADDRESS_DATA_ARR {
            self.operator
                .write(address_data.0, address_data.1)
                .map_err(|e| InitializeError::WriteError {
                    source: e,
                    address: address_data.0,
                    data: address_data.1,
                })?;
        }

        Ok(())
    }
}

impl<SPI: SpiDevice> ReadFromRegister<registers::CONFIG, MainConfig, SPI::Error> for ADS1298<SPI> {
    fn read(&mut self, register: CONFIG) -> Result<MainConfig, ReadError<SPI::Error>> {
        let data = self.operator.read(register.get_address())?;
        Ok(MainConfig(data))
    }
}

impl<SPI: SpiDevice> ReadFromRegister<registers::DATA_STATUS, DataStatus, SPI::Error>
    for ADS1298<SPI>
{
    fn read(
        &mut self,
        register: registers::DATA_STATUS,
    ) -> Result<DataStatus, ReadError<SPI::Error>> {
        let data = self.operator.read(register.get_address())?;
        Ok(DataStatus(data))
    }
}

impl<SPI: SpiDevice> ReadFromRegister<registers::CH_CNFG, LoopReadBackConfig, SPI::Error>
    for ADS1298<SPI>
{
    fn read(
        &mut self,
        register: registers::CH_CNFG,
    ) -> Result<LoopReadBackConfig, ReadError<SPI::Error>> {
        let data = self.operator.read(register.get_address())?;
        Ok(LoopReadBackConfig(data))
    }
}

impl<SPI: SpiDevice> ReadFromRegister<registers::ERROR_STATUS, ErrorStatus, SPI::Error>
    for ADS1298<SPI>
{
    fn read(
        &mut self,
        register: registers::ERROR_STATUS,
    ) -> Result<ErrorStatus, ReadError<SPI::Error>> {
        let data = self.operator.read(register.get_address())?;
        Ok(ErrorStatus(data))
    }
}

impl<SPI: SpiDevice> ReadFromRegister<Address, u8, SPI::Error> for ADS1298<SPI> {
    fn read(&mut self, register: Address) -> Result<u8, ReadError<SPI::Error>> {
        self.operator.read(register)
    }
}
