use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::traits::Identity;
use digest::consts::U64;
use digest::Digest;

use crate::{HashableFromBytes, IncrHash};

pub type RistBlakeIncHash = IncrHash<RistrettoPoint, blake2::Blake2b512>;

impl HashableFromBytes for RistrettoPoint {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self {
        RistrettoPoint::hash_from_bytes::<H>(bytes)
    }
}

impl<H: Digest<OutputSize = U64> + Default> From<&[u8]> for IncrHash<RistrettoPoint, H> {
    fn from(bytes: &[u8]) -> Self {
        IncrHash {
            point: RistrettoPoint::hash_from_bytes::<H>(bytes),
            h: PhantomData,
        }
    }
}

impl<H> Display for IncrHash<RistrettoPoint, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.point.compress().as_bytes()))
    }
}

impl<H> Default for IncrHash<RistrettoPoint, H> {
    fn default() -> Self {
        IncrHash {
            point: RistrettoPoint::identity(),
            h: PhantomData,
        }
    }
}

impl<'a, 'b, H> Add<&'b IncrHash<RistrettoPoint, H>> for &'a IncrHash<RistrettoPoint, H> {
    type Output = IncrHash<RistrettoPoint, H>;

    fn add(self, other: &'b IncrHash<RistrettoPoint, H>) -> IncrHash<RistrettoPoint, H> {
        IncrHash {
            point: self.point + other.point,
            h: PhantomData,
        }
    }
}

define_add_variants!(LHS = IncrHash<RistrettoPoint, blake2::Blake2b512>, RHS = IncrHash<RistrettoPoint, blake2::Blake2b512>, Output = IncrHash<RistrettoPoint, blake2::Blake2b512>);

impl<'b, H> AddAssign<&'b IncrHash<RistrettoPoint, H>> for IncrHash<RistrettoPoint, H> {
    fn add_assign(&mut self, _rhs: &IncrHash<RistrettoPoint, H>) {
        self.point += _rhs.point;
    }
}

define_add_assign_variants!(LHS = IncrHash<RistrettoPoint, blake2::Blake2b512>, RHS = IncrHash<RistrettoPoint, blake2::Blake2b512>);

impl<'a, 'b, H> Sub<&'b IncrHash<RistrettoPoint, H>> for &'a IncrHash<RistrettoPoint, H> {
    type Output = IncrHash<RistrettoPoint, H>;

    fn sub(self, other: &'b IncrHash<RistrettoPoint, H>) -> IncrHash<RistrettoPoint, H> {
        IncrHash {
            point: self.point - other.point,
            h: PhantomData,
        }
    }
}

define_sub_variants!(LHS = IncrHash<RistrettoPoint, blake2::Blake2b512>, RHS = IncrHash<RistrettoPoint, blake2::Blake2b512>, Output = IncrHash<RistrettoPoint, blake2::Blake2b512>);

impl<'b, H> SubAssign<&'b IncrHash<RistrettoPoint, H>> for IncrHash<RistrettoPoint, H> {
    fn sub_assign(&mut self, _rhs: &IncrHash<RistrettoPoint, H>) {
        self.point -= _rhs.point;
    }
}

define_sub_assign_variants!(LHS = IncrHash<RistrettoPoint, blake2::Blake2b512>, RHS = IncrHash<RistrettoPoint, blake2::Blake2b512>);

#[cfg(test)]
mod tests {
    use crate::ristretto::RistBlakeIncHash;

    #[test]
    fn bvt() {
        let mut h1: RistBlakeIncHash = RistBlakeIncHash::default();
        let h2: RistBlakeIncHash = RistBlakeIncHash::default();
        assert_eq!(h1, h2);

        assert_eq!(
            RistBlakeIncHash::from(b"hello world".as_slice()),
            RistBlakeIncHash::from(b"hello world".as_slice())
        );

        let a = RistBlakeIncHash::from(b"hello world".as_slice());
        let b = RistBlakeIncHash::from(b"sup universe".as_slice());

        let mut c = &a + &b;
        h1 += &a;
        h1 += &b;
        assert_eq!(c, h1);

        c -= &a;
        c -= &b;
        assert_eq!(c, RistBlakeIncHash::default());
    }

    #[test]
    fn commutes() {
        let a = RistBlakeIncHash::from(b"hello world".as_slice());
        let b = RistBlakeIncHash::from(b"sup universe".as_slice());
        let c = RistBlakeIncHash::from(b"peace".as_slice());

        let h1 = &a + &b + &c;
        let h2 = &c + &b + &a;
        let h3 = &c + &a + &b;

        assert_eq!(h1, h2);
        assert_eq!(h1, h3);
    }
}
