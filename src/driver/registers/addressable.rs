pub type Address = u8;

pub trait Addressable {
    fn get_address(&self) -> Address;
}

impl Addressable for Address {
    #[inline]
    fn get_address(&self) -> Address {
        *self
    }
}
