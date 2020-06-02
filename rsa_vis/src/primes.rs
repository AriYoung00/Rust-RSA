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