use embedded_hal::spi::{Operation, SpiDevice};
use ux::u5;

use crate::driver::registers::access::{ReadError, ReadFromRegister, WriteError};

use super::registers::{access::WriteToRegister, addressable::Address};

/// 500 ns for transfer a single bit
static TCLK_2_048M: u32 = 500;

/// Table 15. 操作码命令定义
#[derive(Clone, Copy)]
pub enum OpCode {
    /// 退出待机模式
    WakeUp,
    /// 进入待机模式
    StandBy,
    /// 复位器件
    Reset,
    /// 启动/重新启动（同步）转换
    Start,
    /// 停止转换
    Stop,
    /// 启用连续读取数据模式。该模式是上电时的默认模式。
    RDataC,
    /// 停止连续读取数据模式
    SDataC,
    /// 通过命令读取数据；支持多个读回。
    RData,
    /// 从地址 start 开始读取 n+1 个寄存器
    RReg { start: u5, n: u5 },
    /// 从地址 start 开始写入 n+1 个寄存器
    WReg { start: u5, n: u5 },
}

impl Into<Vec<u8>> for OpCode {
    fn into(self) -> Vec<u8> {
        match self {
            OpCode::WakeUp => vec![0b0000_0010],
            OpCode::StandBy => vec![0b0000_0100],
            OpCode::Reset => vec![0b0000_0110],
            OpCode::Start => vec![0b0000_1000],
            OpCode::Stop => vec![0b0000_1010],
            OpCode::RDataC => vec![0b0001_0000],
            OpCode::SDataC => vec![0b0001_0001],
            OpCode::RData => vec![0b0001_0010],
            OpCode::RReg { start, n } => {
                vec![0b0010_0000 | u8::from(start), u8::from(n)]
            }
            OpCode::WReg { start, n } => {
                vec![0b0100_0000 | u8::from(start), u8::from(n)]
            }
        }
    }
}

pub struct Operator<SPI: SpiDevice> {
    spi: SPI,
}

impl<SPI: SpiDevice> Operator<SPI> {
    pub fn new(spi: SPI) -> Operator<SPI> {
        Operator { spi }
    }
}

impl<SPI: SpiDevice> WriteToRegister<Address, u8, SPI::Error> for Operator<SPI> {
    fn write(&mut self, address: Address, data: u8) -> Result<(), WriteError<SPI::Error>> {
        let mut buffer: Vec<_> = OpCode::WReg {
            start: u5::new(address),
            n: u5::new(0),
        }
        .into();
        buffer.push(data);
        self.spi
            .transaction(&mut [Operation::Write(&buffer)])
            .map_err(WriteError::SpiTransferError)?;
        log::debug!("Write {data:#04x} to the address {address:#04x} of ADS1298",);
        Ok(())
    }
}

impl<SPI: SpiDevice> ReadFromRegister<Address, u8, SPI::Error> for Operator<SPI> {
    fn read(&mut self, address: Address) -> Result<u8, ReadError<SPI::Error>> {
        let buffer: Vec<_> = OpCode::RReg {
            start: u5::new(address),
            n: u5::new(0),
        }
        .into();
        let mut r_buffer = [0u8; 1];
        self.spi
            .transaction(&mut [Operation::Transfer(&mut r_buffer, &buffer)])
            .map_err(ReadError::SpiTransferError)?;
        Ok(r_buffer[0])
    }
}

/// todo
impl<SPI: SpiDevice> Operator<SPI> {
    pub fn stream<'a>(
        &mut self,
        address: Address,
        mut buffer: &'a mut [u8],
    ) -> Result<&'a mut [u8], ReadError<SPI::Error>> {
        let command = address | (1u8 << 7);
        buffer[0] = command;

        self.spi
            .transaction(&mut [Operation::TransferInPlace(&mut buffer)])
            .map_err(ReadError::SpiTransferError)?;

        Ok(&mut buffer[1..])
    }
}

impl<SPI: SpiDevice> Operator<SPI> {
    /// 读取一次数据
    ///
    /// buffer size need to be 27 bytes
    pub fn read_single_data<'a>(
        &mut self,
        mut buffer: &'a mut [u8],
    ) -> Result<&'a mut [u8], ReadError<SPI::Error>> {
        let command: Vec<_> = OpCode::RData.into();
        self.spi
            .transaction(&mut [Operation::Transfer(&mut buffer, &command)])
            .map_err(ReadError::SpiTransferError)?;
        Ok(buffer)
    }

    /// 退出待机模式
    ///
    /// 需要 `4` 个 tCLK 周期
    pub fn wake_up<'a>(&mut self) -> Result<(), WriteError<SPI::Error>> {
        let command: Vec<_> = OpCode::WakeUp.into();
        self.spi
            .transaction(&mut [
                Operation::Write(&command),
                Operation::DelayNs(TCLK_2_048M * 4),
            ])
            .map_err(WriteError::SpiTransferError)?;
        Ok(())
    }

    /// 进入待机模式
    pub fn stand_by<'a>(&mut self) -> Result<(), WriteError<SPI::Error>> {
        let command: Vec<_> = OpCode::StandBy.into();
        self.spi
            .transaction(&mut [Operation::Write(&command)])
            .map_err(WriteError::SpiTransferError)?;
        Ok(())
    }

    /// 复位器件
    ///
    /// 需要 `18` 个 tCLK 周期
    pub fn reset<'a>(&mut self) -> Result<(), WriteError<SPI::Error>> {
        let command: Vec<_> = OpCode::Reset.into();
        self.spi
            .transaction(&mut [
                Operation::Write(&command),
                Operation::DelayNs(TCLK_2_048M * 18),
            ])
            .map_err(WriteError::SpiTransferError)?;
        Ok(())
    }

    /// 启动/重新启动（同步）转换
    ///
    /// 需要 `4` 个 tCLK 周期，才可发送 `stop` 操作码
    pub fn start<'a>(&mut self) -> Result<(), WriteError<SPI::Error>> {
        let command: Vec<_> = OpCode::Start.into();
        self.spi
            .transaction(&mut [
                Operation::Write(&command),
                Operation::DelayNs(TCLK_2_048M * 4),
            ])
            .map_err(WriteError::SpiTransferError)?;
        Ok(())
    }

    /// 停止转换
    pub fn stop<'a>(&mut self) -> Result<(), WriteError<SPI::Error>> {
        let command: Vec<_> = OpCode::Stop.into();
        self.spi
            .transaction(&mut [Operation::Write(&command)])
            .map_err(WriteError::SpiTransferError)?;
        Ok(())
    }

    /// 停止连续读取数据模式
    ///
    /// 需要 `4` 个 tCLK 周期
    pub fn stop_stream<'a>(&mut self) -> Result<(), WriteError<SPI::Error>> {
        let command: Vec<_> = OpCode::SDataC.into();
        self.spi
            .transaction(&mut [
                Operation::Write(&command),
                Operation::DelayNs(TCLK_2_048M * 4),
            ])
            .map_err(WriteError::SpiTransferError)?;
        Ok(())
    }
}
