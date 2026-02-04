//! Example: Generate sample payloads for NebulaVRF contract
//!
//! This example generates seed, salt, commitment, BLS keys, and signature
//! that can be used to test the deployed contract on Stellar Testnet.
//!
//! Usage:
//!   cargo run --example sample_payloads
//!   cargo run --example sample_payloads -- --seed-len 16 --salt-len 16

use nebula_vrf::SamplePayload;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let seed_len = if args.len() > 2 && args[1] == "--seed-len" {
        args[2].parse().unwrap_or(8)
    } else {
        8
    };
    
    let salt_len = if args.len() > 4 && args[3] == "--salt-len" {
        args[4].parse().unwrap_or(8)
    } else {
        8
    };

    println!("=== NebulaVRF Sample Payload Generator ===\n");
    println!("Generating payload with seed_len={}, salt_len={}\n", seed_len, salt_len);

    // Generate the test payload
    let payload = SamplePayload::generate(seed_len, salt_len)
        .expect("Failed to generate test payload");

    // Verify the signature is valid
    payload.verify()
        .expect("Generated payload failed verification");

    println!("Payload generated and verified!\n");
    println!("=== HEX FORMAT (for CLI) ===\n");
    println!("Seed (hex): {}", payload.seed_hex());
    println!("Salt (hex): {}", payload.salt_hex());
    println!("Commitment (hex): {}", payload.commitment_hex());
    println!("Pubkey (hex, {} bytes): {}", payload.pubkey.len(), payload.pubkey_hex());
    println!("Signature (hex, {} bytes): {}", payload.signature.len(), payload.signature_hex());

    println!("\n=== BASE64 FORMAT (for Stellar Lab) ===\n");
    println!("Seed (base64): {}", payload.seed_base64());
    println!("Salt (base64): {}", payload.salt_base64());
    println!("Commitment (base64): {}", payload.commitment_base64());
    println!("Pubkey (base64, {} bytes): {}", payload.pubkey.len(), payload.pubkey_base64());
    println!("Signature (base64, {} bytes): {}", payload.signature.len(), payload.signature_base64());

    println!("\n=== STELLAR LAB VALUES ===\n");
    println!("For commit():");
    println!("  user: <YOUR_USER_ADDRESS>");
    println!("  commitment: {}", payload.commitment_base64());
    println!("  pubkey: {}", payload.pubkey_base64());
    
    println!("\nFor reveal():");
    println!("  user: <YOUR_USER_ADDRESS>");
    println!("  seed: {}", payload.seed_base64());
    println!("  salt: {}", payload.salt_base64());
    println!("  signature: {}", payload.signature_base64());

    println!("\n=== VERIFICATION ===\n");
    println!("Commitment matches: sha256(seed || salt)");
    println!("Signature verified against commitment");
    println!("All values ready for contract testing!");
}
