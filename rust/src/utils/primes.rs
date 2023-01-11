pub struct Primes {
    is_prime: Vec<bool>,
    up_to: usize,
    primes: Vec<usize>,
}

use itertools::Itertools;

impl Primes {
    pub fn new(up_to: usize) -> Primes {
        let mut primes = Primes {
            is_prime: vec![true; up_to],
            up_to,
            primes: vec![],
        };
        primes.sieve_primes();
        primes
    }

    /// Use "Sieve of Eratosthenes" to find prime numbers
    fn sieve_primes(&mut self) {
        self.is_prime[0] = false; // 0 is not prime
        self.is_prime[1] = false; // 1 is not prime
        for i in 2..self.up_to {
            if self.is_prime[i] {
                self.primes.push(i);
                // Mark all multiples as not prime (composite)
                for prime_index in (i * 2..self.up_to).step_by(i) {
                    self.is_prime[prime_index] = false;
                }
            }
        }
    }

    pub fn is_prime(&self, number: usize) -> bool {
        self.is_prime[number]
    }

    /// Get the prime factorization of this number
    pub fn factorization(&self, number: usize) -> Vec<Power> {
        assert_ne!(number, 0, "Cannot factorize 0");

        if self.is_prime(number) || number == 1 {
            return vec![Power {
                base: number,
                exponent: 1,
            }];
        }

        let mut factorization = vec![];
        for prime in self.primes.iter() {
            if number % prime == 0 {
                factorization.push(Power {
                    base: *prime,
                    exponent: 1,
                });

                let multiple = number / prime;
                if multiple > 1 {
                    factorization.extend(self.factorization(multiple));
                }
                break;
            }
        }

        // Conbine exponents for matching bases
        factorization.sort_by(|a, b| a.base.cmp(&b.base));
        let mut output = vec![factorization[0].clone()];
        for factor in factorization.iter().skip(1) {
            if factor.base == output.last().unwrap().base {
                // Combine exponents
                output.last_mut().unwrap().exponent += factor.exponent;
            } else {
                output.push(factor.clone());
            }
        }

        output
    }

    /// Get all factors for a number.
    /// For example 12's factors are [1, 2, 3, 4, 6, 12].
    pub fn all_factors(&self, number: usize) -> Vec<usize> {
        let mut output = vec![1, number];

        let mut members = vec![1];
        let factorization = self.factorization(number);
        for factor in factorization.iter() {
            for _ in 0..factor.exponent {
                members.push(factor.base);
            }
        }
        members.sort();
        log::debug!("Comboify {:?}", members);
        for len in 2..=members.len() {
            for combo in members.iter().combinations(len) {
                output.push(combo.iter().fold(1, |a, &x| a * x));
            }
        }
        output.sort();
        output.dedup();

        output
    }
}

#[derive(Debug, Clone)]
pub struct Power {
    pub base: usize,
    pub exponent: usize,
}