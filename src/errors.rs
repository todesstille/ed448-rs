use std::fmt;

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