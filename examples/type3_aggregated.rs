use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type3::{Type3Prover, Type3Statement, Type3Verifier, Type3Witness};
use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..4)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    let params = PedersenParams::new(g, h, gs)?;

    let statement = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let witness = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([7u8; 32]);
    let proof = Type3Prover::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)?;
    println!("verify = {}", Type3Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)?);
    Ok(())
}
