use std::result;

use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};

use crate::{extended_point::eddsa_like_decode, constants32::decafTrue, scalar::{Scalar, decode_long, sub, halve}};

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

pub fn dsa_verify(pubkey: &[u8], sig: &[u8], message: &[u8]) -> bool {
    let (mut p, ok) = eddsa_like_decode(pubkey);
    if ok != decafTrue {
        panic!("Invalit public key");
    }
    let scalarZero:Scalar = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let scalarFour:Scalar = [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut p2 = p.clone();
    p2 = p2.point_scalar_mul(&scalarFour);
    let mut sig1: [u8; 57] = [0;57];
    let mut sig2: [u8; 57] = [0;57];
    for i in 0..57 {
        sig1[i] = sig[i];
        sig2[i] = sig[i + 57];
    }
    let (mut rPoint, ok) = eddsa_like_decode(&sig1); 
    if ok != decafTrue {
        panic!("Invalid signature");
    }
    rPoint = rPoint.point_scalar_mul(&scalarFour);

    let mut challenge: [u8; 114] = [0; 114];
    let mut h = sig1.to_vec();
    h.append(&mut p.eddsa_like_encode().to_vec());
    h.append(&mut message.to_vec());
    hash_with_dom(&mut h, &mut challenge);

    let mut challenge_scalar = decode_long(&challenge);
    challenge_scalar = sub(&scalarZero, &challenge_scalar);
    
    let mut responce_scalar = decode_long(&sig2);

    let pk = p2.point_double_scalamul_non_secret(&responce_scalar, &challenge_scalar); 
    // println!("{:?}", pk);
    // println!("{:?}", rPoint);
    // rPoint.eq(&pk)

    rPoint.mod_equal(&pk)

}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    #[test]
    pub fn test_dsa_verify () {
        let sig: [u8; 114] = [211, 255, 226, 207, 254, 186, 132, 246, 49, 201, 228, 244, 82, 199, 242, 112, 35, 180, 142, 103, 159, 48, 173, 159, 67, 180, 239, 4, 131, 103, 14, 37, 132, 46, 253, 214, 162, 10, 215, 79, 44, 8, 53, 30, 55, 133, 119, 99, 192, 225, 183, 135, 167, 160, 44, 92, 0, 112, 130, 99, 178, 6, 171, 133, 46, 134, 86, 118, 179, 184, 173, 44, 134, 121, 76, 210, 131, 27, 84, 6, 76, 218, 57, 226, 112, 58, 76, 23, 42, 29, 235, 240, 81, 224, 26, 233, 129, 197, 138, 87, 119, 49, 18, 127, 43, 251, 122, 170, 63, 146, 66, 87, 36, 0];
        let public: [u8; 57] = [195, 193, 156, 26, 17, 153, 66, 251, 226, 152, 245, 223, 117, 101, 207, 48, 3, 101, 6, 3, 98, 172, 67, 111, 92, 180, 137, 58, 219, 242, 33, 124, 171, 95, 19, 77, 117, 240, 95, 215, 148, 192, 172, 159, 209, 0, 41, 180, 226, 130, 237, 201, 179, 228, 32, 120, 128];
        let fox = b"The quick brown fox jumps over the lazy dog";
        let result = dsa_verify(&public, &sig, fox);
        assert_eq!(result, true);
    }
}
