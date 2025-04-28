use crate::data::Address;

use thiserror_no_std::Error;

////////////////////////////////////////////////////////////////////////////////

pub trait Image {
    fn load_into_memory(&self, memory: &mut [u8; Address::DOMAIN_SIZE]);
    fn entry_point(&self) -> Address;
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum Ch8ImageError {
    #[error("image is too big")]
    TooBig,
}

#[derive(Copy, Clone)]
pub struct Ch8Image<T: AsRef<[u8]>> {
    data: T,
}

impl<T: AsRef<[u8]>> Ch8Image<T> {
    const BASE_ADDRESS: Address = Address::new(0x200);

    pub fn new(data: T) -> Result<Self, Ch8ImageError> {
        if data.as_ref().len() > 4096 {
            Err(Ch8ImageError::TooBig)
        } else {
            Ok(Self { data })
        }
    }
}

impl<T: AsRef<[u8]>> Image for Ch8Image<T> {
    fn load_into_memory(&self, memory: &mut [u8; Address::DOMAIN_SIZE]) {
        for i in 0..self.data.as_ref().len() {
            memory[i + self.entry_point().as_usize()] = *self.data.as_ref().get(i).unwrap();
        }
    }

    fn entry_point(&self) -> Address {
        Self::BASE_ADDRESS
    }
}
