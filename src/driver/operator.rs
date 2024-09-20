use embedded_hal::spi::{Operation, SpiDevice};

use crate::driver::registers::access::{ReadError, ReadFromRegister, WriteError};

use super::registers::{access::WriteToRegister, addressable::Address};

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
        let buffer = [address, data];
        self.spi
            .transaction(&mut [Operation::Write(&buffer)])
            .map_err(WriteError::SpiTransferError)?;
        log::debug!("Write {data:#04x} to the address {address:#04x} of ADS1298",);
        Ok(())
    }
}

impl<SPI: SpiDevice> ReadFromRegister<Address, u8, SPI::Error> for Operator<SPI> {
    fn read(&mut self, address: Address) -> Result<u8, ReadError<SPI::Error>> {
        let command = address | (1u8 << 7);
        let mut buffer = [command, 0xff];
        self.spi
            .transaction(&mut [Operation::TransferInPlace(&mut buffer)])
            .map_err(ReadError::SpiTransferError)?;
        Ok(buffer[1])
    }
}

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
