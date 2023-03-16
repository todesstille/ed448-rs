use std::result;

use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

pub type PrivateKey = [u8; 57];

pub fn hex_to_private_key(hexx: &str) -> PrivateKey {
    let mut p: PrivateKey = [0;57];
    hex::decode_to_slice(hexx, &mut p).expect("Decoding failed");
    p
}

pub fn sha3_57 (pk: &PrivateKey) -> [u8; 57] {
    let mut hasher = Shake256::default();
    hasher.update(pk);
    let mut reader = hasher.finalize_xof();
    let mut result = [0u8; 57];
    reader.read(&mut result);
    result
}

pub fn clamp(pk: &mut [u8]) {
    
    pk[0] &= 0xfc;
    pk[56] = 0;
    pk[55] |= 0x80;
}

pub fn private_to_secret(pk: &PrivateKey) -> PrivateKey {
    let mut sk = sha3_57(&pk);
    sk[56] |= 0x80;
    sk
}

pub fn secret_to_public(sk: &PrivateKey) {
    let mut sk1 = sk.clone();
    clamp(&mut sk1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_decode_hex () {
        let pk = hex_to_private_key("a8ea212cc24ae0fd029a97b64be540885af0e1b7dc9faf4a591742850c4377f857ae9a8f87df1de98e397a5867dd6f20211ef3f234ae71bc56");
        let exp: PrivateKey = [168, 234, 33, 44, 194, 74, 224, 253, 2, 154, 151, 182, 75, 229, 64, 136, 90, 240, 225, 183, 220, 159, 175, 74, 89, 23, 66, 133, 12, 67, 119, 248, 87, 174, 154, 143, 135, 223, 29, 233, 142, 57, 122, 88, 103, 221, 111, 32, 33, 30, 243, 242, 52, 174, 113, 188, 86];
        assert_eq!(pk, exp);
    }

    #[test]
    pub fn test_sha3_57() {
        let mut pk = hex_to_private_key("a8ea212cc24ae0fd029a97b64be540885af0e1b7dc9faf4a591742850c4377f857ae9a8f87df1de98e397a5867dd6f20211ef3f234ae71bc56");
        pk = sha3_57(&pk);
        let exp: [u8; 57] = [20, 19, 130, 30, 214, 112, 131, 200, 85, 198, 219, 68, 5, 221, 79, 165, 253, 236, 57, 225, 199, 97, 190, 20, 21, 98, 60, 28, 32, 44, 92, 181, 23, 110, 87, 136, 48, 55, 43, 126, 7, 235, 30, 249, 207, 113, 177, 149, 24, 129, 92, 77, 160, 253, 45, 53, 148];
        assert_eq!(pk, exp);
    }

    #[test]
    pub fn test_private_to_secret() {
        let pk = hex_to_private_key("a8ea212cc24ae0fd029a97b64be540885af0e1b7dc9faf4a591742850c4377f857ae9a8f87df1de98e397a5867dd6f20211ef3f234ae71bc56");
        let exp = hex_to_private_key("1413821ed67083c855c6db4405dd4fa5fdec39e1c761be1415623c1c202c5cb5176e578830372b7e07eb1ef9cf71b19518815c4da0fd2d3594");
        let sk = private_to_secret(&pk);        
        assert_eq!(sk, exp);
    }

}
