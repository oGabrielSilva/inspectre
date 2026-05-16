use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use aes::Aes256;
use cipher::array::Array;
use cipher::{BlockCipherEncrypt, KeyInit};
use rayon::prelude::*;
use sha2::{Digest, Sha256};

use super::types::BenchScore;

const SHA_BLOCK_BYTES: usize = 1024 * 1024;
const AES_BLOCK_BYTES: usize = 1024 * 1024;
const PRIME_LIMIT: usize = 1_000_000;

pub fn sha256_single(duration: Duration) -> BenchScore {
    let data = vec![0u8; SHA_BLOCK_BYTES];
    let start = Instant::now();
    let mut iterations = 0u64;
    while start.elapsed() < duration {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let _ = hasher.finalize();
        iterations += 1;
    }
    let elapsed = start.elapsed();
    BenchScore {
        throughput: bytes_per_sec(iterations, SHA_BLOCK_BYTES, elapsed),
        iterations,
        duration_ms: elapsed.as_millis() as u64,
        thread_count: 1,
    }
}

pub fn sha256_multi(duration: Duration) -> BenchScore {
    let threads = rayon::current_num_threads();
    let counter = AtomicU64::new(0);
    let start = Instant::now();

    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                let data = vec![0u8; SHA_BLOCK_BYTES];
                while start.elapsed() < duration {
                    let mut hasher = Sha256::new();
                    hasher.update(&data);
                    let _ = hasher.finalize();
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    let elapsed = start.elapsed();
    let iterations = counter.load(Ordering::Relaxed);
    BenchScore {
        throughput: bytes_per_sec(iterations, SHA_BLOCK_BYTES, elapsed),
        iterations,
        duration_ms: elapsed.as_millis() as u64,
        thread_count: threads as u16,
    }
}

pub fn aes256_single(duration: Duration) -> BenchScore {
    let key = Array::from([0u8; 32]);
    let cipher = Aes256::new(&key);
    let mut block = Array::from([0u8; 16]);
    let blocks_per_iter = AES_BLOCK_BYTES / 16;
    let start = Instant::now();
    let mut iterations = 0u64;
    while start.elapsed() < duration {
        for _ in 0..blocks_per_iter {
            cipher.encrypt_block(&mut block);
        }
        iterations += 1;
    }
    let elapsed = start.elapsed();
    BenchScore {
        throughput: bytes_per_sec(iterations, AES_BLOCK_BYTES, elapsed),
        iterations,
        duration_ms: elapsed.as_millis() as u64,
        thread_count: 1,
    }
}

pub fn aes256_multi(duration: Duration) -> BenchScore {
    let threads = rayon::current_num_threads();
    let counter = AtomicU64::new(0);
    let start = Instant::now();
    let blocks_per_iter = AES_BLOCK_BYTES / 16;

    rayon::scope(|s| {
        for _ in 0..threads {
            s.spawn(|_| {
                let key = Array::from([0u8; 32]);
                let cipher = Aes256::new(&key);
                let mut block = Array::from([0u8; 16]);
                while start.elapsed() < duration {
                    for _ in 0..blocks_per_iter {
                        cipher.encrypt_block(&mut block);
                    }
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });

    let elapsed = start.elapsed();
    let iterations = counter.load(Ordering::Relaxed);
    BenchScore {
        throughput: bytes_per_sec(iterations, AES_BLOCK_BYTES, elapsed),
        iterations,
        duration_ms: elapsed.as_millis() as u64,
        thread_count: threads as u16,
    }
}

pub fn prime_single(duration: Duration) -> BenchScore {
    let start = Instant::now();
    let mut iterations = 0u64;
    while start.elapsed() < duration {
        sieve(PRIME_LIMIT);
        iterations += 1;
    }
    let elapsed = start.elapsed();
    BenchScore {
        throughput: items_per_sec(iterations, PRIME_LIMIT, elapsed),
        iterations,
        duration_ms: elapsed.as_millis() as u64,
        thread_count: 1,
    }
}

pub fn prime_multi(duration: Duration) -> BenchScore {
    let threads = rayon::current_num_threads();
    let start = Instant::now();
    let total = (0..threads)
        .into_par_iter()
        .map(|_| {
            let mut local = 0u64;
            while start.elapsed() < duration {
                sieve(PRIME_LIMIT);
                local += 1;
            }
            local
        })
        .sum::<u64>();

    let elapsed = start.elapsed();
    BenchScore {
        throughput: items_per_sec(total, PRIME_LIMIT, elapsed),
        iterations: total,
        duration_ms: elapsed.as_millis() as u64,
        thread_count: threads as u16,
    }
}

fn sieve(limit: usize) -> usize {
    let mut is_prime = vec![true; limit];
    if limit >= 1 {
        is_prime[0] = false;
    }
    if limit >= 2 {
        is_prime[1] = false;
    }
    let sqrt = (limit as f64).sqrt() as usize + 1;
    for i in 2..sqrt {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    is_prime.iter().filter(|&&v| v).count()
}

fn bytes_per_sec(iterations: u64, block_bytes: usize, elapsed: Duration) -> f64 {
    let total = iterations as f64 * block_bytes as f64;
    total / elapsed.as_secs_f64().max(0.0001)
}

fn items_per_sec(iterations: u64, batch: usize, elapsed: Duration) -> f64 {
    let total = iterations as f64 * batch as f64;
    total / elapsed.as_secs_f64().max(0.0001)
}
