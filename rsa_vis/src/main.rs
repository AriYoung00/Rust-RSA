extern crate num;

mod primes;
mod test;
mod rand;
mod rsa;
mod vis;

fn main() {
    // test_sieve_of_eratosthenes(100);
    // test_sieve_of_atkin(100);
    // test_rng(10000000);
    // let img = vis::generate_rng_bitmap(512);
    // let _ = img.save("./img.bmp");
    //
    // println!("chi^2: {}", test::chi_squared_test(200, 10000));
    // rsa::do_thing();
    //
    // test_primes(640);
    rsa::test_thing();
}


fn test_sieve_of_eratosthenes(n: usize) {
    let l = primes::sieve_of_eratosthenes(n);
    for i in l {
        println!("{}", i);
    }
}

fn test_sieve_of_atkin(n: usize) {
    let l = primes::sieve_of_atkin(n);
    for i in l {
        println!("{}", i);
    }
}

fn test_rng(n: usize) {
    let mut rng = rand::new();
    let mut dist: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for _ in 1..n {
        dist[rng.next_int(0, 10) as usize] += 1;
    }

    for i in 0..10 {
        println!("{0}: {1}", i, dist[i]);
    }
}

fn test_primes(n: usize) {
    let mut i = 8;
    while i < n {
        println!("{} bit prime: {}", i, primes::gen_large_prime(i));
        i += 8;
    }
}

fn test_particular_prime(n: usize) {
    println!("{} bit prime: {}", n, primes::gen_large_prime(n));
}
