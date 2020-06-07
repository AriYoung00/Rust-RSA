use num::{BigUint, BigInt, ToPrimitive, FromPrimitive};
use num::traits::{One, Zero};
use crate::rand;
use crate::primes;
use crate::num::bigint::ToBigInt;
use std::borrow::Borrow;

const KEY_SIZE: usize = 1024;
const BLOCK_SIZE: usize = 4; // Block size in number of characters

fn _gcd(a: BigUint, b: BigUint) -> BigUint {
    return if b == Zero::zero() {
        a.clone()
    } else {
        _gcd(b.clone(), a % b.clone())
    }
}

fn _mod_inverse(mut a: BigUint, mut m: BigUint) -> BigUint {
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

        // Update x and y
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
    let mut rng = rand::new();
    let one: BigUint = One::one();

    let prime_one = primes::gen_large_prime(num_prime_bits);
    let mut prime_two = primes::gen_large_prime(num_prime_bits);
    while prime_one == prime_two {
        prime_two = primes::gen_large_prime(num_prime_bits);
    }

    let prod: BigUint = (prime_one.clone() - one.clone()) * (prime_two.clone() - one.clone());
    let lambda_n = prod / _gcd(prime_one.clone() - one.clone(),
                        prime_two.clone() - one.clone());

    let exponent_bits = lambda_n.bits() / 2;
    // let mut exponent = primes::gen_large_prime(exponent_bits);
    let mut exponent = BigUint::from_i32(65_537).unwrap();
    assert_eq!(_gcd(exponent.clone(), lambda_n.clone()), One::one());

    // compute d, the modular multiplicative inverse of e and lambda_n
    // let d: BigUint = _mod_inverse(exponent.clone(), lambda_n.clone());
    let d: BigUint = _mod_inverse(exponent.clone(), lambda_n.clone());

    ((&prime_one * &prime_two, exponent), d)
}

fn _encrypt_str(msg: &str, key: (BigUint, BigUint)) -> Vec<BigUint> {
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
    print!("enc: [ ");
    for b in &blocks { print!("{:#b}, ", b); }
    println!("]");

    let mut output: Vec<BigUint> = vec![BigUint::from_i32(0).unwrap();
                                        (msg.len() / BLOCK_SIZE) + offset];
    for (i, block) in blocks.iter().enumerate() {
        output[i] = BigUint::from_u32(block.clone()).unwrap()
            .modpow(&key.1.clone(), &key.0.clone());
    }

    output
}

fn _decrypt_str(cipher: Vec<BigUint>, privkey: BigUint, pubkey: BigUint) -> String {
    let mut dec_blocks = vec![0_u32; cipher.len()];
    for (i, enc_block) in cipher.iter().enumerate() {
        dec_blocks[i] = enc_block.modpow(&privkey.clone(), &pubkey.clone())
            .to_u32().unwrap();
    }

    print!("dec: [ ");
    for b in &dec_blocks { print!("{:#b}, ", b); }
    println!("]");

    let mut res: String = String::with_capacity(dec_blocks.len() * BLOCK_SIZE);
    for mut c in dec_blocks.iter_mut().rev() {
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

pub fn test_thing() {
    let (pubkey, privkey) = _gen_key(KEY_SIZE / 2);
    let cipher = _encrypt_str(&"Hello world, how are you today?".to_string(), pubkey.clone());
    let dec_result = _decrypt_str(cipher, privkey, pubkey.0);

    println!("Result: {}", dec_result);
}