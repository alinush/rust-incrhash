// use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
//
// use rand::Rng;
// use digest::Digest;
// use digest::consts::U64;
// use curve25519_dalek::ristretto::RistrettoPoint;
//
// const HASH_INPUT_SIZES: [usize; 5] = [32, 64, 128, 256, 512]; //, 1024, 2048, 4096, 8192, 16384, 32768, 65536];
//
// pub fn hash_to_curve_benchmark<T: Digest<OutputSize = U64> + Default>(c: &mut Criterion) {
//     let bench_name = "Hash to group".to_owned();
//     for input_size in &HASH_INPUT_SIZES {
//         c.bench_with_input(
//             BenchmarkId::new(
//                 &bench_name,
//                 *input_size
//             ),
//             input_size,
//             |b, &size| {
//                 let random_bytes : Vec<u8> = (0..size).map(|_| rand::thread_rng().gen::<u8>()).collect();
//                 b.iter(
//                     || RistrettoPoint::hash_from_bytes::<T>(&random_bytes)
//                 );
//             }
//         );
//     }
// }

//criterion_group!(benches, hash_to_curve_benchmark::<blake2::Blake2b512>, hash_to_curve_benchmark::<sha2::Sha512>);
//criterion_main!(benches);
