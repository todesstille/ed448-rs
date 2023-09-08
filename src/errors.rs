use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum LibgoldilockErrors {
    DecodeError,
    DecodePubkeyError,
    DecodeSignatureError,
    InvalidLengthError,
    InvalidPubkeyLengthError,
    InvalidSignatureLengthError
}

impl fmt::Display for LibgoldilockErrors {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        todo!() 
    }
}

impl Error for LibgoldilockErrors {}

pub trait PrehashSigner<S> {
    fn sign_prehash(&self, prehash: &[u8]) -> Result<S, LibgoldilockErrors>;
}
