use crate::rand;

use num::BigUint;
use num::FromPrimitive;
use num::traits::{Zero, One};

/// Return a list of prime numbers in the range of [2,n]
///
/// # Arguments
///
/// * `n` - A usize specifying range in which to generate primes
pub fn sieve_of_eratosthenes(n: usize) -> std::vec::Vec<u32> {
    // Mark elements that are multiples of primes
    let mut vals = vec![true; n+1];
    for i in 2..((n as f64).sqrt() as usize) {
        if vals[i] {
            let mut j = i*i;
            while j < n+1 {
                vals[j] = false;
                j += i;
            }
        }
    }

    // Build result list, only returning elements that are not composite
    let mut res= Vec::new();
    for i in 2..(n+1) {
        if vals[i] {
            res.push(i as u32);
        }
    }
    return res;
}

/// Return a list of prime numbers in the range of [2,n]
///
/// # Arguments
///
/// * `n` - A usize specifying range in which to generate primes
pub fn sieve_of_atkin(limit: usize) -> std::vec::Vec<u32> {
    let mut res = Vec::new();
    // 2 and 3 are known to be prime
    if limit > 2 {
        res.push(2);
    }
    if limit > 3 {
        res.push(3);
    }

    // Initialise the sieve array with false values
    let mut sieve = vec![false; limit];

    /* Mark sieve[n] is true if one
       of the following is true:
    a) n = (4*x*x)+(y*y) has odd number of
       solutions, i.e., there exist
       odd number of distinct pairs (x, y)
       that satisfy the equation and
        n % 12 = 1 or n % 12 = 5.
    b) n = (3*x*x)+(y*y) has odd number of
       solutions and n % 12 = 7
    c) n = (3*x*x)-(y*y) has odd number of
       solutions, x > y and n % 12 = 11 */
    let mut x:i32 = 1;
    let limit32:i32 = limit as i32;
    while x*x < limit32 {
        let mut y = 1;
        while y*y < limit32 {
            // Main part of Sieve of Atkin
            let mut n:i32 = (4 * x * x) + (y * y);
            if n <= limit32 && (n % 12 == 1 || n % 12 == 5) {
                sieve[n as usize] ^= true;
            }

            n = (3 * x * x) + (y * y);
            if n <= limit32 && n % 12 == 7 {
                sieve[n as usize] ^= true;
            }

            n = (3 * x * x) - (y * y);
            if x > y && n <= limit32 && n % 12 == 11 {
                sieve[n as usize] ^= true;
            }
            y += 1;
        }
        x += 1;
    }

    // Mark all multiples of squares as non-prime
    let mut r = 5;
    while r*r < limit32 {
        if sieve[r as usize] {
            let mut i = r*r;
            while i < limit32 {
                sieve[i as usize] = false;
                i += r * r;
            }
        }
        r += 1;
    }

    // Print primes using sieve[]
    for i in 5..limit32 {
        if sieve[i as usize] {
            res.push(i as u32);
        }
    }

    return res;
}



pub fn _test_miller_rabin(num: &BigUint, accuracy: usize) -> bool {
    let two: &BigUint = &BigUint::from_i32(2).expect("Unable to unpack 2");
    let one: BigUint = One::one();
    let mut rng = rand::new();

    if num.modpow(&one, two) == Zero::zero() {
        return false;
    }

    let mut odd_factor: BigUint = num - one.clone();
    let mut pow_two: u64 = 1;
    while (odd_factor.modpow(&One::one(), two)) == Zero::zero() {
        odd_factor /= two;
        pow_two += 1;
    }

    let mut log2 = pow_two;
    let mut odd_copy = odd_factor.clone();
    while odd_copy > Zero::zero() {
        log2 += 1;
        odd_copy /= two;
    }

    'outer: for _ in 0..accuracy {
        let a = rng.next_bigint((log2 / 8) as usize);
        let mut x = a.modpow(&odd_factor, num);

        if x == one.clone() || x == (num - one.clone()) {
            continue 'outer;
        }

        for _ in 1..(pow_two - 1) {
            x = x.modpow(two, num);
            if x == one {
                return false;
            }
            if x == (num - one.clone()) {
                continue 'outer;
            }
        }

        return false;
    }

    true
}


/// Returns an `n`-bit prime number
pub fn gen_large_prime(n: usize) -> BigUint {
    let mut size = n;
    if n < 8 {
        size = 8;
    }

    let mut rng = rand::new();
    let mut rand_bigint = rng.next_bigint(size / 8);

    while !_test_miller_rabin(&rand_bigint, 100) {
        rand_bigint = rng.next_bigint(size / 8);
    }

    rand_bigint
}
