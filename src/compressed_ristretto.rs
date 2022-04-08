use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ops::{AddAssign, SubAssign};

use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::traits::Identity;
use digest::consts::U64;
use digest::Digest;

use crate::{HashableFromBytes, IncrHash};

pub type CompRistBlakeIncHash = IncrHash<CompressedRistretto, blake2::Blake2b512>;

impl HashableFromBytes for CompressedRistretto {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self {
        RistrettoPoint::hash_from_bytes::<H>(bytes).compress()
    }
}

impl<H> Display for IncrHash<CompressedRistretto, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.point.as_bytes()))
    }
}

impl<H: Digest<OutputSize = U64> + Default> From<IncrHash<RistrettoPoint, H>>
    for IncrHash<CompressedRistretto, H>
{
    fn from(h: IncrHash<RistrettoPoint, H>) -> Self {
        IncrHash {
            point: h.point.compress(),
            h: Default::default(),
        }
    }
}

impl<H: Digest<OutputSize = U64> + Default> From<&IncrHash<RistrettoPoint, H>>
    for IncrHash<CompressedRistretto, H>
{
    fn from(h: &IncrHash<RistrettoPoint, H>) -> Self {
        IncrHash {
            point: h.point.compress(),
            h: Default::default(),
        }
    }
}

impl<H: Digest<OutputSize = U64> + Default> From<&[u8]> for IncrHash<CompressedRistretto, H> {
    fn from(bytes: &[u8]) -> Self {
        IncrHash {
            point: CompressedRistretto::hash_from_bytes::<H>(bytes),
            h: PhantomData,
        }
    }
}

impl<H> Default for IncrHash<CompressedRistretto, H> {
    fn default() -> Self {
        IncrHash {
            point: CompressedRistretto::identity(),
            h: PhantomData,
        }
    }
}

// NOTE(Alin): We do not define + operators since we would have to decompress and recompress points
// which would not be effective enough.
// In practice, we should hash to ristretto points, add the ristretto points and recompress at the end.
//
// We will need CompressedRistretto += RistrettoPoint operators if we want to ever accumulate such
// additions though.

// impl<'a, 'b, H> Add<&'b IncrHash<CompressedRistretto, H>> for &'a IncrHash<CompressedRistretto, H> {
//     type Output = IncrHash<CompressedRistretto, H>;
//
//     fn add(self, other: &'b IncrHash<CompressedRistretto, H>) -> IncrHash<CompressedRistretto, H> {
//         IncrHash {
//             point: self.point + other.point,
//             h: PhantomData,
//         }
//     }
// }

// define_add_variants!(LHS = IncrHash<CompressedRistretto, blake2::Blake2b512>, RHS = IncrHash<CompressedRistretto, blake2::Blake2b512>, Output = IncrHash<CompressedRistretto, blake2::Blake2b512>);

// impl<'a, 'b, H> Sub<&'b IncrHash<CompressedRistretto, H>> for &'a IncrHash<CompressedRistretto, H> {
//     type Output = IncrHash<CompressedRistretto, H>;
//
//     fn sub(self, other: &'b IncrHash<CompressedRistretto, H>) -> IncrHash<CompressedRistretto, H> {
//         IncrHash {
//             point: self.point - other.point,
//             h: PhantomData,
//         }
//     }
// }
//
// define_sub_variants!(LHS = IncrHash<CompressedRistretto, blake2::Blake2b512>, RHS = IncrHash<CompressedRistretto, blake2::Blake2b512>, Output = IncrHash<CompressedRistretto, blake2::Blake2b512>);

// NOTE(Alin): We do define a += operator since we want to accumulate additions of ristretto hashes inside a compressed ristretto hash
impl<'b, H> AddAssign<&'b IncrHash<RistrettoPoint, H>> for IncrHash<CompressedRistretto, H> {
    fn add_assign(&mut self, rhs: &IncrHash<RistrettoPoint, H>) {
        let mut decompressed = self.point.decompress().unwrap();
        decompressed += rhs.point;
        self.point = decompressed.compress();
    }
}

define_add_assign_variants!(
    LHS = IncrHash<CompressedRistretto, blake2::Blake2b512>,
    RHS = IncrHash<RistrettoPoint, blake2::Blake2b512>);

impl<'b, H> SubAssign<&'b IncrHash<RistrettoPoint, H>> for IncrHash<CompressedRistretto, H> {
    fn sub_assign(&mut self, rhs: &IncrHash<RistrettoPoint, H>) {
        let mut decompressed = self.point.decompress().unwrap();
        decompressed -= rhs.point;
        self.point = decompressed.compress();
    }
}

define_sub_assign_variants!(
    LHS = IncrHash<CompressedRistretto, blake2::Blake2b512>,
    RHS = IncrHash<RistrettoPoint, blake2::Blake2b512>);

#[cfg(test)]
mod tests {
    use crate::compressed_ristretto::CompRistBlakeIncHash;
    use crate::ristretto::RistBlakeIncHash;
    use std::mem::size_of;

    #[test]
    fn small_size() {
        let size = size_of::<CompRistBlakeIncHash>();
        println!("CompressedRistretto incremental hashes are {} bytes", size);
        assert_eq!(size, 32);
    }

    #[test]
    fn bvt() {
        let mut h1: CompRistBlakeIncHash = CompRistBlakeIncHash::default();
        let h2: CompRistBlakeIncHash = CompRistBlakeIncHash::default();
        assert_eq!(h1, h2);

        assert_eq!(
            CompRistBlakeIncHash::from(b"hello world".as_slice()),
            CompRistBlakeIncHash::from(b"hello world".as_slice())
        );

        let a = RistBlakeIncHash::from(b"hello world".as_slice());
        let b = RistBlakeIncHash::from(b"sup universe".as_slice());

        let c = &a + &b;
        h1 = CompRistBlakeIncHash::default();
        h1 += &c;
        assert_eq!(CompRistBlakeIncHash::from(&c), h1);

        h1 -= &c;
        assert_eq!(h1, CompRistBlakeIncHash::default());
    }
}
