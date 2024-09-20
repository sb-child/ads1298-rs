use crate::driver::registers::access::WriteError;

pub trait Initializer<Application> {
    type SpiError;
    fn init(&mut self, application: Application) -> Result<(), InitializeError<Self::SpiError>>;
}

pub struct Application3Lead;

#[derive(Debug)]
pub enum InitializeError<SpiError> {
    WriteError {
        source: WriteError<SpiError>,
        address: u8,
        data: u8,
    },
}
