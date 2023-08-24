#[derive(Debug)]
pub enum LibgoldilockErrors {
    DecodeError,
    DecodePubkeyError,
    DecodeSignatureError,
    InvalidLengthError,
    InvalidPubkeyLengthError,
    InvalidSignatureLengthError
}