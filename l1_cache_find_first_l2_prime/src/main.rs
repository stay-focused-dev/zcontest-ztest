use std::io;
use std::time::{Duration, Instant};

const N: usize = 1 << 31;
const WHEEL_SIZE: usize = 30;
const L1_CHUNK: usize = 28_000;
const L2_CHUNK: usize = L1_CHUNK * 7;
const BENCHMARKS: usize = 7;

const BIT_MASK: [u8; WHEEL_SIZE] = {
    let mut bit_mask = [0; WHEEL_SIZE];
    let residues = [1u8, 7, 11, 13, 17, 19, 23, 29];
    let mut i = 0;
    while i < residues.len() {
        bit_mask[residues[i] as usize] = 1 << i;
        i += 1;
    }
    bit_mask
};

fn naive_sieve(n: usize) -> impl Fn(usize) -> bool {
    let mut sieve: Vec<bool> = vec![false; n]; // false - prime, true - composite

    sieve[1] = true;
    for p in (3..n).step_by(2) {
        if sieve[p] {
            continue;
        }

        let step = 2 * p;
        for i in (p * p..n).step_by(step) {
            sieve[i] = true;
        }
    }

    move |a| a == 2 || (a % 2 != 0 && !sieve[a])
}

struct ResumeItem {
    b: u32,
    k: u16,
    r: u8,
    s: u8,
}

impl ResumeItem {
    fn from_prime(p: usize) -> Self {
        let k = p / WHEEL_SIZE;
        let r = p % WHEEL_SIZE;
        let b = p * p / WHEEL_SIZE;

        ResumeItem {
            b: b as u32,
            k: k as u16,
            r: r as u8,
            s: 0,
        }
    }
}

struct Resume {
    data: Vec<ResumeItem>,
}

impl Resume {
    fn from_primes(primes: &[usize]) -> Self {
        let data = primes.iter().map(|p| ResumeItem::from_prime(*p)).collect();
        Self { data }
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (u16, u8, &mut u32, &mut u8)> {
        self.data
            .iter_mut()
            .map(|item| (item.k, item.r, &mut item.b, &mut item.s))
    }
}

fn sieve_helper(sieve: &mut [u8], to: usize, resume: &mut Resume) {
    for (k, r, b_ref, s_ref) in resume.iter_mut() {
        // p = 30k + r
        let mut b = *b_ref as usize;
        let mut s = *s_ref as u8;

        if b > to {
            continue;
        }

        macro_rules! step {
            ($label:lifetime, $n:expr, $mask:expr, $step:expr, $next_s:expr) => {
                if s <= $n {
                    s = 0;
                    sieve[b] |= $mask;
                    b += $step as usize;
                    if b > to {
                        break $label $next_s;
                    }
                }
            };
        }

        *s_ref = match r {
            1 => 'outer: {
                loop {
                    step!('outer, 0,   1, 6*k,     1);
                    step!('outer, 1,   2, 4*k,     2);
                    step!('outer, 2,   4, 2*k,     3);
                    step!('outer, 3,   8, 4*k,     4);
                    step!('outer, 4,  16, 2*k,     5);
                    step!('outer, 5,  32, 4*k,     6);
                    step!('outer, 6,  64, 6*k,     7);
                    step!('outer, 7, 128, 2*k+1,   0);
                }
            }
            7 => 'outer: {
                loop {
                    step!('outer, 0,  32, 4*k+1,   1);
                    step!('outer, 1,  16, 2*k+1,   2);
                    step!('outer, 2,   1, 4*k,     3);
                    step!('outer, 3, 128, 2*k+1,   4);
                    step!('outer, 4,   8, 4*k+1,   5);
                    step!('outer, 5,   4, 6*k+1,   6);
                    step!('outer, 6,  64, 2*k+1,   7);
                    step!('outer, 7,   2, 6*k+1,   0);
                }
            }
            11 => 'outer: {
                loop {
                    step!('outer, 0,   1, 2*k,     1);
                    step!('outer, 1,  64, 4*k+2,   2);
                    step!('outer, 2,   2, 2*k,     3);
                    step!('outer, 3, 128, 4*k+2,   4);
                    step!('outer, 4,   8, 6*k+2,   5);
                    step!('outer, 5,  32, 2*k+1,   6);
                    step!('outer, 6,   4, 6*k+2,   7);
                    step!('outer, 7,  16, 4*k+2,   0);
                }
            }
            13 => 'outer: {
                loop {
                    step!('outer, 0,  32, 4*k+2,   1);
                    step!('outer, 1,   4, 2*k+1,   2);
                    step!('outer, 2,   2, 4*k+1,   3);
                    step!('outer, 3, 128, 6*k+3,   4);
                    step!('outer, 4,  16, 2*k+1,   5);
                    step!('outer, 5,   8, 6*k+3,   6);
                    step!('outer, 6,   1, 4*k+1,   7);
                    step!('outer, 7,  64, 2*k+1,   0);
                }
            }
            17 => 'outer: {
                loop {
                    step!('outer, 0,  32, 2*k+1,   1);
                    step!('outer, 1,  64, 4*k+3,   2);
                    step!('outer, 2,   1, 6*k+3,   3);
                    step!('outer, 3,   8, 2*k+1,   4);
                    step!('outer, 4,  16, 6*k+3,   5);
                    step!('outer, 5, 128, 4*k+3,   6);
                    step!('outer, 6,   2, 2*k+1,   7);
                    step!('outer, 7,   4, 4*k+2,   0);
                }
            }
            19 => 'outer: {
                loop {
                    step!('outer, 0,   1, 4*k+2,   1);
                    step!('outer, 1,  16, 6*k+4,   2);
                    step!('outer, 2,   4, 2*k+1,   3);
                    step!('outer, 3,  32, 6*k+4,   4);
                    step!('outer, 4,   8, 4*k+2,   5);
                    step!('outer, 5, 128, 2*k+2,   6);
                    step!('outer, 6,   2, 4*k+2,   7);
                    step!('outer, 7,  64, 2*k+2,   0);
                }
            }
            23 => 'outer: {
                loop {
                    step!('outer, 0,  32, 6*k+5,   1);
                    step!('outer, 1,   2, 2*k+1,   2);
                    step!('outer, 2,  64, 6*k+5,   3);
                    step!('outer, 3,   4, 4*k+3,   4);
                    step!('outer, 4,   8, 2*k+1,   5);
                    step!('outer, 5, 128, 4*k+4,   6);
                    step!('outer, 6,   1, 2*k+1,   7);
                    step!('outer, 7,  16, 4*k+3,   0);
                }
            }
            29 => 'outer: {
                loop {
                    step!('outer, 0,   1, 2*k+1,   1);
                    step!('outer, 1, 128, 6*k+6,   2);
                    step!('outer, 2,  64, 4*k+4,   3);
                    step!('outer, 3,  32, 2*k+2,   4);
                    step!('outer, 4,  16, 4*k+4,   5);
                    step!('outer, 5,   8, 2*k+2,   6);
                    step!('outer, 6,   4, 4*k+4,   7);
                    step!('outer, 7,   2, 6*k+6,   0);
                }
            }
            _ => unreachable!(),
        };
        *b_ref = b as u32;
    }
}

fn small_primes(n: usize) -> Vec<usize> {
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let is_prime = naive_sieve(sqrt_n);
    (9..sqrt_n).step_by(2).filter(|&i| is_prime(i)).collect()
}

fn l1_and_l2_sieve(n: usize, first_l2_prime: usize) -> Vec<u8> {
    let small_primes: Vec<usize> = small_primes(n);
    let (l1_primes, l2_primes): (Vec<_>, Vec<_>) = small_primes.into_iter().partition(|p| *p < first_l2_prime);

    let sieve_len = (n / WHEEL_SIZE) + 1;
    let mut sieve: Vec<u8> = vec![0; sieve_len];

    let mut l1_resume = Resume::from_primes(&l1_primes);
    let mut l2_resume = Resume::from_primes(&l2_primes);

    sieve[0] |= BIT_MASK[1];
    for l2_from in (0..sieve_len).step_by(L2_CHUNK) {
        let l2_to = (l2_from + L2_CHUNK).min(sieve_len - 1);

        for l1_from in (l2_from..l2_to).step_by(L1_CHUNK) {
            let l1_to = (l1_from + L1_CHUNK).min(l2_to);
            sieve_helper(&mut sieve, l1_to, &mut l1_resume);
        }

        sieve_helper(&mut sieve, l2_to, &mut l2_resume);
    }

    sieve
}

fn l2_only_sieve(n: usize) -> Vec<u8> {
    let small_primes: Vec<usize> = small_primes(n);

    let sieve_len = (n / WHEEL_SIZE) + 1;
    let mut sieve: Vec<u8> = vec![0; sieve_len];

    let mut resume = Resume::from_primes(&small_primes);

    sieve[0] |= BIT_MASK[1];
    for from in (0..sieve_len).step_by(L2_CHUNK) {
        let to = (from + L2_CHUNK).min(sieve_len - 1);
        sieve_helper(&mut sieve, to, &mut resume);
    }

    sieve
}

struct Measure {
    data: Vec<Duration>,
}

impl Measure {
    fn new() -> Self {
        Self {
            data: Vec::with_capacity(BENCHMARKS),
        }
    }

    fn add(&mut self, d: Duration) {
        self.data.push(d);
    }

    fn avg(&mut self) -> f64 {
        self.data.sort();
        let middle = &self.data[1..self.data.len() - 1];
        (middle.iter().sum::<Duration>() / middle.len() as u32).as_secs_f64() * 1000.0
    }
}

fn main() -> io::Result<()> {
    let small_primes = small_primes(N);

    let mut x = 0;
    // I intentionally use a sentinel value instead of Option
    // because I believe it would be more readable
    const SENTINEL: usize = 0;
    for p in std::iter::once(SENTINEL).chain(small_primes) {
        let mut m = Measure::new();

        for _ in 0..BENCHMARKS {
            let t = Instant::now();
            let sieve = if p == SENTINEL {
                l2_only_sieve(N)
            } else {
                l1_and_l2_sieve(N, p)
            };
            m.add(t.elapsed());

            x ^= sieve[p]; // prevent dead-code elimination
        }

        let avg = m.avg();
        println!("{p} - {avg:.1}");
    }

    eprintln!("finished {x}");
    Ok(())
}
