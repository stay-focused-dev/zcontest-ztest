use std::io::{self, BufWriter, Write};

const N: usize = 1 << 31;
const M: usize = 33_333_333;
const D: usize = 1_234_567_890;
const WHEEL_SIZE: usize = 30;
const FIRST_L2_PRIME: usize = 2837;
const L1_CHUNK: usize = 28_000;
const L2_CHUNK: usize = L1_CHUNK * 7;
const AHEAD: usize = 20;
const PREFETCH_OFFSET: usize = ((AHEAD + 1) * D) % N;

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

#[inline(always)]
unsafe fn prefetch(ptr: *const u8) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        std::arch::x86_64::_mm_prefetch(ptr as *const i8, std::arch::x86_64::_MM_HINT_T0);
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        std::arch::asm!(
            "prfm pldl1keep, [{p}]",
            p = in(reg) ptr,
            options(nostack, readonly)
        );
    }
}

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

fn l1_and_l2_sieve(n: usize) -> Vec<u8> {
    let small_primes: Vec<usize> = small_primes(n);
    let (l1_primes, l2_primes): (Vec<_>, Vec<_>) = small_primes.into_iter().partition(|p| *p < FIRST_L2_PRIME);

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

fn main() -> io::Result<()> {
    let out = io::stdout();
    let mut out = BufWriter::with_capacity(65_536, out.lock());

    let t_start = std::time::Instant::now();
    let sieve = l1_and_l2_sieve(N);
    let t_sieve = std::time::Instant::now();

    let is_prime = |a| {
        if a == 2 || a == 3 || a == 5 || a == 7 {
            return true;
        }

        if a % 7 == 0 {
            return false;
        }

        let r = a % WHEEL_SIZE;
        match BIT_MASK[r] {
            0 => false,
            b => (sieve[a / WHEEL_SIZE] & b) == 0,
        }
    };

    let mut a = 1;

    let sieve_ptr = sieve.as_ptr();
    {
        let mut warm = a;
        for _ in 0..AHEAD {
            warm = (warm + D) % N;
            unsafe { prefetch(sieve_ptr.add(warm / WHEEL_SIZE)) };
        }
    }

    for _ in 0..M {
        let future_a = (a + PREFETCH_OFFSET) % N;
        unsafe { prefetch(sieve_ptr.add(future_a / WHEEL_SIZE)) };

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
