use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, BenchmarkGroup, measurement::{ Measurement }};

use curve25519_dalek::ristretto::RistrettoPoint;
use digest::Digest;
use digest::consts::U64;
use rand::Rng;

const HASH_INPUT_SIZES: [usize; 8] = [32, 64, 128, 256, 512, 1024, 2048, 4096]; // 8192, 16384, 32768, 65536];

pub fn hash_to_curve_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_to_curve");
    hash_to_curve_benchmark::<blake2::Blake2b512, _>(&mut group, "blake2b512");
    // WARNING: It seems like the Elligator2 hashing used inside curve25519-dalek asks for a hash function with 512 bit output
    //hash_to_curve_benchmark::<sha2::Sha256, _>(&mut group, "sha256");
    hash_to_curve_benchmark::<sha2::Sha512, _>(&mut group, "sha2-512");
    hash_to_curve_benchmark::<sha3::Sha3_512, _>(&mut group, "sha3-512");
    hash_to_curve_benchmark::<sha3::Keccak512, _>(&mut group, "keccak512");
    group.finish();
}

pub fn hash_to_curve_benchmark<T: Digest<OutputSize = U64> + Default, M: Measurement>(c: &mut BenchmarkGroup<M>, hash_alg_name: &str) {
    let bench_name = hash_alg_name.to_owned();
    for input_size in &HASH_INPUT_SIZES {
        c.bench_with_input(
            BenchmarkId::new(
                &bench_name,
                *input_size
            ),
            input_size,
            |b, &size| {
                let random_bytes : Vec<u8> = (0..size).map(|_| rand::thread_rng().gen::<u8>()).collect();
                b.iter(
                    || RistrettoPoint::hash_from_bytes::<T>(&random_bytes)
                );
            }
        );
    }
}

criterion_group!(benches, hash_to_curve_group);
criterion_main!(benches);

