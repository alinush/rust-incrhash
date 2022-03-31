use criterion::{
    criterion_group, criterion_main,
    Criterion,
};
use rand::{distributions::Alphanumeric, Rng};
use rust_incrhash::RistBlakeIncHash;

const KEY_SIZE: usize = 32; // 32 bytes or characters
const VALUE_SIZE: usize = 64; // 64 bytes or characters

pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn incr_hash_benchmark(c: &mut Criterion) {
    let mut h = RistBlakeIncHash::default();
    c.bench_function(
        "Incremental hash",
        |b| {
            let random_key: String = random_string(KEY_SIZE);
            let old_val: String = random_string(VALUE_SIZE);
            let new_val: String = random_string(VALUE_SIZE);
            let old_elem : String = random_key.clone() + "|" + old_val.as_str();
            let new_elem : String = random_key + "|" + new_val.as_str();
            b.iter(
                || {
                    h -= RistBlakeIncHash::from(old_elem.as_bytes());
                    h += RistBlakeIncHash::from(new_elem.as_bytes());
                }
            );
        }
    );
}

criterion_group!(benches, incr_hash_benchmark);
criterion_main!(benches);
