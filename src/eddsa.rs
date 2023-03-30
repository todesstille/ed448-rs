use std::result;

use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

pub fn clamp(pk: &mut [u8]) {
    
    pk[0] &= 0xfc;
    pk[56] = 0;
    pk[55] |= 0x80;

}

pub fn sha3 (input: &[u8], output: &mut [u8]) {
    let mut hasher = Shake256::default();
    hasher.update(input);
    let mut reader = hasher.finalize_xof();
    reader.read(output);
}

pub fn hash_with_dom(input: &mut Vec<u8>, output: &mut [u8]) {
    let mut sig  = b"SigEd448".to_vec();
    sig.push(0);
    sig.push(0);
    sig.append(input);
    let mut hasher = Shake256::default();
    hasher.update(&sig);
    let mut reader = hasher.finalize_xof();
    reader.read(output);
}

pub fn dsa_sign(sym: &[u8], ) {

}
