use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_sdk::{
    Parameters, Prover, Type1Statement, Type1Witness, Type2PStatement, Type2PWitness,
    Type2Statement, Type2Witness, Type3Statement, Type3Witness, Type4BatchStatement,
    Type4BatchWitness, Verifier,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params_4 = Parameters::bn254_java_compat(4)?;
    let prover_4 = Prover::new(params_4.clone(), TranscriptMode::JavaCompat);
    let verifier_4 = Verifier::new(params_4, TranscriptMode::JavaCompat);

    let mut rng = ChaCha20Rng::from_seed([1u8; 32]);

    let type1_stmt = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let type1_wit = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };
    let type1_proof = prover_4.prove_type1(&type1_stmt, &type1_wit, &mut rng)?;
    println!("type1 verify = {}", verifier_4.verify_type1(&type1_stmt, &type1_proof)?);

    let type2_stmt = Type2Statement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let type2_wit = Type2Witness {
        values: vec![BigUint::from(181u32), BigUint::from(77u32)],
    };
    let type2_proof = prover_4.prove_type2(&type2_stmt, &type2_wit, &mut rng)?;
    println!("type2 verify = {}", verifier_4.verify_type2(&type2_stmt, &type2_proof)?);

    let type2p_stmt = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let type2p_wit = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let type2p_proof = prover_4.prove_type2p(&type2p_stmt, &type2p_wit, &mut rng)?;
    println!(
        "type2p verify = {}",
        verifier_4.verify_type2p(&type2p_stmt, &type2p_proof)?
    );

    let type3_stmt = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let type3_wit = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };
    let type3_proof = prover_4.prove_type3(&type3_stmt, &type3_wit, &mut rng)?;
    println!("type3 verify = {}", verifier_4.verify_type3(&type3_stmt, &type3_proof)?);

    // Type4_batch needs a larger basis (l == params.gs.len()).
    let params_32 = Parameters::bn254_java_compat(32)?;
    let prover_32 = Prover::new(params_32.clone(), TranscriptMode::JavaCompat);
    let verifier_32 = Verifier::new(params_32, TranscriptMode::JavaCompat);

    let type4_stmt = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let type4_wit = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let type4_proof = prover_32.prove_type4_batch(&type4_stmt, &type4_wit, &mut rng)?;
    println!(
        "type4_batch verify = {}",
        verifier_32.verify_type4_batch(&type4_stmt, &type4_proof)?
    );

    Ok(())
}
