#![allow(warnings)] 
mod scalar;
mod constants32;
mod bignumber;
mod karatsuba_32;
mod karatsuba_square_32;
mod extended_point;
mod decaf_combs_32;
mod goldilocks;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
