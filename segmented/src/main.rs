use std::io::{self, BufWriter, Write};

const N: usize = 1 << 31;
const M: usize = 33_333_333;
const D: usize = 1_234_567_890;
const CHUNK: usize = 100_000;

fn naive_sieve(n: usize) -> impl Fn(usize) -> bool {
    let mut sieve: Vec<bool> = vec![false; n]; // false - prime, true - composite

    sieve[1] = true;
    for p in (3..n).step_by(2) {
        if sieve[p] {
            continue;
        }

        let step = 2 * p;
        for v in (p * p..n).step_by(step) {
            sieve[v] = true;
        }
    }

    move |a| a == 2 || (a % 2 != 0 && !sieve[a])
}

fn segmented_sieve(n: usize) -> impl Fn(usize) -> bool {
    // compute small primes up to sqrt(n)
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let is_prime = naive_sieve(sqrt_n);
    let small_primes: Vec<usize> = (3..sqrt_n).filter(|&i| is_prime(i)).collect();

    let mut sieve: Vec<bool> = vec![false; n];
    sieve[1] = true;

    for chunk_start in (0..n).step_by(CHUNK) {
        let chunk_end = (chunk_start + CHUNK).min(n);

        for &p in &small_primes {
            if p * p > chunk_end {
                break;
            }

            let r = chunk_start % p;
            let mut j = match r {
                0 => chunk_start,
                _ => chunk_start + p - r,
            };
            if j % 2 == 0 {
                j += p;
            }

            j = j.max(p * p);

            let step = 2 * p;
            for j in (j..chunk_end).step_by(step) {
                sieve[j] = true;
            }
        }
    }

    move |a| a == 2 || (a % 2 != 0 && !sieve[a])
}

fn main() -> io::Result<()> {
    let out = io::stdout();
    let mut out = BufWriter::with_capacity(65_536, out.lock());

    let t_start = std::time::Instant::now();
    let is_prime = segmented_sieve(N);
    let t_sieve = std::time::Instant::now();

    let mut a = 1;
    for _ in 0..M {
        let c = if is_prime(a) { b'1' } else { b'0' };
        out.write_all(&[c])?;

        a = (a + D) % N;
    }

    let t_end = std::time::Instant::now();

    let sieve_ms = (t_sieve - t_start).as_secs_f64() * 1000.0;
    let lookup_ms = (t_end - t_sieve).as_secs_f64() * 1000.0;
    let total_ms = (t_end - t_start).as_secs_f64() * 1000.0;

    eprintln!("sieve: {sieve_ms:.1} ms, lookup: {lookup_ms:.1} ms, total: {total_ms:.1} ms");

    Ok(())
}
