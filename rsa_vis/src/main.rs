mod primes;
mod rand;
mod rsa;

fn main() {
    println!("Hello, world!");
    test_sieve_of_eratosthenes(100);
}

fn test_sieve_of_eratosthenes(n: usize) {
    let l = primes::sieve_of_eratosthenes(n);
    for i in l {
        println!("{}", i);
    }
}