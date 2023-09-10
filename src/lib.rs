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

use crate::errors::LibgoldilockErrors;

pub trait PrehashSigner<S> {
    fn sign_prehash(&self, prehash: &[u8]) -> Result<S, LibgoldilockErrors>;
}

pub struct SecretKey {
    key: [u8; 57],
}

pub struct VerifyingKey {
    key: [u8; 57],
}

