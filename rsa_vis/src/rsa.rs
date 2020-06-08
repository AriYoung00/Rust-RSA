use num::{BigUint, BigInt, ToPrimitive, FromPrimitive};
use num::traits::{One, Zero};
use crate::rand;
use crate::primes;
use crate::num::bigint::ToBigInt;

const KEY_SIZE: usize = 1024;
const BLOCK_SIZE: usize = 4; // Block size in increments of 8 bytes

/// Return greatest common divisor of elements a and b as a BigUint
fn _gcd(a: BigUint, b: BigUint) -> BigUint {
    return if b == Zero::zero() {
        a.clone()
    } else {
        _gcd(b.clone(), a % b.clone())
    }
}

/// Return modular multiplicative inverse of a and m as a BigUint
fn _modular_multiplicative_inverse(mut a: BigUint, mut m: BigUint) -> BigUint {
    // This code adapted from GeeksForGeeks: https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
    let mut a = a.to_bigint().unwrap();
    let mut m = m.to_bigint().unwrap();

    let mut m0 = m.clone();
    let mut y: BigInt = BigInt::zero();
    let mut x: BigInt = BigInt::one();

    if m == x {
        return y.to_biguint().unwrap();
    }

    while a > BigInt::one() {
        let mut q: BigInt = a.clone() / m.clone();
        let mut t: BigInt = m.clone();

        // m is remainder now, process same as Euclid's algo
        m = a % m;
        a = t;
        t = y.clone();

        y = x - q * y.clone();
        x = t;
    }
    // Make x positive if needed
    if x < BigInt::zero() {
        x = x + m0;
    }

    return x.to_biguint().unwrap();
}

fn _gen_key(num_prime_bits: usize) -> ((BigUint, BigUint), BigUint) {
    // Algorithm adapted from https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation
    let mut rng = rand::new();
    let one: BigUint = One::one();

    // 1. Choose distinct prime numbers prime_one and prime_two
    let prime_one = primes::gen_large_prime(num_prime_bits);
    let mut prime_two = primes::gen_large_prime(num_prime_bits);
    while prime_one == prime_two {
        prime_two = primes::gen_large_prime(num_prime_bits);
    }

    // 2. Compute n = prime_one * prime_two (only needed as return val, computed below)
    // n is used as the modulus for both the public and private keys.

    // 3. Compute lambda_n = lcm(p-1, q-1). Note that lcm(a, b) = abs(a*b} / gcd(a, b).
    // Here, prime_one > 0 and prime_two > 0, so prime_one*prime_two = abs(prime_one*prime_two)
    let prod: BigUint = (prime_one.clone() - one.clone()) * (prime_two.clone() - one.clone());
    let lambda_n = prod / _gcd(prime_one.clone() - one.clone(),
                        prime_two.clone() - one.clone());

    // 4. Choose an integer e s.t. 1 < e < lambda_n, and s.t. e and lambda_n are co-prime
    let mut exponent = BigUint::from_i32(65_537).unwrap();
    assert!(one.clone() < exponent.clone() && exponent.clone() < lambda_n.clone());
    assert_eq!(_gcd(exponent.clone(), lambda_n.clone()), One::one());

    // 5. Compute d s.t. d * e ≡ 1 mod lambda_n. d is modular multiplicative inverse of e, lambda_n
    // d is the private key exponent
    let d: BigUint = _modular_multiplicative_inverse(exponent.clone(), lambda_n.clone());

    // Return tuple of (public_key, private_key)
    ((&prime_one * &prime_two, exponent), d)
}

pub fn gen_key() -> ((BigUint, BigUint), BigUint) {
    _gen_key(KEY_SIZE / 2)
}

/// Helper function, encrypts bytes contained in blocks using the given publickey, returns cipher as
/// a vector of `BigUint`
///
/// # Arguments
///     * `blocks` - Packed vector of `u32`. The encryption algorith is run on each block, resulting in a
///             corresponding output block in the returned vector
///     * `key` - Public key to encrypt with, tuple of the form (modulus, exponent) where both are BigUints
fn _encrypt_bytes(blocks: Vec<u32>, key: (BigUint, BigUint)) -> Vec<BigUint> {
    print!("Pre-Encrypt Blocks: [ ");
    for b in &blocks { print!("{:#b}, ", b); }
    println!("]");

    let mut output: Vec<BigUint> = vec![BigUint::from_i32(0).unwrap();
                                        blocks.len()];
    for (i, block) in blocks.iter().enumerate() {
        output[i] = BigUint::from_u32(block.clone()).unwrap()
            .modpow(&key.1.clone(), &key.0.clone());
    }

    output
}

/// Helper function, decrypts cipher blocks using the given private key and exponent, returning the decrypted
/// blocks as a vector of `u32`
///
/// # Arguments
///     * `cipher` - The cipher to decrypt, as a reference to a vector of BigUint encrypted blocks
///     * `privkey` - The private key to use when decrypting the given cipher
///     * `modulus` - The "modulus" component of the public key, also used for decryption
fn _decrypt_bytes(cipher: &Vec<BigUint>, privkey: BigUint, modulus: BigUint) -> Vec<u32> {
    let mut dec_blocks = vec![0_u32; cipher.len()];
    for (i, enc_block) in cipher.iter().enumerate() {
        dec_blocks[i] = enc_block.modpow(&privkey.clone(), &modulus.clone())
            .to_u32().unwrap();
    }

    print!("Post-Decrypt Blocks: [ ");
    for b in &dec_blocks { print!("{:#b}, ", b); }
    println!("]");

    dec_blocks
}

/// Helper function, packs a string into a vector of `u32` (which is the return value) to pass to encryption
/// function
///
/// # Arguments
///     * `msg` - The string to pack into `Vec<u32>`
fn _pack_string(msg: &str) -> Vec<u32> {
    let offset: usize = if (msg.len() % BLOCK_SIZE) == 0 { 0 } else { 1 };

    let mut blocks = vec![0_u32; (msg.len() / BLOCK_SIZE) + offset];
    let mut block_index = 0;
    let mut block_count = 0;
    for c in msg.chars() {
        blocks[block_index] ^= c as u32;
        block_count += 1;
        if block_count == BLOCK_SIZE {
            block_index += 1;
            block_count = 0;
        } else {
            blocks[block_index] <<= 8;
        }
    }

    blocks
}

/// Helper function, unpacks and returns a string from a vector of `u32` returned by decryption function.
///
/// # Arguments
///     * `blocks` - `Vec<u32>` to unpack characters from
fn _unpack_string(mut blocks: Vec<u32>) -> String {
    let mut res: String = String::new();
    for mut c in blocks.iter_mut().rev() {
        let mut c = *c;
        while c > 0 {
            let char = (c & 0b11111111) as u8;
            res = char::from(char).to_string().to_owned() + &res;
            // Mask to get rid of the bytes representing the character we just pulled out of this u32
            c &= 0b11111111111111111111111100000000;
            c = c.overflowing_shr(8).0;
        }
    }

    res
}

/// Encrypts string `msg` using given public key
///
/// # Arguments
///     * `msg` - String to encrypt
///     * `pubkey` - Publickey to use to encrypt `msg`, in format (modulus: `BigUint`, exponent: `BigUint')
pub fn encrypt_str(msg: &str, pubkey: (BigUint, BigUint)) -> Vec<BigUint> {
    let packed_string = _pack_string(msg);
    _encrypt_bytes(packed_string, pubkey)
}

/// Returns cipher decrypted and unpacked as string
///
/// # Arguments
///     * `cipher` - Vector of `BigUint` representing encrypted string
///     * `privkey` - The private key to use for decryption
///     * `modulus` - The modulus component of the public key, also used in decryption
pub fn decrypt_str(cipher: &Vec<BigUint>, privkey: BigUint, modulus: BigUint) -> String {
    let dec_blocks = _decrypt_bytes(cipher, privkey, modulus);
    _unpack_string(dec_blocks)
}

pub fn test_thing() {
    let (pubkey, privkey) = _gen_key(KEY_SIZE / 2);
    let cipher = encrypt_str(&"Hello world, how are you today?".to_string(), pubkey.clone());
    let dec_result = decrypt_str(&cipher, privkey, pubkey.0);

    println!("Result: {}", dec_result);
}