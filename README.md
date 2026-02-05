# NebulaVRF 

**NebulaVRF** is a **Verifiable Random Function (VRF)** core for Soroban developers. It transforms a secure seed into cryptographically sound, verifiable randomness without external oracles. You can generate randomness locally and, if needed, submit proofs for on-chain verification on testnet.

> **View full API documentation and live endpoint usage:** [api/docs.md](api/docs.md)

This crate is ideal for developers building dApps that need:
- Randomized game mechanics or loot drops
- Unbiased DAO voting and committee selection
- Fair token lotteries or airdrops
- NFT trait randomization
- Anything else that might require randomization

Built in pure Rust using the `blst` [BLS12-381 signature library](https://github.com/supranational/blst/blob/master/bindings/rust/README.md). Designed to be compiled to WASM and later integrated into Soroban smart contracts.

---

## Features

- Secure randomness generation using BLS signatures (min-sig mode)
- Proof of randomness—anyone can verify correctness
- Deterministic output (same seed → same randomness)
- Unpredictable if seed is kept secret
- Commit–reveal scheme to prevent last-mover bias

---

## Installation

Add `nebula-vrf` to your `Cargo.toml`:

```toml
[dependencies]
nebula-vrf = "0.1.4"
```

If you need the optional API features (for running the HTTP server), include the `api` feature:

```toml
[dependencies]
nebula-vrf = { version = "0.1.4", features = ["api"] }
```

Then use it in your code:

```rust
use nebula_vrf::vrf::generate_random;
use nebula_vrf::vrf::verify_proof;
use nebula_vrf::vrf::commit::{commit, verify_commit};
```

---

## How It Works

1. User generates a private random `seed` (not shared initially).
2. Hash of seed is committed on-chain (via the testnet contract) using `commit()`.
3. After a delay, the seed is revealed and passed into `generate_random()`.
4. The function returns:
   - `output` — a random-looking 48-byte value (BLS signature)
   - `public_key` — the proof of validity (BLS pubkey)
5. Others verify the output using `verify_proof()`.

### Testnet Contract Integration

The Soroban testnet contract lives in the `vrf-testnet` repo. It expects:
- `commitment = sha256(seed || salt)`
- **G1 pubkey** (96 bytes, uncompressed)
- **G2 signature** (192 bytes, uncompressed)
- **DST**: `NEBULA-VRF-V01-BLS12381G2`

---

## Types and Output Formats

| Function                      | Input Type         | Output Type                                         | Description                |
|-------------------------------|--------------------|-----------------------------------------------------|----------------------------|
| `generate_random(seed)`       | `&[u8]`           | `VRFProof { output: Vec<u8>, public_key: Vec<u8> }` | Main VRF output            |
| `verify_proof(seed, output, pubkey)` | `&[u8]`, byte arrays | `Result<(), VRFError>`                              | Verifies randomness        |
| `commit(seed)`                | `&[u8]`           | `[u8; 32]`                                          | SHA256-based hash of seed  |
| `verify_commit(seed, commitment)` | `&[u8]`, `&[u8; 32]` | `bool`                                         | Check if seed matches hash |

### Interpreting the Output

The output (`Vec<u8>` of 48 bytes) is cryptographically strong pseudorandomness. You can extract values like:

```rust
let output = &vrf.output;
let as_u64 = u64::from_le_bytes(output[0..8].try_into().unwrap());
let as_u128 = u128::from_le_bytes(output[0..16].try_into().unwrap());
let as_hash = sha2::Sha256::digest(output);
```

---

## Payload Generation (Testnet Helper)

If you are coming from the NebulaVRF testnet contract, you can generate
testnet‑compatible payloads (seed, salt, commitment, pubkey, signature) here.
These helpers are for **demo/testing** only — for real integrations you should
implement your own seed/salt generation logic.

Generate payloads:

```bash
cargo run --example sample_payloads
```

Run the local API:

```bash
cargo run --bin nebula_vrf_api --features api
```

Endpoints:
- `GET http://localhost:3000/payloads`
- `GET http://localhost:3000/payloads?seed_len=8&salt_len=8`


---

## Seed Generation 

Use this for secure randomness:

```rust
use rand::rngs::OsRng;
use rand::RngCore;

fn generate_secure_seed() -> [u8; 32] {
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);
    seed
}
```

- Never hardcode seeds
- Never reuse seeds
- Always commit before revealing

---


## Example Usage

### Generate Random Output

```rust
use nebula_vrf::vrf::generate_random;

let seed = b"user-supplied-entropy";
let vrf = generate_random(seed).unwrap();

println!("Randomness: {:?}", vrf.output);
println!("Proof (public key): {:?}", vrf.public_key);
```

### Verify Proof

```rust
use nebula_vrf::vrf::verify_proof;

let is_valid = verify_proof(seed, &vrf.output, &vrf.public_key).unwrap();
assert!(is_valid);
```

### Commit–Reveal Flow

```rust
use nebula_vrf::vrf::commit::{commit, verify_commit};

let seed = b"secret-seed";
let commitment = commit(seed);

// Save or store `commitment` on-chain
assert!(verify_commit(seed, &commitment));  // later when revealed
```
---

## Testing

Run tests:

```bash
cargo test -- --nocapture
```

Coverage:
- Determinism (same seed → same randomness)
- Collision resistance
- Corruption rejection (sig/pubkey tampering)
- Edge cases (empty seed)
- Commit-reveal validation

---

## Standards & Compliance

- Based on the [IETF CFRG VRF Draft Spec](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-vrf)
- Uses BLS12-381 signatures (min-sig mode)
- Domain separation follows [RFC 9380](https://datatracker.ietf.org/doc/html/rfc9380) format
- Similar to ETH2.0 / Chainlink / Polkadot randomness implementations

---

## Crate Structure

```
nebula-vrf/
├── src/
│   ├── lib.rs
│   ├── vrf/
│   │   ├── bls.rs        # Core BLS logic
│   │   ├── commit.rs     # Commit-reveal layer
│   │   └── types.rs      # Error types
│   └── utils/
│       └── hash.rs       # SHA256 utilities
├── tests/
│   └── vrf_tests.rs      # Full test suite
```

---

## FAQ

**Is the randomness deterministic?**  
Yes, same seed → same output. This allows verifiability.

**Can attackers predict the randomness?**  
Not if the seed is secret and committed before reveal.

**How many bits of randomness do I get?**  
384 bits (48 bytes) directly. You can truncate or hash to get `u64`, `u128`, `SHA256`, etc.

**Can this be used on-chain?**  
Yes. The testnet contract lives in the `vrf-testnet` repo and verifies proofs on-chain.

---

## License

MIT — Free for public and commercial use in Stellar/Soroban dApps.

---

## Author

Built by Mukund Jha  
[GitHub](https://github.com/NebulaVRF/vrf-core) | [https://nebulavrf.vercel.app](https://nebulavrf.vercel.app)  
Supported by Stellar Community Fund
