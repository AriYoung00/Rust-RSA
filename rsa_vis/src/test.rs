use crate::rand;

pub fn chi_squared_test(num_test: usize, num_primes_per_test: usize) -> f64 {
    let mut rng = rand::new();
    let mut res = vec![[0 as i64; 2]; num_test];

    for i in 0..num_test {
        for _ in 0..num_primes_per_test {
            if rng.next() >= 0.5 {
                res[i][0] += 1;
            } else {
                res[i][1] += 1;
            }
        }
    }

    let mut chi_square: f64 = 0_f64;
    let expected:i64 = (num_primes_per_test / 2) as i64;
    println!("Degree of freedom: {}", 2 * (num_test - 1));

    for i in 0..num_test {
        chi_square += ((res[i][0] - expected) as f64).powi(2);
        chi_square += ((res[i][1] - expected) as f64).powi(2);
    }

    chi_square
}