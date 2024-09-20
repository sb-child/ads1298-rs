pub trait WriteToRegister<Register, Data, SpiError> {
    fn write(&mut self, register: Register, data: Data) -> Result<(), WriteError<SpiError>>;
}

#[derive(Debug)]
pub enum WriteError<SpiError> {
    SpiTransferError(SpiError),
}

pub trait ReadFromRegister<Register, Data, SpiError> {
    fn read(&mut self, register: Register) -> Result<Data, ReadError<SpiError>>;
}

#[derive(Debug)]
pub enum ReadError<SpiError> {
    SpiTransferError(SpiError),
}
