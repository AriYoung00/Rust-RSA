mod primes;
mod rand;
mod rsa;
mod vis;

fn main() {
    println!("Hello, world!");
    test_sieve_of_eratosthenes(100);
    test_sieve_of_atkin(100);
    test_rng(10000000);
    let img = vis::generate_rng_bitmap(512);
    let _ = img.save("./img.bmp");
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
    let mut dis: [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for _ in 1..n {
        dis[rng.next_int(0, 10) as usize] += 1;
    }

    for i in 0..10 {
        println!("{0}: {1}", i, dis[i]);
    }
}