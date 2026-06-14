use std::time::Instant;

fn main() {
    let limit = 100_000_000;
    println!("Calculating primes up to {}...", limit);

    let start_time = Instant::now();

    // Vec<bool> in Rust is byte-aligned (1 byte per bool)
    // This uses more memory than C++'s vector<bool> but is significantly faster to access
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;
    while p * p <= limit {
        if is_prime[p] {
            let mut i = p * p;
            while i <= limit {
                is_prime[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    let mut prime_count = 0;
    for p in 2..=limit {
        if is_prime[p] {
            prime_count += 1;
        }
    }

    let duration = start_time.elapsed();

    println!("Sieve complete!");
    println!("Primes found: {}", prime_count);
    println!("Rust Prime Sieve Time: {:.4} seconds", duration.as_secs_f64());
}
