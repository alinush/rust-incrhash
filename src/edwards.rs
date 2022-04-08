use curve25519_dalek::edwards::EdwardsPoint;
use digest::consts::U64;
use digest::Digest;

use crate::HashableFromBytes;

impl HashableFromBytes for EdwardsPoint {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self {
        EdwardsPoint::hash_from_bytes::<H>(bytes)
    }
}
