use crate::constants32::{self, word, dword, sdword, scalarWords, wordBits, ScalarQ, scalarLimbs, scalarSerBytes, montgomeryFactor, ScalarR2, scalarBytes};

const scalarQ: [word; scalarWords] = constants32::ScalarQ;

pub type Scalar = [word; scalarWords];

pub fn create_zero_scalar() -> Scalar {
    let r: Scalar = [0; scalarWords];
    r
}

pub fn copy(a: &Scalar) -> Scalar {
    let mut r = create_zero_scalar();
    for i in 0..scalarWords {
        r[i] = a[i];
    }
    r
}

pub fn set(w: word) -> Scalar {
    let mut r = create_zero_scalar();
    r[0] = w;
    r
}

pub fn sub_extra(minuend: &Scalar, subtrahend: &Scalar, carry: &word) -> Scalar {
    let mut r = create_zero_scalar();
    let mut chain: sdword = 0;
    for i in 0..scalarWords {
        chain += minuend[i] as sdword - subtrahend[i] as sdword;
        r[i] = chain as word;
        chain >>= wordBits;
    }

    let mut borrow = chain + *carry as sdword;
    chain = 0;

    for i in 0..scalarWords {
        chain += r[i] as sdword + ((scalarQ[i] as sdword) & borrow);
        r[i] = chain as word;
        chain >>= wordBits;
    }
    r
}

pub fn add(a: &Scalar, b: &Scalar) -> Scalar {
    let mut r = create_zero_scalar();
    let mut chain: dword = 0;
    for i in 0..scalarWords {
        chain += a[i] as dword + b[i] as dword;
        r[i] = chain as word;
        chain >>= wordBits;
    }
    r = sub_extra(&r, &scalarQ, &(chain as word));
    r
}

pub fn sub(a: &Scalar, b: &Scalar) -> Scalar {
    let no_extra = 0 as word;
    sub_extra(a, b, &no_extra)
}

pub fn halve(mut s: Scalar) -> Scalar {
	let mask: word = u32::wrapping_sub(0, s[0] & 1);
	let mut chain: dword = 0;

	for i in 0..scalarWords {
		chain = u64::wrapping_add(chain, u64::wrapping_add((s[i] as dword), ((ScalarQ[i] & mask) as dword)));
		s[i] = (chain as word);
		chain >>= wordBits;
	}
	for i in 0..(scalarWords - 1) {
		s[i] = (s[i]>>1) | (s[i+1]<<(wordBits-1));
	}

	s[scalarWords - 1] = (s[scalarWords - 1]>>1) | ((chain<<(wordBits-1)) as word);

    s
}

pub fn montgomery_multiply(x: &Scalar, y: &Scalar) -> Scalar {
    let mut out = create_zero_scalar();
    let mut carry = 0 as word;
    for i in 0..scalarWords {
        let mut chain = 0 as dword;
        for j in 0..scalarWords {
            chain = u64::wrapping_add(u64::wrapping_add(u64::wrapping_mul((x[i] as dword), (y[j] as dword)), (out[j] as dword)), chain);
            out[j] = chain as word;
            chain >>= wordBits;
        }

        let saved = chain as word;
        let multiplicand = u32::wrapping_mul(out[0], montgomeryFactor);
        chain = 0 as dword;

        for j in 0..scalarWords {
            chain = u64::wrapping_add(u64::wrapping_add(u64::wrapping_mul((multiplicand as dword), (scalarQ[j] as dword)), out[j] as dword), chain);
            if j > 0 {
                out[j-1] = chain as word;
            }
            chain >>= wordBits;
        }
        chain = u64::wrapping_add(u64::wrapping_add((saved as dword), (carry as dword)), chain);
        out[scalarWords - 1] = chain as word;
        carry = (chain >> wordBits) as word;
    }
    out = sub_extra(&out, &ScalarQ, &carry);

    out
}

pub fn mul (x: &Scalar, y: &Scalar) -> Scalar {
    let s = montgomery_multiply(&x, &y);
    montgomery_multiply(&s, &ScalarR2)
}

pub fn decode_short(b: &[u8], size: usize) -> Scalar {
    let mut s = create_zero_scalar();
    let mut k: usize = 0;
    for i in 0..scalarLimbs {
        let mut out: word = 0;
        for j in 0..(4 as usize) {
            if k >= size {
                break;
            }
            out |= (b[k] as word) << (8 * j);
            k += 1;
        }
        s[i] = out;
    }
    s
}

pub fn decode(b: &[u8]) -> Scalar {
    let mut s = decode_short(&b, scalarBytes);

    let mut accum = 0 as sdword;
    for i in 0..scalarLimbs {
        accum += (s[i] as sdword - ScalarQ[i] as sdword);
        accum >>= wordBits;
    }

    mul(&s, &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])

}

pub fn decode_long(b: &[u8]) -> Scalar{
    let mut y = create_zero_scalar();
    let b_len = b.len();
    let mut size = b_len - (b_len % scalarSerBytes);
    if b_len == 0 {
        return y;
    }
    if size == b_len {
        size -= scalarSerBytes;
    }
    let mut res = decode_short(&b[size..b_len], b_len - size);
    if b_len == scalarSerBytes {
        res = mul(&res, &[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        return res;
    }

    while size > 0 {
        size -= scalarSerBytes;
        res = montgomery_multiply(&res, &ScalarR2);
        y = decode(&b[size..b_len]);
        res = add(&res, &y);
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{scalar::{Scalar, copy, set, add, sub, scalarQ, halve, mul}, constants32::{ScalarR2, ScalarZero}};

    use super::{montgomery_multiply, decode_long};

    #[test]
    fn test_scalar_copy() {
        let result: Scalar = [0xffb823a3, 0xc96a3c35, 0x7f8ed27d, 0x087b8fb9,
		0x1d9ac30a, 0x74d65764, 0xc0be082e, 0xa8cb0ae8,
		0xa8fa552b, 0x2aae8688, 0x2c3dc273, 0x47cf8cac,
		0x3b089f07, 0x1e63e807];
        let b = copy(&result);
        assert_eq!(result, b);
    }

    fn test_scalar_set() {
        let mut a = set(0xee);
        let mut result = [0; 14];
        result[0] = 0xee;
        assert_eq!(result, a);
    }

    #[test]
    fn test_add_scalars() {
        let a: Scalar = [0x529eec33, 0x721cf5b5, 0xc8e9c2ab, 0x7a4cf635,
		                 0x44a725bf, 0xeec492d9, 0x0cd77058, 0x00000002, 0, 0, 0, 0, 0, 0];
        let b: Scalar = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let exp: Scalar = [0x529eec34, 0x721cf5b5, 0xc8e9c2ab, 0x7a4cf635,
		                 0x44a725bf, 0xeec492d9, 0x0cd77058, 0x00000002, 0, 0, 0, 0, 0, 0]; 
        let mut result: Scalar = add(&a, &b);
        assert_eq!(result, exp);

        result = add(&b, &scalarQ);
        assert_eq!(result, b);
    }

    #[test]
    fn test_sub_scalars() {
        let mut a: Scalar = [0; 14];
        a[0] = 2;
        let mut b: Scalar = [0; 14];
        b[0] = 1;
        let mut result = sub(&a, &b);
        assert_eq!(result, b);
        result = sub(&b, &a);
        assert_eq!(result, sub(&scalarQ, &b));
    }

    #[test]
    fn test_scalar_halve() {
        let mut a: Scalar = [0; 14];
        a[0] = 0x0c;
        let mut b: Scalar = [0; 14];
        b[0] = 0x06;
        assert_eq!(b, halve(a));
    }

    #[test]
    fn test_montgomery_multiply() {
        let mut a: Scalar = [0xd013f18b, 0xa03bc31f, 0xa5586c00, 0x5269ccea, 0x80becb3f, 0x38058556, 0x736c3c5b, 0x07909887, 0x87190ede, 0x2aae8688, 0x2c3dc273, 0x47cf8cac, 0x3b089f07, 0x1e63e807];
        let b: Scalar = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut exp: Scalar = [0xf19fb32f, 0x62bc6ae6, 0xed626086, 0x0e2d81d7, 0x7a83d54b, 0x38e73799, 0x485ad3d6, 0x45399c9e, 0x824b12d9, 0x5ae842c9, 0x5ca5b606, 0x3c0978b3, 0x893b4262, 0x22c93812];
        let mut out = montgomery_multiply(&a, &b);
        assert_eq!(exp, out);

        out = montgomery_multiply(&out, &ScalarR2);
        assert_eq!(a, out);

        a = [0xd013f18b, 0xa03bc31f, 0xa5586c00, 0x5269ccea, 0x80becb3f, 0x38058556, 0x736c3c5b, 0x07909887, 0x87190ede, 0x2aae8688, 0x2c3dc273, 0x47cf8cac, 0x3b089f07, 0x1e63e807];
        out = montgomery_multiply(&a, &ScalarZero);
        assert_eq!(out, ScalarZero);

        let x: Scalar = [0xffb823a3, 0xc96a3c35, 0x7f8ed27d, 0x087b8fb9, 0x1d9ac30a, 0x74d65764, 0xc0be082e, 0xa8cb0ae8, 0xa8fa552b, 0x2aae8688, 0x2c3dc273, 0x47cf8cac, 0x3b089f07, 0x1e63e807];
        let y: Scalar = [0xd8bedc42, 0x686eb329, 0xe416b899, 0x17aa6d9b, 0x1e30b38b, 0x188c6b1a, 0xd099595b, 0xbc343bcb, 0x1adaa0e7, 0x24e8d499, 0x8e59b308, 0x0a92de2d, 0xcae1cb68, 0x16c5450a];
        exp = [0x14aec10b, 0x426d3399, 0x3f79af9e, 0xb1f67159, 0x6aa5e214, 0x33819c2b, 0x19c30a89, 0x480bdc8b, 0x7b3e1c0f, 0x5e01dfc8, 0x9414037f, 0x345954ce, 0x611e7191, 0x19381160];
        out = montgomery_multiply(&x, &y);
        assert_eq!(out, exp);
    }

    #[test]
    fn test_scalar_multiply() {
        let a: Scalar = [0xffb823a3, 0xc96a3c35, 0x7f8ed27d, 0x087b8fb9, 0x1d9ac30a, 0x74d65764, 0xc0be082e, 0xa8cb0ae8, 0xa8fa552b, 0x2aae8688, 0x2c3dc273, 0x47cf8cac, 0x3b089f07, 0x1e63e807];
        let b: Scalar = [0xd8bedc42, 0x686eb329, 0xe416b899, 0x17aa6d9b, 0x1e30b38b, 0x188c6b1a, 0xd099595b, 0xbc343bcb, 0x1adaa0e7, 0x24e8d499, 0x8e59b308, 0x0a92de2d, 0xcae1cb68, 0x16c5450a];
        let exp: Scalar = [0xa18d010a, 0x1f5b3197, 0x994c9c2b, 0x6abd26f5, 0x08a3a0e4, 0x36a14920, 0x74e9335f, 0x07bcd931, 0xf2d89c1e, 0xb9036ff6, 0x203d424b, 0xfccd61b3, 0x4ca389ed, 0x31e055c1];
        assert_eq!(mul(&a, &b), exp);
    }

    #[test]
    fn test_scalar_decode() {
        let mut b: &[u8] = &[];
        let mut x = decode_long(&b);
        assert_eq!(x, ScalarZero);

        b = &[0xf5, 0x81, 0x74, 0xd5, 0x7a, 0x33, 0x72, 0x36, 0x3c, 0x0d, 0x9f, 0xcf, 0xaa, 0x3d, 0xc1, 0x8b, 0x1e, 0xff, 0x7e, 0x89, 0xbf, 0x76, 0x78, 0x63, 0x65, 0x80, 0xd1, 0x7d, 0xd8, 0x4a, 0x87, 0x3b, 0x14, 0xb9, 0xc0, 0xe1, 0x68, 0x0b, 0xbd, 0xc8, 0x76, 0x47, 0xf3, 0xc3, 0x82, 0x90, 0x2d, 0x2f, 0x58, 0xd2, 0x75, 0x4b, 0x39, 0xbc, 0xa8, 0x74];
        let mut exp: Scalar = [0x2a1c3d02, 0x12f970e8, 0x41d97de7, 0x6a547b38, 0xdaa8c88e, 0x9f299b75, 0x01075c7b, 0x3b874ad9, 0xe1c0b914, 0xc8bd0b68, 0xc3f34776, 0x2f2d9082, 0x4b75d258, 0x34a8bc39];
        x = decode_long(&b);
        assert_eq!(x, exp);

        b = &[0xf0, 0xe4, 0x4d, 0xd4, 0x98, 0xf3, 0xad, 0x30, 0x83, 0xe1, 0xf5, 0xfc, 0xc1, 0x44, 0xed, 0x1f, 0xf5, 0xfb, 0x62, 0x5b, 0xa6, 0x21, 0x41, 0xa8, 0xde, 0x2a, 0x90, 0x23, 0x13, 0xb3, 0x1a, 0xd1, 0x41, 0x13, 0x42, 0x94, 0xdb, 0x9b, 0x0d, 0x84, 0xec, 0x43, 0x7a, 0x51, 0x5a, 0x9b, 0x85, 0xbd, 0xa1, 0xb1, 0x5e, 0xac, 0xeb, 0xe4, 0xa3, 0xb2, 0x0];
        exp = [0x7d9d5b0a, 0xe9bc6e73, 0xe16ac2d8, 0xdd13bfdc, 0xfdb68ed4, 0x1fa36b12, 0x29fbe30b, 0xd11ab314, 0x94421341, 0x840d9bdb, 0x517a43ec, 0xbd859b5a, 0xac5eb1a1, 0x32a3e4eb];
        x = decode_long(&b);
        assert_eq!(x, exp);
    }
}