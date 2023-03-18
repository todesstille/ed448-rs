pub fn clamp(pk: &mut [u8]) {
    
    pk[0] &= 0xfc;
    pk[56] = 0;
    pk[55] |= 0x80;

}

pub fn hash_with_dom() {}
