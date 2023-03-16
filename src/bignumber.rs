use crate::{constants32::{self, wordBits, nLimbs, fieldBytes, word, dword, radixMask, radix, sdword, lmask, BigNumber, serialized}, karatsuba_32::karatsuba_mul, karatsuba_square_32::karatsuba_square};

pub fn create_zero_bignumber() -> BigNumber {
	let n = [0; nLimbs];
	n
}

pub fn is_zero_mask(n: word) -> word {
    let mut nn = n as dword;
    nn = u64::wrapping_sub(nn, 1);
    (nn >> wordBits) as word
}

fn bias(mut n: BigNumber, b: word) -> BigNumber {
    let co1 = radixMask * b;
    let co2 = co1 - b;

    n[0] = u32::wrapping_add(n[0], co1);
    n[1] = u32::wrapping_add(n[1],co1);
    n[2] = u32::wrapping_add(n[2],co1);
    n[3] = u32::wrapping_add(n[3],co1);
    n[4] = u32::wrapping_add(n[4],co1);
    n[5] = u32::wrapping_add(n[5],co1);
    n[6] = u32::wrapping_add(n[6],co1);
    n[7] = u32::wrapping_add(n[7],co1);

    n[8] = u32::wrapping_add(n[8],co2);
    n[9] = u32::wrapping_add(n[9],co1);
    n[10] = u32::wrapping_add(n[10],co1);
    n[11] = u32::wrapping_add(n[11],co1);
    n[12] = u32::wrapping_add(n[12],co1);
    n[13] = u32::wrapping_add(n[13],co1);
    n[14] = u32::wrapping_add(n[14],co1);
    n[15] = u32::wrapping_add(n[15],co1);
    n
    
}

pub fn constant_time_greater_or_equal_p(n: &BigNumber) -> word {
	let mut ge = lmask;

	for i in 0..4 {
		ge &= n[i]
	}

	ge = (ge & (n[4] + 1)) | is_zero_mask((n[4]^radixMask) as word);

	for i in 5..8 {
		ge &= n[i]
	}

	!is_zero_mask((ge ^ radixMask) as word)
}

pub fn neg_raw(x: &BigNumber) -> BigNumber {
    let mut n = create_zero_bignumber();
	n[0] = u32::wrapping_sub(0, x[0]);
	n[1] = u32::wrapping_sub(0, x[1]);
	n[2] = u32::wrapping_sub(0, x[2]);
	n[3] = u32::wrapping_sub(0, x[3]);
	n[4] = u32::wrapping_sub(0, x[4]);
	n[5] = u32::wrapping_sub(0, x[5]);
	n[6] = u32::wrapping_sub(0, x[6]);
	n[7] = u32::wrapping_sub(0, x[7]);
	n[8] = u32::wrapping_sub(0, x[8]);
	n[9] = u32::wrapping_sub(0, x[9]);
	n[10] = u32::wrapping_sub(0, x[10]);
	n[11] = u32::wrapping_sub(0, x[11]);
	n[12] = u32::wrapping_sub(0, x[12]);
	n[13] = u32::wrapping_sub(0, x[13]);
	n[14] = u32::wrapping_sub(0, x[14]);
	n[15] = u32::wrapping_sub(0, x[15]);

	n
}

pub fn neg(x: &BigNumber) -> BigNumber {
	weak_reduce(bias(neg_raw(&x), 2))
}

pub fn constant_time_select(x: &BigNumber, y: &BigNumber, first: &word) -> BigNumber {
	let mut a = x.clone();
	let mut b = y.clone();
	conditional_swap(&mut b, &mut a, &first);
	b
}

pub fn conditional_negate(n: &BigNumber, negate: &word) -> BigNumber {
	constant_time_select(&neg(&n), &n, &negate)
}

pub fn add_raw (x: &BigNumber, y: &BigNumber) -> BigNumber {
    let mut n = create_zero_bignumber();
	n[0] = u32::wrapping_add(x[0], y[0]);
	n[1] = u32::wrapping_add(x[1], y[1]);
	n[2] = u32::wrapping_add(x[2], y[2]);
	n[3] = u32::wrapping_add(x[3], y[3]);
	n[4] = u32::wrapping_add(x[4], y[4]);
	n[5] = u32::wrapping_add(x[5], y[5]);
	n[6] = u32::wrapping_add(x[6], y[6]);
	n[7] = u32::wrapping_add(x[7], y[7]);
	n[8] = u32::wrapping_add(x[8], y[8]);
	n[9] = u32::wrapping_add(x[9], y[9]);
	n[10] = u32::wrapping_add(x[10], y[10]);
	n[11] = u32::wrapping_add(x[11], y[11]);
	n[12] = u32::wrapping_add(x[12], y[12]);
	n[13] = u32::wrapping_add(x[13], y[13]);
	n[14] = u32::wrapping_add(x[14], y[14]);
	n[15] = u32::wrapping_add(x[15], y[15]);

	return n
}

pub fn add(x: &BigNumber, y: &BigNumber) -> BigNumber {
	weak_reduce(add_raw(&x, &y))
}

pub fn sub_raw (x: &BigNumber, y: &BigNumber) -> BigNumber {
    let mut n = create_zero_bignumber();
	n[0] = u32::wrapping_sub(x[0], y[0]);
	n[1] = u32::wrapping_sub(x[1], y[1]);
	n[2] = u32::wrapping_sub(x[2], y[2]);
	n[3] = u32::wrapping_sub(x[3], y[3]);
	n[4] = u32::wrapping_sub(x[4], y[4]);
	n[5] = u32::wrapping_sub(x[5], y[5]);
	n[6] = u32::wrapping_sub(x[6], y[6]);
	n[7] = u32::wrapping_sub(x[7], y[7]);
	n[8] = u32::wrapping_sub(x[8], y[8]);
	n[9] = u32::wrapping_sub(x[9], y[9]);
	n[10] = u32::wrapping_sub(x[10], y[10]);
	n[11] = u32::wrapping_sub(x[11], y[11]);
	n[12] = u32::wrapping_sub(x[12], y[12]);
	n[13] = u32::wrapping_sub(x[13], y[13]);
	n[14] = u32::wrapping_sub(x[14], y[14]);
	n[15] = u32::wrapping_sub(x[15], y[15]);

	return n
}

pub fn sub(x: &BigNumber, y: &BigNumber) -> BigNumber {
	weak_reduce(bias(sub_raw(&x, &y), 2))
}

pub fn sub_x_bias(x: &BigNumber, y: &BigNumber, amt: &word) -> BigNumber {
	weak_reduce(bias(sub_raw(&x, &y), *amt))
}

pub fn square(x: &BigNumber) -> BigNumber {
	karatsuba_square(x)
}

pub fn mul(x: &BigNumber, y: &BigNumber) -> BigNumber {
	karatsuba_mul(&x,&y)
}

pub fn conditional_swap(n: &mut BigNumber, x: &mut BigNumber, swap: &word) {
	for i in 0..nLimbs {
		let s = (x[i] ^ n[i]) & swap;
		x[i] ^= s;
		n[i] ^= s;
	}
}

pub fn weak_reduce(mut n: BigNumber) -> BigNumber {
	let mut tmp = ((n[nLimbs-1] as dword) >> radix) as word;
    n[nLimbs/2] = u32::wrapping_add(n[nLimbs/2], tmp);

    n[15] = u32::wrapping_add((n[15] & radixMask), (n[14] >> radix));
	n[14] = u32::wrapping_add((n[14] & radixMask), (n[13] >> radix));
	n[13] = u32::wrapping_add((n[13] & radixMask), (n[12] >> radix));
	n[12] = u32::wrapping_add((n[12] & radixMask), (n[11] >> radix));
	n[11] = u32::wrapping_add((n[11] & radixMask), (n[10] >> radix));
	n[10] = u32::wrapping_add((n[10] & radixMask), (n[9] >> radix));
	n[9] = u32::wrapping_add((n[9] & radixMask), (n[8] >> radix));
	n[8] = u32::wrapping_add((n[8] & radixMask), (n[7] >> radix));
	n[7] = u32::wrapping_add((n[7] & radixMask), (n[6] >> radix));
	n[6] = u32::wrapping_add((n[6] & radixMask), (n[5] >> radix));
	n[5] = u32::wrapping_add((n[5] & radixMask), (n[4] >> radix));
	n[4] = u32::wrapping_add((n[4] & radixMask), (n[3] >> radix));
	n[3] = u32::wrapping_add((n[3] & radixMask), (n[2] >> radix));
	n[2] = u32::wrapping_add((n[2] & radixMask), (n[1] >> radix));
	n[1] = u32::wrapping_add((n[1] & radixMask), (n[0] >> radix));
	n[0] = u32::wrapping_add((n[0] & radixMask), tmp);

    n
}

pub fn strong_reduce(mut n: BigNumber) -> BigNumber {
    n = weak_reduce(n);

    let mut scarry = 0 as sdword;
    scarry += i64::wrapping_sub( n[0] as sdword, 0xfffffff);
	n[0] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[1] as sdword, 0xfffffff);
	n[1] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[2] as sdword, 0xfffffff);
	n[2] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[3] as sdword, 0xfffffff);
	n[3] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[4] as sdword, 0xfffffff);
	n[4] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[5] as sdword, 0xfffffff);
	n[5] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[6] as sdword, 0xfffffff);
	n[6] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[7] as sdword, 0xfffffff);
	n[7] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[8] as sdword, 0xffffffe);
	n[8] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[9] as sdword, 0xfffffff);
	n[9] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[10] as sdword, 0xfffffff);
	n[10] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[11] as sdword, 0xfffffff);
	n[11] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[12] as sdword, 0xfffffff);
	n[12] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[13] as sdword, 0xfffffff);
	n[13] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[14] as sdword, 0xfffffff);
	n[14] = scarry as word & radixMask;
	scarry >>= 28;

	scarry += i64::wrapping_sub(n[15] as sdword, 0xfffffff);
	n[15] = scarry as word & radixMask;
	scarry >>= 28;

	let scarryMask = (scarry as word) & (radixMask as word);
	let mut carry = 0 as dword;
	let m = scarryMask as dword;

	carry += u64::wrapping_add(n[0] as dword, m);
	n[0] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[1] as dword, m);
	n[1] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[2] as dword, m);
	n[2] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[3] as dword, m);
	n[3] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[4] as dword, m);
	n[4] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[5] as dword, m);
	n[5] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[6] as dword, m);
	n[6] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[7] as dword, m);
	n[7] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[8] as dword, (m & (0xfffffffffffffffe as dword)));
	n[8] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[9] as dword, m);
	n[9] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[10] as dword, m);
	n[10] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[11] as dword, m);
	n[11] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[12] as dword, m);
	n[12] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[13] as dword, m);
	n[13] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[14] as dword, m);
	n[14] = carry as word & radixMask;
	carry >>= 28;

	carry += u64::wrapping_add( n[15] as dword, m);
	n[15] = carry as word & radixMask;
    
    n
}

pub fn deserialize_return_mask(inp: serialized) -> (BigNumber, word) {
	let mut n = create_zero_bignumber();

	for i in 0..8 {
		let mut out = 0 as dword;
		for j in 0..7 {
			out |= (inp[7*i+j] as dword) << (8 * j)
		}

		n[2*i] = (out as word) & radixMask;
		n[2*i+1] = (out >> 28) as word;
	}

	(n, constant_time_greater_or_equal_p(&n))
}

pub fn deserialize(inp: serialized) -> (BigNumber, bool) {
	let (n, mask) = deserialize_return_mask(inp);
	let ok = (mask == lmask);
	(n, ok)
}

pub fn must_deserialize(inp: serialized) -> BigNumber {
	let (n, ok) = deserialize(inp);
	if !ok {
		panic!("Failed to deserialize");
	}
    
    n
}

#[cfg(test)]
mod tests {
    use crate::{constants32::{fieldBytes, bigOne, bigZero}};

    use super::*;

    #[test]
    pub fn test_deserialize () {
        let mut ser = [0; fieldBytes];
        ser[0] = 1;
        let (mut n, mut ok) = deserialize(ser);
    
        assert_eq!(n, bigOne);
        assert_eq!(ok, true);
    
        ser = [0xf5, 0x81, 0x74, 0xd5, 0x7a, 0x33, 0x72,
            0x36, 0x3c, 0x0d, 0x9f, 0xcf, 0xaa, 0x3d,
            0xc1, 0x8b, 0x1e, 0xff, 0x7e, 0x89, 0xbf,
            0x76, 0x78, 0x63, 0x65, 0x80, 0xd1, 0x7d,
            0xd8, 0x4a, 0x87, 0x3b, 0x14, 0xb9, 0xc0,
            0xe1, 0x68, 0x0b, 0xbd, 0xc8, 0x76, 0x47,
            0xf3, 0xc3, 0x82, 0x90, 0x2d, 0x2f, 0x58,
            0xd2, 0x75, 0x4b, 0x39, 0xbc, 0xa8, 0x74];
    
        (n, ok) = deserialize(ser);
        assert_eq!(ok, true);
        assert_eq!(n, [0x57481f5, 0x72337ad, 0xf0d3c36, 0x3daacf9,
            0xf1e8bc1, 0xbf897ef, 0x5637876, 0x7dd1806,
            0xb874ad8, 0xc0b9143, 0xd0b68e1, 0x4776c8b,
            0x082c3f3, 0x582f2d9, 0x94b75d2, 0x74a8bc3]);
    
        ser = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

        (n, ok) = deserialize(ser);
        assert_eq!(ok, false);
        assert_eq!(n,[
                0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff,
                0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff,
                0xffffffe, 0xfffffff, 0xfffffff, 0xfffffff,
                0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff]);

        }

        #[test]
        pub fn test_strong_reduce() {
            let (mut n, _) = deserialize([
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        
            n = strong_reduce(n);
            assert_eq!(n, bigZero);
        
            n = must_deserialize([
                0xf5, 0x81, 0x74, 0xd5, 0x7a, 0x33, 0x72,
                0x36, 0x3c, 0x0d, 0x9f, 0xcf, 0xaa, 0x3d,
                0xc1, 0x8b, 0x1e, 0xff, 0x7e, 0x89, 0xbf,
                0x76, 0x78, 0x63, 0x65, 0x80, 0xd1, 0x7d,
                0xd8, 0x4a, 0x87, 0x3b, 0x14, 0xb9, 0xc0,
                0xe1, 0x68, 0x0b, 0xbd, 0xc8, 0x76, 0x47,
                0xf3, 0xc3, 0x82, 0x90, 0x2d, 0x2f, 0x58,
                0xd2, 0x75, 0x4b, 0x39, 0xbc, 0xa8, 0x74]);
        
            n = strong_reduce(n);
        
            assert_eq!(n, must_deserialize([
                0xf5, 0x81, 0x74, 0xd5, 0x7a, 0x33, 0x72,
                0x36, 0x3c, 0x0d, 0x9f, 0xcf, 0xaa, 0x3d,
                0xc1, 0x8b, 0x1e, 0xff, 0x7e, 0x89, 0xbf,
                0x76, 0x78, 0x63, 0x65, 0x80, 0xd1, 0x7d,
                0xd8, 0x4a, 0x87, 0x3b, 0x14, 0xb9, 0xc0,
                0xe1, 0x68, 0x0b, 0xbd, 0xc8, 0x76, 0x47,
                0xf3, 0xc3, 0x82, 0x90, 0x2d, 0x2f, 0x58,
                0xd2, 0x75, 0x4b, 0x39, 0xbc, 0xa8, 0x74
            ]));
        }

        #[test]
        fn test_subtraction() {
            let mut x = must_deserialize([0xda, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            let mut y = must_deserialize([0x83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            let mut exp = must_deserialize([0x57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(strong_reduce(sub(&x, &y)), exp);

			x = must_deserialize([0, 0, 0, 0xf1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
			y = must_deserialize([0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            exp = must_deserialize([0xff, 0xff, 0xff, 0xf0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(strong_reduce(sub(&x, &y)), exp);
        }

		#[test]
		fn test_addition() {
            let mut x = must_deserialize([0x57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            let mut y = must_deserialize([0x83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            let mut exp = must_deserialize([0xda, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(add(&x, &y), exp);

			exp = must_deserialize([0, 0, 0, 0xf1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
			y = must_deserialize([0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            x = must_deserialize([0xff, 0xff, 0xff, 0xf0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            assert_eq!(add(&x, &y), exp);
		}


		#[test]
		fn test_sub_x_bias() {
			let x = must_deserialize([0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
			let y = must_deserialize([0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
			let exp = [0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xffffffe, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff, 0xfffffff];
			assert_eq!(exp, sub_x_bias(&x, &y, &(2 as word)));
		}

		#[test]
		fn test_conditional_swap() {
			let x = must_deserialize([0xe6, 0xf5, 0xb8, 0xae, 0x49, 0xce, 0xf7, 0x79, 0xe5, 0x77, 0xdc, 0x29, 0x82, 0x4e, 0xff, 0x45, 0x3f, 0x1c, 0x41, 0x06, 0x03, 0x00, 0x88, 0x11, 0x5e, 0xa4, 0x9b, 0x4e, 0xe8, 0x4a, 0x7b, 0x7c, 0xdf, 0xe0, 0x6e, 0x0d, 0x62, 0x2f, 0xc5, 0x5c, 0x7c, 0x55, 0x9a, 0xb1, 0xf6, 0xc3, 0xea, 0x32, 0x57, 0xc0, 0x79, 0x79, 0x80, 0x90, 0x26, 0xde]);
			let y = must_deserialize([0x19, 0x0a, 0x47, 0x51, 0xb6, 0x31, 0x08, 0x86, 0x1a, 0x88, 0x23, 0xd6, 0x7d, 0xb1, 0x00, 0xba, 0xc0, 0xe3, 0xbe, 0xf9, 0xfc, 0xff, 0x77, 0xee, 0xa1, 0x5b, 0x64, 0xb0, 0x17, 0xb5, 0x84, 0x83, 0x20, 0x1f, 0x91, 0xf2, 0x9d, 0xd0, 0x3a, 0xa3, 0x83, 0xaa, 0x65, 0x4e, 0x09, 0x3c, 0x15, 0xcd, 0xa8, 0x3f, 0x86, 0x86, 0x7f, 0x6f, 0xd9, 0x21]);
			let mut a = must_deserialize([0xe6, 0xf5, 0xb8, 0xae, 0x49, 0xce, 0xf7, 0x79, 0xe5, 0x77, 0xdc, 0x29, 0x82, 0x4e, 0xff, 0x45, 0x3f, 0x1c, 0x41, 0x06, 0x03, 0x00, 0x88, 0x11, 0x5e, 0xa4, 0x9b, 0x4e, 0xe8, 0x4a, 0x7b, 0x7c, 0xdf, 0xe0, 0x6e, 0x0d, 0x62, 0x2f, 0xc5, 0x5c, 0x7c, 0x55, 0x9a, 0xb1, 0xf6, 0xc3, 0xea, 0x32, 0x57, 0xc0, 0x79, 0x79, 0x80, 0x90, 0x26, 0xde]);
			let mut b = must_deserialize([0x19, 0x0a, 0x47, 0x51, 0xb6, 0x31, 0x08, 0x86, 0x1a, 0x88, 0x23, 0xd6, 0x7d, 0xb1, 0x00, 0xba, 0xc0, 0xe3, 0xbe, 0xf9, 0xfc, 0xff, 0x77, 0xee, 0xa1, 0x5b, 0x64, 0xb0, 0x17, 0xb5, 0x84, 0x83, 0x20, 0x1f, 0x91, 0xf2, 0x9d, 0xd0, 0x3a, 0xa3, 0x83, 0xaa, 0x65, 0x4e, 0x09, 0x3c, 0x15, 0xcd, 0xa8, 0x3f, 0x86, 0x86, 0x7f, 0x6f, 0xd9, 0x21]);
			conditional_swap(&mut a, &mut b, &lmask);
			assert_eq!(a, y);
			assert_eq!(b, x);
			conditional_swap(&mut a, &mut b, &0);
			assert_eq!(a, y);
			assert_eq!(b, x);
		}

}
    

