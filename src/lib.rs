#![allow(warnings)]

use goldilocks::ed448_verify; 
mod scalar;
mod constants32;
mod bignumber;
mod karatsuba_32;
mod karatsuba_square_32;
mod extended_point;
mod decaf_combs_32;
mod decaf_wnaf_table;
mod goldilocks;
mod eddsa;
mod errors;