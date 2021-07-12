# deterministic-hash
Tiny Rust library to create deterministic hashes regardless of architecture. This library is `no-std` compatible and uses no allocations or dependencies.

The default `core::hash::Hasher` implementation ensures a platform dependant hashing of datastructures that use `#[derive(Hash)]`. Most notably by:
* using `to_ne_bytes` for `u{8,16,32,64,128}`.
* using the native bytelength of `usize`.

The `DeterministicHasher` of this library forces the use of `to_le_bytes` and casts `usize` to `u64` regardless of your platform. Hence the hasher will be less efficient, but will be deterministic when using the same library in different architecture contexts. I use a common dataprotocol library both on ARM embedded systems, wasm and x64.

You can validate the operation of this library with `cross` by running:

```bash
cargo install cross
cross test --target=x86_64-unknown-linux-gnu
cross test --target=aarch64-unknown-linux-gnu
cross test --target=arm-unknown-linux-gnueabihf
```