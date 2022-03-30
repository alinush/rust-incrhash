use curve25519_dalek::ristretto::RistrettoPoint;
use rust_incrhash::IncrHash;

fn main() {
    println!("Hello, world!");

    type RistBlakeHash = IncrHash<RistrettoPoint, blake2::Blake2b512>;
    let h: RistBlakeHash = RistBlakeHash::default();

    let a = RistBlakeHash::from(b"hello world".as_slice());
    let b = RistBlakeHash::from(b"sup universe".as_slice());

    // h += a;
    // h += b;
}
