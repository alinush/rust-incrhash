use curve25519_dalek::ristretto::RistrettoPoint;
use rust_incrhash::IncrHash;

fn main() {
    type RistBlakeHash = IncrHash<RistrettoPoint, blake2::Blake2b512>;
    let mut h: RistBlakeHash = RistBlakeHash::default();

    let a = RistBlakeHash::from(b"hello world".as_slice());
    let b = RistBlakeHash::from(b"sup universe".as_slice());

    let mut c = &a + &b;

    h += &a;
    h += &b;

    assert_eq!(c, h);

    c -= &a;
    c -= &b;

    assert_eq!(c, RistBlakeHash::default());

    println!("All is well.");
}
