//! Statistical tests for the uniformity of VRF output randomness.

use nebula_vrf::vrf::generate_random;
use statrs::distribution::{ChiSquared, ContinuousCDF};
use sha2::{Sha256, Digest};

#[test]
fn test_chi_square_randomness_uniformity() {
    const NUM_SAMPLES: usize = 5000;
    const BIN_COUNT: usize = 256;
    let mut bins = [0u64; BIN_COUNT];

    // Collect byte frequencies from the hash of the VRF output
    for i in 0..NUM_SAMPLES {
        let seed = format!("seed-{}", i);
        let vrf = generate_random(seed.as_bytes()).expect("generation failed");
        let hash = Sha256::digest(&vrf.output);
        for byte in hash.iter() {
            bins[*byte as usize] += 1;
        }
    }

    let expected = (NUM_SAMPLES * 32) as f64 / BIN_COUNT as f64;

    // Compute Chi-Square statistic
    let chi_square_stat: f64 = bins.iter()
        .map(|&obs| {
            let diff = obs as f64 - expected;
            (diff * diff) / expected
        })
        .sum();

    let df = BIN_COUNT as f64 - 1.0;
    let chi_dist = ChiSquared::new(df).unwrap();
    let p_value = 1.0 - chi_dist.cdf(chi_square_stat);

    println!("Chi² statistic: {}", chi_square_stat);
    println!("p-value: {}", p_value);

    // 99% confidence: p-value must be > 0.01 to accept null hypothesis
    assert!(
        p_value > 0.01,
        "Chi-square test failed: p-value = {}, randomness not uniform", p_value
    );
} 