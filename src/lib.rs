//! Tiny Rust library to create deterministic hashes regardless of architecture. This library is `no-std` compatible and uses no allocations or dependencies.
//!
//! The default `core::hash::Hasher` implementation ensures a platform dependant hashing of datastructures that use `#[derive(Hash)]`. Most notably by:
//! * using `to_ne_bytes` for `u{8,16,32,64,128}`.
//! * using the native bytelength of `usize`.
//!
//! The `DeterministicHasher` of this library forces the use of `to_le_bytes` and casts `usize` to `u64` regardless of your platform. Hence the hasher will be less efficient, but will be deterministic when using the same library in different architecture contexts. I use a common dataprotocol library both on ARM embedded systems, wasm and x64.
//!
//! From any hasher make it deterministic by inserting `DeterministicHasher` in between:
//! ```
//! let hasher = crc::crc32::Digest::new(crc::crc32::KOOPMAN);
//! let hasher = deterministic_hash::DeterministicHasher::new(hasher);
//! ```

#![no_std]
use core::hash::Hasher;

/// Wrapper around any hasher to make it deterministic.
///
/// ```
/// use core::hash::Hash;
/// use crc::crc32::Hasher32;
/// use deterministic_hash::DeterministicHasher;
/// let mut hasher = DeterministicHasher::new(crc::crc32::Digest::new(crc::crc32::KOOPMAN));
/// (0x1337 as usize).hash(&mut hasher);
/// assert_eq!(hasher.as_inner().sum32(), 2482448842);
/// ```
pub struct DeterministicHasher<T: Hasher>(T);

impl<T: Hasher> DeterministicHasher<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn as_inner(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Implementation of hasher that forces all bytes written to be platform agnostic.
impl<T: Hasher> core::hash::Hasher for DeterministicHasher<T> {
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes);
    }

    fn write_u8(&mut self, i: u8) {
        self.write(&i.to_le_bytes())
    }

    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_le_bytes())
    }

    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_le_bytes())
    }

    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_le_bytes())
    }

    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_le_bytes())
    }

    fn write_usize(&mut self, i: usize) {
        self.write(&(i as u64).to_le_bytes())
    }

    fn write_i8(&mut self, i: i8) {
        self.write_u8(i as u8)
    }

    fn write_i16(&mut self, i: i16) {
        self.write_u16(i as u16)
    }

    fn write_i32(&mut self, i: i32) {
        self.write_u32(i as u32)
    }

    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64)
    }

    fn write_i128(&mut self, i: i128) {
        self.write_u128(i as u128)
    }

    fn write_isize(&mut self, i: isize) {
        self.write_usize(i as usize)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
