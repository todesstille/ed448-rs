pub type word = u32;
pub type sword = i32;
pub type dword  = u64;
pub type sdword = i64;

pub const lmask: word    = 0xffffffff;
pub const zeroMask: word = 0x80;
pub const allZeros: word = 0x00;
pub const allOnes: word = 0xffffffff;

pub const decafTrue: word = 0xffffffff;
pub const decafFalse: word = 0x00;


pub const nLimbs: usize      = 16;
pub const scalarLimbs: usize = 14;
pub const radix: usize       = 28;
pub const radixMask: word   = 0xfffffff as word;

pub const edwardsD: sdword = -39081;

pub type BigNumber = [word; nLimbs];
pub type serialized = [u8; fieldBytes];


const fieldBits: usize = 448;
//const edwardsD: usize  = -39081;
//const twistedD: usize  = (edwardsD) - 1;
const effD: usize      = 39082;

pub const fieldBytes: usize  = fieldBits / 8; // 56
pub const scalarSerBytes: usize = 56;
const dsaFieldBytes: usize  = 57;
const x448FieldBytes: usize = 56;
const x448FieldBits: usize  = 448;

// The size of the Goldilocks scalars, in bits.
pub const scalarBits: usize = fieldBits - 2; // 446
// The size of the Goldilocks field, in bytes.
pub const scalarBytes: usize = (scalarBits + 7) / 8; // 56

pub const wordBits: usize = 32; // 32-bits

pub const scalarWords: usize = (scalarBits + wordBits - 1) / wordBits;

pub const decafCombNumber: usize  = 0x05;
pub const decafCombTeeth: usize   = 0x05;
pub const decafCombSpacing: usize = 0x12;

pub const montgomeryFactor: word = 0xae918bc5 as word;

pub const bigZero: BigNumber = [0; nLimbs];
pub const bigOne: BigNumber  = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const bigTwo: BigNumber  = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub const ScalarQ: [word; scalarWords] = [0xab5844f3, 0x2378c292, 0x8dc58f55, 0x216cc272,
                                          0xaed63690, 0xc44edb49, 0x7cca23e9, 0xffffffff,
                                          0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
                                          0xffffffff, 0x3fffffff];
pub const ScalarR2: [word; scalarWords] = [0x049b9b60, 0xe3539257, 0xc1b195d9, 0x7af32c4b,
                                          0x88ea1859, 0x0d66de23, 0x5ee4d838, 0xae17cf72,
                                          0xa3c47c44, 0x1a9cc14b, 0xe4d070af, 0x2052bcb7,
                                          0xf823b729, 0x3402a939];
pub const ScalarZero: [word; scalarWords] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
pub const modulus: BigNumber = [0xffffff, 0xffffffff, 0xffffff, 0xffffffff, 0xffffff, 0xffffffff, 0xffffff, 0xffffffff, 0xfffffe, 0xffffffff, 0xffffff, 0xffffffff, 0xffffff, 0xffffffff, 0xffffff, 0xffffffff];