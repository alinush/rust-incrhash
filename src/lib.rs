use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::ristretto::RistrettoPoint;
use digest::consts::U64;
use digest::Digest;
use std::marker::PhantomData;

pub trait HashableFromBytes {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self;
}

impl HashableFromBytes for RistrettoPoint {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self {
        RistrettoPoint::hash_from_bytes::<H>(bytes)
    }
}

impl HashableFromBytes for EdwardsPoint {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self {
        EdwardsPoint::hash_from_bytes::<H>(bytes)
    }
}

// TODO: I need +, -, +=, -= operators on this
pub struct IncrHash<P, H> {
    point: P,
    h: PhantomData<H>,
}

impl<H: Digest<OutputSize = U64> + Default> From<&[u8]> for IncrHash<RistrettoPoint, H> {
    fn from(bytes: &[u8]) -> Self {
        IncrHash {
            point: RistrettoPoint::hash_from_bytes::<H>(bytes),
            h: PhantomData,
        }
    }
}

impl<H> Default for IncrHash<RistrettoPoint, H> {
    fn default() -> Self {
        IncrHash {
            point: RISTRETTO_BASEPOINT_POINT,
            h: PhantomData,
        }
    }
}
