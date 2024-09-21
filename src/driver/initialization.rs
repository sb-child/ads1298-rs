use crate::driver::registers::access::WriteError;

pub trait Initializer<Application> {
    type SpiError;
    fn init(&mut self, application: Application) -> Result<(), InitializeError<Self::SpiError>>;
}

pub struct Default8Lead1x32K;

#[derive(Debug)]
pub enum InitializeError<SpiError> {
    WriteError {
        source: WriteError<SpiError>,
        address: u8,
        data: u8,
    },
    ResetError(WriteError<SpiError>),
}
