use num::{BigUint, Integer, ToPrimitive, FromPrimitive};
use num::traits::{One, Zero};
use crate::rand;
use crate::primes;

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
    let mut m0 = m.clone();
    let mut y: BigUint = BigUint::zero();
    let mut x: BigUint = BigUint::one();

    if m == x {
        return y;
    }

    while a > BigUint::one() {
        let mut q = a.clone() / m.clone();
        let mut t = m.clone();

        // m is remainder now, process same as Euclid's algo
        m = a % m;
        a = t;
        t = y.clone();

        // Update x and y
        y = x - q * y.clone();
        x = t;
    }
    // Make x positive if needed
    if x < BigUint::zero() {
        x = x + m0;
    }

    return x;
}

fn _gen_key(num_prime_bits: usize) -> ((BigUint, BigUint), BigUint) {
    let mut rng = rand::new();
    let one: BigUint = One::one();

    let prime_one = primes::gen_large_prime(num_prime_bits);
    let prime_two = primes::gen_large_prime(num_prime_bits);
    let prod: BigUint = (prime_one.clone() - one.clone()) * (prime_two.clone() - one.clone());
    let lambda_n = prod / _gcd(prime_one.clone() - one.clone(),
                        prime_two.clone() - one.clone());

    let exponent_bits = lambda_n.bits() / 2;
    let mut exponent = primes::gen_large_prime(exponent_bits);

    // compute d, the modular multiplicative inverse of e and lambda_n
    let d: BigUint = _mod_inverse(exponent.clone(), lambda_n.clone());

    ((&prime_one * &prime_two, exponent), d)
}

fn _encrypt_str(msg: &str, key: (BigUint, BigUint)) -> Vec<i32> {
    let offset = if (msg.len() % BLOCK_SIZE) == 0 { 0 } else { 1 };

    let mut blocks = vec![0_i32, (msg.len() / BLOCK_SIZE) as i32 + offset];
    let mut block_index = 0;
    let mut block_count = 0;
    for c in msg.chars() {
        blocks[block_index] += (c as i32) - 96;
        block_count += 1;
        if block_count == BLOCK_SIZE {
            block_index += 1;
            block_count = 0;
        } else {
            blocks[block_index] *= 10;
        }
    }

    let mut output = vec![0_i32, (msg.len() / BLOCK_SIZE) as i32 + offset];
    for (i, block) in blocks.iter().enumerate() {
        output[i] = BigUint::from_i32(block.clone()).unwrap()
            .modpow(&key.0.clone(), &key.1.clone())
            .to_i32()
            .unwrap();
    }

    output
}

// fn _decrypt_str(msg: &str, key: (BigUint, BigUint)) -> String {
//
// }

pub fn test_thing() {
    let (n, e) = _gen_key(KEY_SIZE / 2);
    println!("(n: {}, e: {})", n.0, n.1);
}