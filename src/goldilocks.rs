use std::result;

use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

use crate::{scalar::{decode_long, halve}, extended_point::precomputed_scalar_mul, eddsa::clamp};

pub type PrivateKey = [u8; 57];
pub type PublicKey = [u8; 57];

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

pub fn private_to_secret(pk: &PrivateKey) -> PrivateKey {
    let mut sk = sha3_57(&pk);
    sk[56] |= 0x80;
    sk
}

pub fn secret_to_public(sk: &PrivateKey) -> PublicKey {
    let mut sk1 = sk.clone();
    clamp(&mut sk1);
    let mut r = decode_long(&sk1);
    r = halve(r);
    r = halve(r);
    let h = precomputed_scalar_mul(r);
    let mut p: PublicKey = h.eddsa_like_encode();

    p
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

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

    #[test]
    pub fn test_secret_to_public() {
        let pk = hex_to_private_key("1413821ed67083c855c6db4405dd4fa5fdec39e1c761be1415623c1c202c5cb5176e578830372b7e07eb1ef9cf71b19518815c4da0fd2d3594");
        let pubk = secret_to_public(&pk);
        let exp: PublicKey = [182, 21, 229, 125, 212, 209, 92, 62, 209, 50, 55, 37, 192, 186, 139, 29, 127, 110, 116, 13, 8, 224, 226, 156, 109, 63, 245, 100, 200, 150, 192, 195, 221, 40, 169, 187, 80, 101, 224, 103, 37, 200, 249, 227, 247, 194, 198, 187, 173, 73, 0, 183, 68, 126, 207, 152, 128];
        assert_eq!(pubk, exp);
    }

    // #[test]
    // pub fn test_secret_to_public_benchmarks() {
    //     let pk = hex_to_private_key("1413821ed67083c855c6db4405dd4fa5fdec39e1c761be1415623c1c202c5cb5176e578830372b7e07eb1ef9cf71b19518815c4da0fd2d3594");
    //     let before = SystemTime::now();
    //     for i in 0..10000 {
    //         let pubk = secret_to_public(&pk);
    //     }
    //     let duration = before.elapsed().unwrap();
    //     println!("{:?}", duration);
    // }


}
