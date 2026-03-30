use std::io::{self, BufWriter, Write};

const N: usize = 1 << 31;
const M: usize = 33_333_333;
const D: usize = 1_234_567_890;
const WHEEL_SIZE: usize = 30;
const RESIDUES_PER_WHEEL: usize = 8;
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

fn segmented_wheel235_sieve(n: usize) -> impl Fn(usize) -> bool {
    // compute small primes up to sqrt(n)
    let sqrt_n = (n as f64).sqrt() as usize + 1;
    let is_prime = naive_sieve(sqrt_n);
    let small_primes: Vec<usize> = (7..sqrt_n).step_by(2).filter(|&i| is_prime(i)).collect();

    // surviving residues mod 30 after wheel factorization: 1, 7, 11, 13, 17, 19, 23, 29
    //
    // so we keep in our sieve these numbers:
    //  1,  7,  11,  13,  17,  19,  23,  29,
    // 31, 37,  41,  43,  47,  49,  53,  59,
    // 61, 67,  71,  73,  77,  79,  83,  89,
    // 91, 97, 101, 103, 107, 109, 113, 119,
    // ...
    //
    let sieve_len = (n / WHEEL_SIZE) + 1;
    let mut sieve: Vec<u8> = vec![0; sieve_len];

    let mut bit_mask: [u8; WHEEL_SIZE] = [0; WHEEL_SIZE];
    for (i, &r) in [1, 7, 11, 13, 17, 19, 23, 29].iter().enumerate() {
        bit_mask[r] = 1 << i;
    }

    let mut resume_state: Vec<_> = small_primes
        .iter()
        .map(|&p| (p * p / WHEEL_SIZE, 0))
        .collect();

    sieve[0] |= bit_mask[1];
    for chunk_start in (0..sieve_len).step_by(CHUNK) {
        let chunk_end = (chunk_start + CHUNK).min(sieve_len - 1);

        for (i, &p) in small_primes.iter().enumerate() {
            let r = p % WHEEL_SIZE;
            let k = p / WHEEL_SIZE;

            let cycle: [(u8, usize); RESIDUES_PER_WHEEL] = match r {
                1 => {
                    // r = 1 => p = 30*k + 1 => p^2 = (30*k + 1)^2 => p^2 = 1 (mod 30)
                    // we want to cross out these numbers:
                    //
                    //  1: d0 = p*p        = 30*b0 + 1
                    //  7: d1 = d0 + 6*p = (30*b0 + 1)  + 6*(30*k + 1) = 30*(b0 + 6*k)     +  7: let b1 = b0 + 6*k
                    // 11: d2 = d1 + 4*p = (30*b1 + 7)  + 4*(30*k + 1) = 30*(b1 + 4*k)     + 11: let b2 = b1 + 4*k
                    // 13: d3 = d2 + 2*p = (30*b2 + 11) + 2*(30*k + 1) = 30*(b2 + 2*k)     + 13: let b3 = b2 + 2*k
                    // 17: d4 = d3 + 4*p = (30*b3 + 13) + 4*(30*k + 1) = 30*(b3 + 4*k)     + 17: let b4 = b3 + 4*k
                    // 19: d5 = d4 + 2*p = (30*b4 + 17) + 2*(30*k + 1) = 30*(b4 + 2*k)     + 19: let b5 = b4 + 2*k
                    // 23: d6 = d5 + 4*p = (30*b5 + 19) + 4*(30*k + 1) = 30*(b5 + 4*k)     + 23: let b6 = b5 + 4*k
                    // 29: d7 = d6 + 6*p = (30*b6 + 23) + 6*(30*k + 1) = 30*(b6 + 6*k)     + 29: let b7 = b6 + 6*k
                    //  1: d8 = d7 + 2*p = (30*b7 + 29) + 2*(30*k + 1) = 30*(b7 + 2*k + 1) +  1: let b8 = b7 + 2*k + 1
                    // ...

                    // so we mark d0 as composite (res = 1), jump forward by 6 * k;
                    //       mark d1 as composite (res = 7), jump forward by 4 * k;
                    //       ...
                    //       mark d7 as composite (res = 29), jump forward by (2 * k + 1);

                    [
                        (bit_mask[1], 6 * k),
                        (bit_mask[7], 4 * k),
                        (bit_mask[11], 2 * k),
                        (bit_mask[13], 4 * k),
                        (bit_mask[17], 2 * k),
                        (bit_mask[19], 4 * k),
                        (bit_mask[23], 6 * k),
                        (bit_mask[29], 2 * k + 1),
                    ]
                }
                7 => {
                    // r = 7 => p = 30*k + 7 => p^2 = (30*k + 7)^2 => p^2 = 19 (mod 30)
                    // we want to cross out these numbers:
                    //
                    //  7: d0 = p*p = 30*b0 + 19
                    // 11: d1 = d0 + 4*p = (30*b0 + 19) + 4*(30*k + 7) = 30*(b0 + 4*k + 1) + 17; let b1 = b0 + 4*k + 1
                    // 13: d2 = d1 + 2*p = (30*b1 + 17) + 2*(30*k + 7) = 30*(b1 + 2*k + 1) +  1; let b2 = b1 + 2*k + 1
                    // 17: d3 = d2 + 4*p = (30*b2 +  1) + 4*(30*k + 7) = 30*(b2 + 4*k)     + 29; let b3 = b2 + 4*k
                    // 19: d4 = d3 + 2*p = (30*b3 + 29) + 2*(30*k + 7) = 30*(b3 + 2*k + 1) + 13; let b4 = b3 + 2*k + 1
                    // 23: d5 = d4 + 4*p = (30*b4 + 13) + 4*(30*k + 7) = 30*(b4 + 4*k + 1) + 11; let b5 = b4 + 4*k + 1
                    // 29: d6 = d5 + 6*p = (30*b5 + 11) + 6*(30*k + 7) = 30*(b5 + 6*k + 1) + 23; let b6 = b5 + 6*k + 1
                    //  1: d7 = d6 + 2*p = (30*b6 + 23) + 2*(30*k + 7) = 30*(b6 + 2*k + 1) +  7; let b7 = b6 + 2*k + 1
                    //  7: d8 = d7 + 6*p = (30*b7 +  7) + 6*(30*k + 7) = 30*(b7 + 6*k + 1) + 19; let b8 = b7 + 6*k + 1
                    [
                        (bit_mask[19], 4 * k + 1),
                        (bit_mask[17], 2 * k + 1),
                        (bit_mask[1], 4 * k),
                        (bit_mask[29], 2 * k + 1),
                        (bit_mask[13], 4 * k + 1),
                        (bit_mask[11], 6 * k + 1),
                        (bit_mask[23], 2 * k + 1),
                        (bit_mask[7], 6 * k + 1),
                    ]
                }
                11 => {
                    // r = 11 => p = 30*k + 11 => p^2 = (30*k + 11)^2 => p^2 = 1 (mod 30)
                    // we want to cross out these numbers:
                    //
                    // 11: d0 = p*p = 30*b0 + 1
                    // 13: d1 = d0 + 2*p = (30*b0 +  1) + 2*(30*k + 11) = 30*(b0 + 2*k)     + 23; let b1 = b0 + 2*k
                    // 17: d2 = d1 + 4*p = (30*b1 + 23) + 4*(30*k + 11) = 30*(b1 + 4*k + 2) +  7; let b2 = b1 + 4*k + 2
                    // 19: d3 = d2 + 2*p = (30*b2 +  7) + 2*(30*k + 11) = 30*(b2 + 2*k)     + 29; let b3 = b2 + 2*k
                    // 23: d4 = d3 + 4*p = (30*b3 + 29) + 4*(30*k + 11) = 30*(b3 + 4*k + 2) + 13; let b4 = b3 + 4*k + 2
                    // 29: d5 = d4 + 6*p = (30*b4 + 13) + 6*(30*k + 11) = 30*(b4 + 6*k + 2) + 19; let b5 = b4 + 6*k + 2
                    //  1: d6 = d5 + 2*p = (30*b5 + 19) + 2*(30*k + 11) = 30*(b5 + 2*k + 1) + 11; let b6 = b5 + 2*k + 1
                    //  7: d7 = d6 + 6*p = (30*b6 + 11) + 6*(30*k + 11) = 30*(b6 + 6*k + 2) + 17; let b7 = b6 + 6*k + 2
                    // 11: d8 = d7 + 4*p = (30*b7 + 17) + 4*(30*k + 11) = 30*(b7 + 4*k + 2) +  1; let b8 = b7 + 4*k + 2
                    [
                        (bit_mask[1], 2 * k),
                        (bit_mask[23], 4 * k + 2),
                        (bit_mask[7], 2 * k),
                        (bit_mask[29], 4 * k + 2),
                        (bit_mask[13], 6 * k + 2),
                        (bit_mask[19], 2 * k + 1),
                        (bit_mask[11], 6 * k + 2),
                        (bit_mask[17], 4 * k + 2),
                    ]
                }
                13 => {
                    // r = 13 => p = 30*k + 13 => p^2 = (30*k + 13)^2 => p^2 = 19 (mod 30)
                    // we want to cross out these numbers:
                    //
                    // 13: d0 = p*p = 30*b0 + 19
                    // 17: d1 = d0 + 4*p = (30*b0 + 19) + 4*(30*k + 13) = 30*(b0 + 4*k + 2) + 11; let b1 = b0 + 4*k + 2
                    // 19: d2 = d1 + 2*p = (30*b1 + 11) + 2*(30*k + 13) = 30*(b1 + 2*k + 1) +  7; let b2 = b1 + 2*k + 1
                    // 23: d3 = d2 + 4*p = (30*b2 +  7) + 4*(30*k + 13) = 30*(b2 + 4*k + 1) + 29; let b3 = b2 + 4*k + 1
                    // 29: d4 = d3 + 6*p = (30*b3 + 29) + 6*(30*k + 13) = 30*(b3 + 6*k + 3) + 17; let b4 = b3 + 6*k + 3
                    //  1: d5 = d4 + 2*p = (30*b4 + 17) + 2*(30*k + 13) = 30*(b4 + 2*k + 1) + 13; let b5 = b4 + 2*k + 1
                    //  7: d6 = d5 + 6*p = (30*b5 + 13) + 6*(30*k + 13) = 30*(b5 + 6*k + 3) +  1; let b6 = b5 + 6*k + 3
                    // 11: d7 = d6 + 4*p = (30*b6 +  1) + 4*(30*k + 13) = 30*(b6 + 4*k + 1) + 23; let b7 = b6 + 4*k + 1
                    // 13: d8 = d7 + 2*p = (30*b7 + 23) + 2*(30*k + 13) = 30*(b7 + 2*k + 1) + 19; let b8 = b7 + 2*k + 1
                    [
                        (bit_mask[19], 4 * k + 2),
                        (bit_mask[11], 2 * k + 1),
                        (bit_mask[7], 4 * k + 1),
                        (bit_mask[29], 6 * k + 3),
                        (bit_mask[17], 2 * k + 1),
                        (bit_mask[13], 6 * k + 3),
                        (bit_mask[1], 4 * k + 1),
                        (bit_mask[23], 2 * k + 1),
                    ]
                }
                17 => {
                    // r = 17 => p = 30*k + 17 => p^2 = (30*k + 17)^2 => p^2 = 19 (mod 30)
                    // we want to cross out these numbers:
                    //
                    // 17: d0 = p*p = 30*b0 + 19
                    // 19: d1 = d0 + 2*p = (30*b0 + 19) + 2*(30*k + 17) = 30*(b0 + 2*k + 1) + 23; let b1 = b0 + 2*k + 1
                    // 23: d2 = d1 + 4*p = (30*b1 + 23) + 4*(30*k + 17) = 30*(b1 + 4*k + 3) +  1; let b2 = b1 + 4*k + 3
                    // 29: d3 = d2 + 6*p = (30*b2 +  1) + 6*(30*k + 17) = 30*(b2 + 6*k + 3) + 13; let b3 = b2 + 6*k + 3
                    //  1: d4 = d3 + 2*p = (30*b3 + 13) + 2*(30*k + 17) = 30*(b3 + 2*k + 1) + 17; let b4 = b3 + 2*k + 1
                    //  7: d5 = d4 + 6*p = (30*b4 + 17) + 6*(30*k + 17) = 30*(b4 + 6*k + 3) + 29; let b5 = b4 + 6*k + 3
                    // 11: d6 = d5 + 4*p = (30*b5 + 29) + 4*(30*k + 17) = 30*(b5 + 4*k + 3) +  7; let b6 = b5 + 4*k + 3
                    // 13: d7 = d6 + 2*p = (30*b6 +  7) + 2*(30*k + 17) = 30*(b6 + 2*k + 1) + 11; let b7 = b6 + 2*k + 1
                    // 17: d8 = d7 + 4*p = (30*b7 + 11) + 4*(30*k + 17) = 30*(b7 + 4*k + 2) + 19; let b8 = b7 + 4*k + 2
                    [
                        (bit_mask[19], 2 * k + 1),
                        (bit_mask[23], 4 * k + 3),
                        (bit_mask[1], 6 * k + 3),
                        (bit_mask[13], 2 * k + 1),
                        (bit_mask[17], 6 * k + 3),
                        (bit_mask[29], 4 * k + 3),
                        (bit_mask[7], 2 * k + 1),
                        (bit_mask[11], 4 * k + 2),
                    ]
                }
                19 => {
                    // r = 19 => p = 30*k + 19 => p^2 = (30*k + 19)^2 => p^2 = 1 (mod 30)
                    // we want to cross out these numbers:
                    //
                    // 19: d0 = p*p = 30*b0 + 1
                    // 23: d1 = d0 + 4*p = (30*b0 +  1) + 4*(30*k + 19) = 30*(b0 + 4*k + 2) + 17; let b1 = b0 + 4*k + 2
                    // 29: d2 = d1 + 6*p = (30*b1 + 17) + 6*(30*k + 19) = 30*(b1 + 6*k + 4) + 11; let b2 = b1 + 6*k + 4
                    //  1: d3 = d2 + 2*p = (30*b2 + 11) + 2*(30*k + 19) = 30*(b2 + 2*k + 1) + 19; let b3 = b2 + 2*k + 1
                    //  7: d4 = d3 + 6*p = (30*b3 + 19) + 6*(30*k + 19) = 30*(b3 + 6*k + 4) + 13; let b4 = b3 + 6*k + 4
                    // 11: d5 = d4 + 4*p = (30*b4 + 13) + 4*(30*k + 19) = 30*(b4 + 4*k + 2) + 29; let b5 = b4 + 4*k + 2
                    // 13: d6 = d5 + 2*p = (30*b5 + 29) + 2*(30*k + 19) = 30*(b5 + 2*k + 2) +  7; let b6 = b5 + 2*k + 2
                    // 17: d7 = d6 + 4*p = (30*b6 +  7) + 4*(30*k + 19) = 30*(b6 + 4*k + 2) + 23; let b7 = b6 + 4*k + 2
                    // 19: d8 = d7 + 2*p = (30*b7 + 23) + 2*(30*k + 19) = 30*(b7 + 2*k + 2) +  1; let b8 = b7 + 2*k + 2
                    [
                        (bit_mask[1], 4 * k + 2),
                        (bit_mask[17], 6 * k + 4),
                        (bit_mask[11], 2 * k + 1),
                        (bit_mask[19], 6 * k + 4),
                        (bit_mask[13], 4 * k + 2),
                        (bit_mask[29], 2 * k + 2),
                        (bit_mask[7], 4 * k + 2),
                        (bit_mask[23], 2 * k + 2),
                    ]
                }
                23 => {
                    // r = 23 => p = 30*k + 23 => p^2 = (30*k + 23)^2 => p^2 = 19 (mod 30)
                    // we want to cross out these numbers:
                    //
                    // 23: d0 = p*p = 30*b0 + 19
                    // 29: d1 = d0 + 6*p = (30*b0 + 19) + 6*(30*k + 23) = 30*(b0 + 6*k + 5) +  7; let b1 = b0 + 6*k + 5
                    //  1: d2 = d1 + 2*p = (30*b1 +  7) + 2*(30*k + 23) = 30*(b1 + 2*k + 1) + 23; let b2 = b1 + 2*k + 1
                    //  7: d3 = d2 + 6*p = (30*b2 + 23) + 6*(30*k + 23) = 30*(b2 + 6*k + 5) + 11; let b3 = b2 + 6*k + 5
                    // 11: d4 = d3 + 4*p = (30*b3 + 11) + 4*(30*k + 23) = 30*(b3 + 4*k + 3) + 13; let b4 = b3 + 4*k + 3
                    // 13: d5 = d4 + 2*p = (30*b4 + 13) + 2*(30*k + 23) = 30*(b4 + 2*k + 1) + 29; let b5 = b4 + 2*k + 1
                    // 17: d6 = d5 + 4*p = (30*b5 + 29) + 4*(30*k + 23) = 30*(b5 + 4*k + 4) +  1; let b6 = b5 + 4*k + 4
                    // 19: d7 = d6 + 2*p = (30*b6 +  1) + 2*(30*k + 23) = 30*(b6 + 2*k + 1) + 17; let b7 = b6 + 2*k + 1
                    // 23: d8 = d7 + 4*p = (30*b7 + 17) + 4*(30*k + 23) = 30*(b7 + 4*k + 3) + 19; let b8 = b7 + 4*k + 3
                    [
                        (bit_mask[19], 6 * k + 5),
                        (bit_mask[7], 2 * k + 1),
                        (bit_mask[23], 6 * k + 5),
                        (bit_mask[11], 4 * k + 3),
                        (bit_mask[13], 2 * k + 1),
                        (bit_mask[29], 4 * k + 4),
                        (bit_mask[1], 2 * k + 1),
                        (bit_mask[17], 4 * k + 3),
                    ]
                }
                29 => {
                    // r = 29 => p = 30*k + 29 => p^2 = (30*k + 29)^2 => p^2 = 1 (mod 30)
                    // we want to cross out these numbers:
                    //
                    // 29: d0 = p*p = 30*b0 + 1
                    //  1: d1 = d0 + 2*p = (30*b0 +  1) + 2*(30*k + 29) = 30*(b0 + 2*k + 1) + 29; let b1 = b0 + 2*k + 1
                    //  7: d2 = d1 + 6*p = (30*b1 + 29) + 6*(30*k + 29) = 30*(b1 + 6*k + 6) + 23; let b2 = b1 + 6*k + 6
                    // 11: d3 = d2 + 4*p = (30*b2 + 23) + 4*(30*k + 29) = 30*(b2 + 4*k + 4) + 19; let b3 = b2 + 4*k + 4
                    // 13: d4 = d3 + 2*p = (30*b3 + 19) + 2*(30*k + 29) = 30*(b3 + 2*k + 2) + 17; let b4 = b3 + 2*k + 2
                    // 17: d5 = d4 + 4*p = (30*b4 + 17) + 4*(30*k + 29) = 30*(b4 + 4*k + 4) + 13; let b5 = b4 + 4*k + 4
                    // 19: d6 = d5 + 2*p = (30*b5 + 13) + 2*(30*k + 29) = 30*(b5 + 2*k + 2) + 11; let b6 = b5 + 2*k + 2
                    // 23: d7 = d6 + 4*p = (30*b6 + 11) + 4*(30*k + 29) = 30*(b6 + 4*k + 4) +  7; let b7 = b6 + 4*k + 4
                    // 29: d8 = d7 + 6*p = (30*b7 +  7) + 6*(30*k + 29) = 30*(b7 + 6*k + 6) +  1; let b8 = b7 + 6*k + 6
                    [
                        (bit_mask[1], 2 * k + 1),
                        (bit_mask[29], 6 * k + 6),
                        (bit_mask[23], 4 * k + 4),
                        (bit_mask[19], 2 * k + 2),
                        (bit_mask[17], 4 * k + 4),
                        (bit_mask[13], 2 * k + 2),
                        (bit_mask[11], 4 * k + 4),
                        (bit_mask[7], 6 * k + 6),
                    ]
                }
                _ => unreachable!(),
            };

            let (mut b, mut s) = resume_state[i];
            let mut next_s = s;
            'outer: while b <= chunk_end {
                for (j, &(bit, jump)) in cycle[s..].iter().enumerate() {
                    sieve[b] |= bit;
                    b += jump;
                    if b > chunk_end {
                        next_s = (s + j + 1) % RESIDUES_PER_WHEEL;
                        break 'outer;
                    }
                }
                s = 0;
                next_s = 0;
            }
            resume_state[i] = (b, next_s);
        }
    }

    move |a| {
        if a == 2 || a == 3 || a == 5 {
            return true;
        }

        let r = a % WHEEL_SIZE;
        match bit_mask[r] {
            0 => false,
            b => (sieve[a / WHEEL_SIZE] & b) == 0,
        }
    }
}

fn main() -> io::Result<()> {
    let out = io::stdout();
    let mut out = BufWriter::with_capacity(65_536, out.lock());

    let t_start = std::time::Instant::now();
    let is_prime = segmented_wheel235_sieve(N);
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
