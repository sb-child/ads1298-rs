use embedded_hal::spi::SpiDevice;
use registers::access::WriteError;
use registers::data::{
    ChSetReg, Config1Reg, Config2Reg, Config3Reg, Config4Reg, GpioReg, IdReg, LOffReg,
    LOffSensNReg, LOffSensPReg, LoffFlipReg, PaceReg, RespReg, RldSensNReg, RldSensPReg, Wct1Reg,
    Wct2Reg,
};
use registers::{
    CH1SET, CH2SET, CH3SET, CH4SET, CH5SET, CH6SET, CH7SET, CH8SET, CONFIG1, CONFIG2, CONFIG3,
    CONFIG4, GPIO, ID, LOFF, LOFF_FLIP, LOFF_SENSN, LOFF_SENSP, PACE, RESP, RLD_SENSN, RLD_SENSP,
    WCT1, WCT2,
};

use crate::driver::initialization::{Default8Lead1x8K, InitializeError, Initializer};
use crate::driver::registers::access::{ReadError, ReadFromRegister, WriteToRegister};
use crate::driver::registers::addressable::Addressable;

use self::operator::Operator;

use self::stream_reader::StreamReader;

pub mod initialization;
pub mod operator;
pub mod registers;
pub mod stream_reader;

pub struct ADS1298<SPI: SpiDevice> {
    pub operator: Operator<SPI>,
}

impl<SPI: SpiDevice> ADS1298<SPI> {
    pub fn new(spi: SPI) -> ADS1298<SPI> {
        ADS1298 {
            operator: Operator::new(spi),
        }
    }

    pub fn stream_reader(&mut self) -> Result<StreamReader<SPI>, StreamError<SPI::Error>> {
        StreamReader::new(self)
    }
}

#[derive(Debug)]
pub enum StreamError<SpiError> {
    ReadConfigError(ReadError<SpiError>),
    StreamingAbort(ReadError<SpiError>),
}

impl<SPI: SpiDevice> Initializer<Default8Lead1x8K> for ADS1298<SPI> {
    type SpiError = SPI::Error;

    /// Before init, please set `CLKSEL` to what you need, and wait for 20 us.
    /// Then set `PDWN` = `high` and `RESET` = `high`, and wait for > 150 ms.
    fn init(
        &mut self,
        _application: Default8Lead1x8K,
    ) -> Result<(), InitializeError<Self::SpiError>> {
        // 重置芯片
        self.operator
            .reset()
            .map_err(|e| InitializeError::ResetError(e, Some(format!("Failed to reset chip"))))?;
        // 停止数据连续发送
        self.operator.stop_stream().map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to disable converting mode")))
        })?;
        // 高分辨率模式, 输出数据速率 8kSPS
        self.write(CONFIG1, {
            let mut x = Config1Reg(0);
            x.set_hr(true);
            x.set_dr(0b010);
            x
        })
        .map_err(|e| InitializeError::ResetError(e, Some(format!("Failed to set sample rate"))))?;
        // 不更改配置寄存器2
        self.write(CONFIG2, {
            let x = Config2Reg(0);
            x
        })
        .map_err(|e| InitializeError::ResetError(e, Some(format!("Failed to set CONFIG2"))))?;
        // 使用内部基准
        self.write(CONFIG3, {
            let mut x = Config3Reg(0);
            x.set_rev_6(true);
            x.set_pd_refbuf(true);
            x
        })
        .map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to switch to internal reference")))
        })?;
        // 调节所有通道增益为 1
        let data = {
            let mut x = ChSetReg(0);
            x.set_mux(0b000);
            x.set_gain(0b001);
            x
        };
        self.write(CH1SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH1")))
        })?;
        self.write(CH2SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH2")))
        })?;
        self.write(CH3SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH3")))
        })?;
        self.write(CH4SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH4")))
        })?;
        self.write(CH5SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH5")))
        })?;
        self.write(CH6SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH6")))
        })?;
        self.write(CH7SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH7")))
        })?;
        self.write(CH8SET, data).map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to set gain for CH8")))
        })?;

        // 启动转换
        self.operator.start().map_err(|e| {
            InitializeError::ResetError(e, Some(format!("Failed to enable converting mode")))
        })?;

        Ok(())
    }
}

macro_rules! impl_rw_reg {
    ($reg: ident, $result_type: tt) => {
        impl<SPI: SpiDevice> ReadFromRegister<$reg, $result_type, SPI::Error> for ADS1298<SPI> {
            fn read(&mut self, register: $reg) -> Result<$result_type, ReadError<SPI::Error>> {
                let data = self.operator.read(register.get_address())?;
                Ok($result_type(data))
            }
        }
        impl<SPI: SpiDevice> WriteToRegister<$reg, $result_type, SPI::Error> for ADS1298<SPI> {
            fn write(
                &mut self,
                register: $reg,
                data: $result_type,
            ) -> Result<(), WriteError<SPI::Error>> {
                self.operator.write(register.get_address(), data.0)?;
                Ok(())
            }
        }
    };
}

impl_rw_reg!(ID, IdReg);
impl_rw_reg!(CONFIG1, Config1Reg);
impl_rw_reg!(CONFIG2, Config2Reg);
impl_rw_reg!(CONFIG3, Config3Reg);
impl_rw_reg!(LOFF, LOffReg);
impl_rw_reg!(CH1SET, ChSetReg);
impl_rw_reg!(CH2SET, ChSetReg);
impl_rw_reg!(CH3SET, ChSetReg);
impl_rw_reg!(CH4SET, ChSetReg);
impl_rw_reg!(CH5SET, ChSetReg);
impl_rw_reg!(CH6SET, ChSetReg);
impl_rw_reg!(CH7SET, ChSetReg);
impl_rw_reg!(CH8SET, ChSetReg);
impl_rw_reg!(RLD_SENSP, RldSensPReg);
impl_rw_reg!(RLD_SENSN, RldSensNReg);
impl_rw_reg!(LOFF_SENSP, LOffSensPReg);
impl_rw_reg!(LOFF_SENSN, LOffSensNReg);
impl_rw_reg!(LOFF_FLIP, LoffFlipReg);
impl_rw_reg!(GPIO, GpioReg);
impl_rw_reg!(PACE, PaceReg);
impl_rw_reg!(RESP, RespReg);
impl_rw_reg!(CONFIG4, Config4Reg);
impl_rw_reg!(WCT1, Wct1Reg);
impl_rw_reg!(WCT2, Wct2Reg);
