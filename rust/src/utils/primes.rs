/// Find prime numbers! This uses the "Sieve of Eratosthenes" to find primes.
/// Once you have primes, then you can do fun things like factorization :)
pub struct Primes {
    is_prime: Vec<bool>,
    up_to: usize,
    primes: Vec<usize>,
}

use itertools::Itertools;

impl Primes {
    /// Find all prime numbers up to a certain number.
    pub fn new(up_to: usize) -> Primes {
        let mut primes = Primes {
            is_prime: vec![true; up_to],
            up_to,
            primes: vec![],
        };
        primes.sieve_primes(); // Find the primes
        primes
    }

    /// Use "Sieve of Eratosthenes" to find prime numbers.
    /// This works by checking if a number is prime, if it is then all future
    /// multiples are not prime.
    fn sieve_primes(&mut self) {
        self.is_prime[0] = false; // 0 is not prime
        self.is_prime[1] = false; // 1 is not prime

        // Now check all future numbers for primeness
        for i in 2..self.up_to {
            if self.is_prime[i] {
                // This is prime!
                self.primes.push(i);

                // Mark all multiples as not prime (composite)
                for prime_index in (i * 2..self.up_to).step_by(i) {
                    self.is_prime[prime_index] = false;
                }
            }
        }
    }

    /// Get if a number is prime.
    pub fn is_prime(&self, number: usize) -> bool {
        self.is_prime[number]
    }

    /// Get the prime factorization of this number. This uses recursion.
    pub fn prime_factors(&self, number: usize) -> Vec<Power> {
        assert_ne!(number, 0, "Cannot factorize 0");
        assert_ne!(number, 1, "1 has no prime factors");

        if self.is_prime(number) {
            // All prime factors have been found
            return vec![Power {
                base: number,
                exponent: 1,
            }];
        }

        // Find the first prime factor for this number
        let mut prime_factors = vec![];
        for prime in self.primes.iter() {
            if number % prime == 0 {
                // This prime is a factor
                prime_factors.push(Power {
                    base: *prime,
                    exponent: 1,
                });

                // Use recursion to find the other primes inside the multiple
                let multiple = number / prime;
                prime_factors.extend(self.prime_factors(multiple));

                // Stop after one prime is found as this uses recursion to find
                // the other prime factors
                break;
            }
        }

        // Conbine exponents for matching bases, so 2^1*2^1*3^1 --> 2^2*3^1
        prime_factors.sort_by(|a, b| a.base.cmp(&b.base));
        let mut output = vec![prime_factors[0].clone()];
        for factor in prime_factors.iter().skip(1) {
            // Inspect previous number to see if matching base
            if factor.base == output.last().unwrap().base {
                // Base matches, combine exponents
                output.last_mut().unwrap().exponent += factor.exponent;
            } else {
                // This is a new base, add it to the list
                output.push(factor.clone());
            }
        }

        output
    }

    /// Get all factors for a number.
    /// For example 12's factors are [1, 2, 3, 4, 6, 12].
    pub fn all_factors(&self, number: usize) -> Vec<usize> {
        if number == 1 {
            // 1 is a special case, it's not prime factorable so just do it manually
            return vec![1];
        }

        let mut output = vec![];

        // Find the prime factors
        let prime_factors = self.prime_factors(number);

        // Remove exponents, convert 2^2*3^1 --> 2*2*3
        let mut prime_factor_bases = vec![];
        for prime_factor in prime_factors.iter() {
            for _ in 0..prime_factor.exponent {
                prime_factor_bases.push(prime_factor.base);
            }

            // Each prime factor is also a factor, ensure they are in the output
            output.push(prime_factor.base);
        }

        // Find all factors by multiple all combinations of prime factors
        // So 1*2*2*3 --> 1*2, 1*3, 2*2, 2*3, and 2*2*3 --> 2, 3, 4, 6, 12
        for len in 2..=prime_factor_bases.len() {
            for combo in prime_factor_bases.iter().combinations(len) {
                output.push(combo.iter().fold(1, |a, &x| a * x));
            }
        }

        output.push(1); // Ensure 1 is in the list
        output.sort();
        output.dedup(); // There could be duplicates from the combinations

        output
    }
}

/// Power Struct. This is base^exponent data type.
#[derive(Debug, Clone)]
pub struct Power {
    pub base: usize,
    pub exponent: usize,
}
