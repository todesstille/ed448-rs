#![allow(warnings)]
 
mod scalar;
mod constants32;
mod bignumber;
mod karatsuba_32;
mod karatsuba_square_32;
mod extended_point;
mod decaf_combs_32;
mod decaf_wnaf_table;
pub mod goldilocks;
mod eddsa;
pub mod errors;

use goldilocks::{hex_to_private_key, ed448_derive_public};
use crate::errors::LibgoldilockErrors;

pub trait PrehashSigner<S> {
    fn sign_prehash(&self, prehash: &[u8]) -> Result<S, LibgoldilockErrors>;
}

#[derive(Debug, Clone)]
pub struct SecretKey {
    key: [u8; 57],
}

#[derive(Debug, Clone)]
pub struct VerifyingKey {
    key: [u8; 57],
}

#[derive(Debug, Clone)]
pub struct SigningKey {
    secret_key: SecretKey,
    verifying_key: VerifyingKey,
}

impl SecretKey {
    pub fn from_str(str: &str) -> Self {
    let key = hex_to_private_key(str); 
        
    Self {key}
    }
}

impl VerifyingKey {
    pub fn from_str(str: &str) -> Self {
    let key = hex_to_private_key(str); 
        
    Self {key}
    }

    pub fn as_bytes(&self) -> &[u8] {
        
        &self.key
    }
}

impl SigningKey {
    pub fn from_str(str: &str) -> Self {
        let private_key = hex_to_private_key(str);
        let public_key = ed448_derive_public(&private_key);
        let secret_key = SecretKey {key: private_key};
        let verifying_key = VerifyingKey {key: public_key};
        
        Self {secret_key, verifying_key}
    }

    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

}

