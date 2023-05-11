use std::result;

use crate::{scalar::{decode_long, halve, Scalar, self, encode}, extended_point::precomputed_scalar_mul, eddsa::{clamp, sha3, hash_with_dom, dsa_verify}};

pub type PrivateKey = [u8; 57];
pub type PublicKey = [u8; 57];

pub fn hex_to_private_key(hexx: &str) -> PrivateKey {
    let mut p: PrivateKey = [0;57];
    hex::decode_to_slice(hexx, &mut p).expect("Decoding failed");
    p
}

pub fn hex_to_signature(hexx: &str) -> [u8; 114] {
    let mut s: [u8; 114] = [0;114];
    hex::decode_to_slice(hexx, &mut s).expect("Decoding failed");
    s
}

pub fn private_to_secret(pk: &PrivateKey) -> PrivateKey {
    let mut sk: PrivateKey = [0; 57];
    sha3(pk, &mut sk);
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

pub fn sign_by_private(pk: &PrivateKey, message: &[u8]) -> [u8; 114] {

    let mut secret: [u8; 114] = [0; 114];
    sha3(pk, &mut secret);
    clamp(&mut secret);
    let mut sk: [u8; 57] = [0; 57];
    sk.clone_from_slice(&secret[0..57]);
    let mut sec = decode_long(&sk);
    let mut seed: [u8; 57] = [0; 57];
    seed.clone_from_slice(&secret[57..114]);
    clamp(&mut sk);
    let mut r = decode_long(&sk);
    r = halve(r);
    r = halve(r);
    let mut point = precomputed_scalar_mul(r);

    let mut nonce: [u8; 114] = [0; 114];
    let mut v1: Vec<u8> = seed.to_vec();
    v1.append(&mut message.to_vec());
    hash_with_dom(&mut v1, &mut nonce);
    let mut nonce_scalar = decode_long(&nonce);
    let mut nonce_scalar2 = nonce_scalar.clone();
    nonce_scalar2 = halve(nonce_scalar2);
    nonce_scalar2 = halve(nonce_scalar2);
    let mut nonce_point = precomputed_scalar_mul(nonce_scalar2).eddsa_like_encode();

    let mut challenge: [u8; 114] = [0; 114];
    let mut h = nonce_point.to_vec();
    h.append(&mut point.eddsa_like_encode().to_vec());
    h.append(&mut message.to_vec());
    hash_with_dom(&mut h, &mut challenge);
    
    let mut challenge_scalar = decode_long(&challenge);
    challenge_scalar = scalar::mul(&challenge_scalar, &sec);
    challenge_scalar = scalar::add(&challenge_scalar, &nonce_scalar);

    let mut result: [u8; 114] = [0; 114];
    result[0..57].copy_from_slice(&nonce_point);
    result[57..114].copy_from_slice(&encode(&challenge_scalar));

    result

}

pub fn verify(pubkey: &[u8], sig: &[u8], message: &[u8]) -> bool {
    dsa_verify(pubkey, sig, message)
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


    #[test]
    pub fn test_sign_with_private() {
        let pk = hex_to_private_key("64c2754ee8f55f285d1c6efac34345c78da28df5c31d9ae3748417e0754903004eca31389e978df148e3941de8d4c3585b6dd3669903f00bb5");
        let fox = b"The quick brown fox jumps over the lazy dog";
        let sig = sign_by_private(&pk, fox);
        println!("{:?}", sig);
        let sec = private_to_secret(&pk);
        let public = secret_to_public(&sec);
        println!("{:?}", public);
        println!("{:?}", pk);
        let sig2 = hex_to_signature("d3ffe2cffeba84f631c9e4f452c7f27023b48e679f30ad9f43b4ef0483670e25842efdd6a20ad74f2c08351e37857763c0e1b787a7a02c5c00708263b206ab852e865676b3b8ad2c86794cd2831b54064cda39e2703a4c172a1debf051e01ae981c58a577731127f2bfb7aaa3f9242572400");
        assert_eq!(sig, sig2);
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
