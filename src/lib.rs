use std::fmt::Debug;
use std::marker::PhantomData;

use digest::consts::U64;
use digest::Digest;
use serde::{self, Deserialize, Serialize};

#[macro_use]
pub(crate) mod macros;
pub mod compressed_ristretto;
pub mod edwards;
pub mod ristretto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrHash<P, H> {
    point: P,
    h: PhantomData<H>,
}

pub trait HashableFromBytes {
    fn hash_from_bytes<H: Digest<OutputSize = U64> + Default>(bytes: &[u8]) -> Self;
}

impl<P: PartialEq + Debug, H> PartialEq for IncrHash<P, H> {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
