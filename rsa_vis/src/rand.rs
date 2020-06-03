use std::time::{SystemTime, UNIX_EPOCH};

/// The modulus constant from the GCC implementation of rand (2^31). We can make this more efficient
/// with binary trickery if we so choose, since it's just a power of two.
const GCC_MOD: u64 = 2147483648;
/// The multiplication constant from GCC
const GCC_MULT: u64 = 1103515245;
/// The increment constant from GCC
const GCC_INC: u64 = 12345;

/// This struct represents a random number generator using the linear congruential method (L.C.M.),
/// since RNG is ultimately a sequence. `Rng` is not accessible outside of the `rand` module,
/// creation will have two layers of abstraction.
pub struct Rng {
    /// Multiplier
    a: u64,
    /// Current value
    x: u64,
    /// Increment
    c: u64,
    /// Modulus
    m: u64,
}

impl Rng {
    /// Returns a new random number generator with the parameters specified here
    ///
    /// # Arguments
    ///     * `modulus` - The modulus value for L.C.M.
    ///     * `multiplier` - The multiplier for L.C.M.
    ///     * `increment` - The increment for L.C.M.
    ///     * `seed` - Initial value for L.C.M. sequence
    fn new(modulus: u64, multiplier: u64, increment: u64, seed: u64) -> Rng {
        let mut ret = Rng {
            a: multiplier,
            x: seed,
            c: increment,
            m: modulus
        };
        let _ = ret.next(); // So we don't just return the seed as the first value

        ret
    }

    /// Return the next random number in the sequence, normalized as a value in the range [0..1)
    pub fn next(&mut self) -> f64 {
        self.x = (self.a * self.x + self.c) % self.m;
        (self.x as f64) / (self.m as f64)
    }

    /// Return the next random number in the sequence, as an integer in the range [min..max)
    pub fn next_int(&mut self, min: u64, max: u64) -> u64 {
        let range: f64 = (max - min) as f64;

        min + ((self.next() * range) as u64)
    }
}

/// Returns a new RNG object, seeded with the explicitly provided seed
pub fn new_seed(seed: u64) -> Rng {
    Rng::new(GCC_MOD, GCC_MULT, GCC_INC, seed)
}

/// Returns a new RNG object, seeded with the current Unix time in seconds
pub fn new() -> Rng {
    let seed: u64 = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("oh darn something's wrong")
        .as_millis() as u64;

    new_seed(seed)
}
