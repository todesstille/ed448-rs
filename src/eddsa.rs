use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

pub fn clamp(pk: &mut [u8]) {
    
    pk[0] &= 0xfc;
    pk[56] = 0;
    pk[55] |= 0x80;

}

pub fn sha3_57 (input: &[u8], mut output: &[u8]) {
    let hasher = Shake256::default();
    hasher.update(input);
    let mut reader = hasher.finalize_xof();
    reader.read(output);
}

pub fn hash_with_dom() {

}
