use std::io::{self, BufWriter, Write};

const N: usize = 1 << 31;
const M: usize = 33_333_333;
const D: usize = 1_234_567_890;
const WHEEL_SIZE: usize = 30;
const CHUNK: usize = 130_000;
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

fn small_primes_gt(n: usize, from: usize) -> Vec<usize> {
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let is_prime = naive_sieve(sqrt_n);
    (3..sqrt_n)
        .step_by(2)
        .filter(|&i| i > from)
        .filter(|&i| is_prime(i))
        .collect()
}

fn make_presieve(primes: &[u8]) -> Vec<u8> {
    let len = primes.iter().map(|&p| p as usize).product();
    let mut sieve = vec![0u8; len];

    for p in primes {
        let p = *p as usize;
        for i in (p..len * WHEEL_SIZE).step_by(p) {
            let r = i % WHEEL_SIZE;
            if BIT_MASK[r] > 0 {
                let b = i / WHEEL_SIZE;
                sieve[b] |= BIT_MASK[r];
            }
        }
    }

    sieve
}

#[inline(always)]
fn apply_presieve4(sieve: &mut [u8], from: usize, to: usize, pre0: &[u8], pre1: &[u8], pre2: &[u8], pre3: &[u8]) {
    let last = to - from + 1;

    let len0 = pre0.len();
    let len1 = pre1.len();
    let len2 = pre2.len();
    let len3 = pre3.len();

    let mut pos0 = from % len0;
    let mut pos1 = from % len1;
    let mut pos2 = from % len2;
    let mut pos3 = from % len3;

    let mut offset = 0;
    while offset < last {
        let min_len = (last - offset)
            .min(len0 - pos0)
            .min(len1 - pos1)
            .min(len2 - pos2)
            .min(len3 - pos3);

        for i in 0..min_len {
            sieve[from + offset + i] |= pre0[pos0 + i] | pre1[pos1 + i] | pre2[pos2 + i] | pre3[pos3 + i];
        }

        offset += min_len;
        pos0 = if pos0 + min_len >= len0 { 0 } else { pos0 + min_len };
        pos1 = if pos1 + min_len >= len1 { 0 } else { pos1 + min_len };
        pos2 = if pos2 + min_len >= len2 { 0 } else { pos2 + min_len };
        pos3 = if pos3 + min_len >= len3 { 0 } else { pos3 + min_len };
    }
}

fn mark_prime(sieve: &mut [u8], p: u8) {
    let p = p as usize;
    let r = p % WHEEL_SIZE;

    if BIT_MASK[r] > 0 {
        let k = p / WHEEL_SIZE;
        sieve[k] &= !BIT_MASK[r];
    }
}

fn mark_nonprime(sieve: &mut [u8], p: u8) {
    let p = p as usize;
    let r = p % WHEEL_SIZE;

    if BIT_MASK[r] > 0 {
        let k = p / WHEEL_SIZE;
        sieve[k] |= BIT_MASK[r];
    }
}

fn sieve(n: usize) -> Vec<u8> {
    let t_start = std::time::Instant::now();

    let primes_0 = [7, 23, 37]; // 7 * 23 * 37 = 5957
    let primes_1 = [11, 19, 31]; // 11 * 19 * 31 = 6479
    let primes_2 = [13, 17, 29]; // 13 * 17 * 29 = 6409
    let primes_3 = [41, 163]; // 41 * 163 = 6683
    let primes_4 = [43, 157]; // 43 * 157 = 6751
    let primes_5 = [47, 151]; // 47 * 151 = 7097
    let primes_6 = [53, 149]; // 53 * 149 = 7897
    let primes_7 = [59, 139]; // 59 * 139 = 8201
    let primes_8 = [61, 137]; // 61 * 137 = 8357
    let primes_9 = [67, 131]; // 67 * 131 = 8777
    let primes_a = [71, 127]; // 71 * 127 = 9017
    let primes_b = [73, 113]; // 73 * 113 = 8249
    let primes_c = [79, 109]; // 79 * 109 = 8611
    let primes_d = [83, 107]; // 83 * 107 = 8881
    let primes_e = [89, 103]; // 89 * 103 = 9167
    let primes_f = [97, 101]; // 97 * 101 = 9797

    let presieve_0 = make_presieve(&primes_0);
    let presieve_1 = make_presieve(&primes_1);
    let presieve_2 = make_presieve(&primes_2);
    let presieve_3 = make_presieve(&primes_3);
    let presieve_4 = make_presieve(&primes_4);
    let presieve_5 = make_presieve(&primes_5);
    let presieve_6 = make_presieve(&primes_6);
    let presieve_7 = make_presieve(&primes_7);
    let presieve_8 = make_presieve(&primes_8);
    let presieve_9 = make_presieve(&primes_9);
    let presieve_a = make_presieve(&primes_a);
    let presieve_b = make_presieve(&primes_b);
    let presieve_c = make_presieve(&primes_c);
    let presieve_d = make_presieve(&primes_d);
    let presieve_e = make_presieve(&primes_e);
    let presieve_f = make_presieve(&primes_f);

    let all_presieve_primes: Vec<_> = primes_0
        .iter()
        .chain(primes_1.iter())
        .chain(primes_2.iter())
        .chain(primes_3.iter())
        .chain(primes_4.iter())
        .chain(primes_5.iter())
        .chain(primes_6.iter())
        .chain(primes_7.iter())
        .chain(primes_8.iter())
        .chain(primes_9.iter())
        .chain(primes_a.iter())
        .chain(primes_b.iter())
        .chain(primes_c.iter())
        .chain(primes_d.iter())
        .chain(primes_e.iter())
        .chain(primes_f.iter())
        .collect();

    let max_presieve_prime = all_presieve_primes.iter().copied().max().unwrap();

    let presieve_ms = t_start.elapsed().as_secs_f64() * 1000.0;
    eprintln!("presieve: {presieve_ms:.1} ms");

    let sieve_len = (n / WHEEL_SIZE) + 1;
    let mut sieve: Vec<u8> = vec![0; sieve_len];

    let small_primes: Vec<usize> = small_primes_gt(n, *max_presieve_prime as usize);
    let mut resume = Resume::from_primes(&small_primes);

    for from in (0..sieve_len).step_by(CHUNK) {
        let to = (from + CHUNK - 1).min(sieve_len - 1);

        apply_presieve4(&mut sieve, from, to, &presieve_0, &presieve_1, &presieve_2, &presieve_3);
        apply_presieve4(&mut sieve, from, to, &presieve_4, &presieve_5, &presieve_6, &presieve_7);
        apply_presieve4(&mut sieve, from, to, &presieve_8, &presieve_9, &presieve_a, &presieve_b);
        apply_presieve4(&mut sieve, from, to, &presieve_c, &presieve_d, &presieve_e, &presieve_f);
        sieve_helper(&mut sieve, to, &mut resume);
    }

    mark_nonprime(&mut sieve, 1);
    for p in all_presieve_primes {
        mark_prime(&mut sieve, *p);
    }

    sieve
}

fn main() -> io::Result<()> {
    let out = io::stdout();
    let mut out = BufWriter::with_capacity(65_536, out.lock());

    let t_start = std::time::Instant::now();
    let sieve = sieve(N);
    let t_sieve = std::time::Instant::now();

    let is_prime = |a| {
        if a == 2 || a == 3 || a == 5 {
            return true;
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
