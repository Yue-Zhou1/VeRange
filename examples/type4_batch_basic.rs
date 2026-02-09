use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type4_batch::{
    Type4BatchProver, Type4BatchStatement, Type4BatchVerifier, Type4BatchWitness,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..32)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    let params = PedersenParams::new(g, h, gs)?;

    // Type4_batch constraints:
    // - nbits is a power of two
    // - b * nbits is a power of two
    // - l == params.gs.len()
    let statement = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let witness = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };

    let mut rng = ChaCha20Rng::from_seed([99u8; 32]);
    let proof = Type4BatchProver::prove(
        &statement,
        &witness,
        &params,
        TranscriptMode::JavaCompat,
        &mut rng,
    )?;
    println!(
        "verify = {}",
        Type4BatchVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)?
    );
    Ok(())
}
