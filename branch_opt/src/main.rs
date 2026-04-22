use std::io::{self, BufWriter, Write};

const N: usize = 1 << 31;
const M: usize = 33_333_333;
const D: usize = 1_234_567_890;
const WHEEL_SIZE: usize = 30;
const L1_CHUNK: usize = 32 * 1024;
const L2_CHUNK: usize = 256 * 1024;
const TPRIME: usize = 4999;
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

    fn iter_mut(&mut self) -> impl Iterator<Item = (usize, u8, &mut u32, &mut u8)> {
        self.data
            .iter_mut()
            .map(|item| (item.k as usize, item.r, &mut item.b, &mut item.s))
    }
}

fn sieve_helper(sieve: &mut [u8], to: usize, resume: &mut Resume) {
    debug_assert!(to < sieve.len(), "to must be < sieve.len()");

    for (k, r, b_ref, s_ref) in resume.iter_mut() {
        // p = 30k + r
        let mut b = *b_ref as usize;
        let mut s = *s_ref as u8;

        if b > to {
            continue;
        }

        macro_rules! prestep {
            ($label:lifetime, $step:expr, $mask:expr, $jump:expr) => {
                if s == $step {
                    // can't overflow because b <= to < sieve.len()
                    unsafe { *sieve.get_unchecked_mut(b) |= $mask };
                    b += $jump;
                    s = (s + 1) & 7;
                    if b > to {
                        break $label s;
                    }
                }
            }
        }

        macro_rules! poststep {
            ($label:lifetime, $next_step:expr, $mask:expr, $jump:expr) => {
                if b + $jump > to {
                    b += $jump;
                    break $label $next_step;
                }
                // can't overflow because b + $jump <= to < sieve.len()
                unsafe { *sieve.get_unchecked_mut(b + $jump) |= $mask };
            }
        }

        macro_rules! cycle {
            ($label:lifetime, $j1:expr, $j2:expr, $j3:expr, $j4:expr, $j5:expr, $j6:expr, $j7:expr, $j8:expr, $m0:expr, $m1:expr, $m2:expr, $m3:expr, $m4:expr, $m5:expr, $m6:expr, $m7:expr) => {{
                prestep!($label, 1, $m1, $j2 - $j1);
                prestep!($label, 2, $m2, $j3 - $j2);
                prestep!($label, 3, $m3, $j4 - $j3);
                prestep!($label, 4, $m4, $j5 - $j4);
                prestep!($label, 5, $m5, $j6 - $j5);
                prestep!($label, 6, $m6, $j7 - $j6);
                prestep!($label, 7, $m7, $j8 - $j7);

                let limit = to - $j7;
                while b <= limit {
                    // can't overflow because b + $j7 <= to < sieve.len()
                    unsafe {
                        *sieve.get_unchecked_mut(b) |= $m0;
                        *sieve.get_unchecked_mut(b + $j1) |= $m1;
                        *sieve.get_unchecked_mut(b + $j2) |= $m2;
                        *sieve.get_unchecked_mut(b + $j3) |= $m3;
                        *sieve.get_unchecked_mut(b + $j4) |= $m4;
                        *sieve.get_unchecked_mut(b + $j5) |= $m5;
                        *sieve.get_unchecked_mut(b + $j6) |= $m6;
                        *sieve.get_unchecked_mut(b + $j7) |= $m7;
                    }
                    b += $j8;
                }

                poststep!($label, 0, $m0, 0);
                poststep!($label, 1, $m1, $j1);
                poststep!($label, 2, $m2, $j2);
                poststep!($label, 3, $m3, $j3);
                poststep!($label, 4, $m4, $j4);
                poststep!($label, 5, $m5, $j5);
                poststep!($label, 6, $m6, $j6);

                b += $j7;
                7
            }};
        }

        *s_ref = match r {
            1 => 'outer: {
                cycle!('outer, 6*k, 10*k, 12*k, 16*k, 18*k, 22*k, 28*k, 30*k+1, 1, 2, 4, 8, 16, 32, 64, 128)
            }
            7 => 'outer: {
                cycle!('outer, 4*k+1, 6*k+2, 10*k+2, 12*k+3, 16*k+4, 22*k+5, 24*k+6, 30*k+7, 32, 16, 1, 128, 8, 4, 64, 2)
            }
            11 => 'outer: {
                cycle!('outer, 2*k, 6*k+2, 8*k+2, 12*k+4, 18*k+6, 20*k+7, 26*k+9, 30*k+11, 1, 64, 2, 128, 8, 32, 4, 16)
            }
            13 => 'outer: {
                cycle!('outer, 4*k+2, 6*k+3, 10*k+4, 16*k+7, 18*k+8, 24*k+11, 28*k+12, 30*k+13, 32, 4, 2, 128, 16, 8, 1, 64)
            }
            17 => 'outer: {
                cycle!('outer, 2*k+1, 6*k+4, 12*k+7, 14*k+8, 20*k+11, 24*k+14, 26*k+15, 30*k+17, 32, 64, 1, 8, 16, 128, 2, 4)
            }
            19 => 'outer: {
                cycle!('outer, 4*k+2, 10*k+6, 12*k+7, 18*k+11, 22*k+13, 24*k+15, 28*k+17, 30*k+19, 1, 16, 4, 32, 8, 128, 2, 64)
            }
            23 => 'outer: {
                cycle!('outer, 6*k+5, 8*k+6, 14*k+11, 18*k+14, 20*k+15, 24*k+19, 26*k+20, 30*k+23, 32, 2, 64, 4, 8, 128, 1, 16)
            }
            29 => 'outer: {
                cycle!('outer, 2*k+1, 8*k+7, 12*k+11, 14*k+13, 18*k+17, 20*k+19, 24*k+23, 30*k+29, 1, 128, 64, 32, 16, 8, 4, 2)
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

macro_rules! apply_presieve4_impl {
    ($sieve:expr, $from:expr, $to:expr, $pre0:expr, $pre1:expr, $pre2:expr, $pre3:expr, $assign:tt) => {{
        let last = $to - $from + 1;
        let mut pos0 = $from % $pre0.len();
        let mut pos1 = $from % $pre1.len();
        let mut pos2 = $from % $pre2.len();
        let mut pos3 = $from % $pre3.len();

        let mut offset = 0;
        while offset < last {
            let min_len = (last - offset)
                .min($pre0.len() - pos0)
                .min($pre1.len() - pos1)
                .min($pre2.len() - pos2)
                .min($pre3.len() - pos3);

            for i in 0..min_len {
                $sieve[$from + offset + i] $assign
                    $pre0[pos0 + i] | $pre1[pos1 + i]
                  | $pre2[pos2 + i] | $pre3[pos3 + i];
            }

            offset += min_len;
            pos0 = (pos0 + min_len) % $pre0.len();
            pos1 = (pos1 + min_len) % $pre1.len();
            pos2 = (pos2 + min_len) % $pre2.len();
            pos3 = (pos3 + min_len) % $pre3.len();
        }
    }};
}

macro_rules! apply_presieve_first4 {
    ($sieve:expr, $from:expr, $to:expr, $pre0:expr, $pre1:expr, $pre2:expr, $pre3:expr) => {
        apply_presieve4_impl!($sieve, $from, $to, $pre0, $pre1, $pre2, $pre3, =)
    };
}

macro_rules! apply_presieve4 {
    ($sieve:expr, $from:expr, $to:expr, $pre0:expr, $pre1:expr, $pre2:expr, $pre3:expr) => {
        apply_presieve4_impl!($sieve, $from, $to, $pre0, $pre1, $pre2, $pre3, |=)
    };
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

struct SieveAlloc {
    ptr: *mut u8,
    len: usize,
    cap: usize,
    from_mmap: bool,
}

impl Drop for SieveAlloc {
    fn drop(&mut self) {
        unsafe {
            if self.from_mmap {
                libc::munmap(self.ptr as *mut libc::c_void, self.cap);
            } else {
                drop(Vec::from_raw_parts(self.ptr, self.len, self.cap));
            }
        }
    }
}

impl std::ops::Deref for SieveAlloc {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl std::ops::DerefMut for SieveAlloc {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

fn allocate_sieve(sieve_len: usize) -> SieveAlloc {
    #[cfg(target_os = "linux")]
    {
        const HUGE_PAGE: usize = 2 * 1024 * 1024;
        let cap = (sieve_len + HUGE_PAGE - 1) & !(HUGE_PAGE - 1);

        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                cap,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        assert_ne!(
            ptr,
            libc::MAP_FAILED,
            "mmap failed: {}",
            std::io::Error::last_os_error()
        );
        let ptr = ptr as *mut u8;

        unsafe {
            libc::madvise(ptr as *mut libc::c_void, cap, libc::MADV_HUGEPAGE);
            let ret = libc::madvise(ptr as *mut libc::c_void, cap, libc::MADV_POPULATE_WRITE);
            if ret != 0 {
                std::slice::from_raw_parts_mut(ptr, sieve_len).fill(0);
            }
        }

        SieveAlloc {
            ptr,
            len: sieve_len,
            cap,
            from_mmap: true,
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        let mut v = vec![0u8; sieve_len];
        let ptr = v.as_mut_ptr();
        let cap = v.capacity();
        std::mem::forget(v);
        SieveAlloc {
            ptr,
            len: sieve_len,
            cap,
            from_mmap: false,
        }
    }
}

fn sieve(n: usize) -> SieveAlloc {
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

    let sieve_len = (n / WHEEL_SIZE) + 1;
    let mut sieve = allocate_sieve(sieve_len);

    let mut small_primes: Vec<usize> = small_primes_gt(n, *max_presieve_prime as usize);
    small_primes.sort_by_key(|&p| p % WHEEL_SIZE);
    let (l1_primes, l2_primes): (Vec<_>, Vec<_>) = small_primes.into_iter().partition(|&p| p <= TPRIME);
    let mut l1_resume = Resume::from_primes(&l1_primes);
    let mut l2_resume = Resume::from_primes(&l2_primes);

    for from in (0..sieve_len).step_by(L2_CHUNK) {
        let to = (from + L2_CHUNK - 1).min(sieve_len - 1);

        apply_presieve_first4!(&mut sieve, from, to, &presieve_0, &presieve_1, &presieve_2, &presieve_3);
        apply_presieve4!(&mut sieve, from, to, &presieve_4, &presieve_5, &presieve_6, &presieve_7);
        apply_presieve4!(&mut sieve, from, to, &presieve_8, &presieve_9, &presieve_a, &presieve_b);
        apply_presieve4!(&mut sieve, from, to, &presieve_c, &presieve_d, &presieve_e, &presieve_f);

        for from_l1 in (from..=to).step_by(L1_CHUNK) {
            let to_l1 = (from_l1 + L1_CHUNK - 1).min(to);
            sieve_helper(&mut sieve, to_l1, &mut l1_resume);
        }
        sieve_helper(&mut sieve, to, &mut l2_resume);
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
