use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::panic::{catch_unwind, AssertUnwindSafe};
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type1::{Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use verange_proof::type2::{Type2Prover, Type2Statement, Type2Verifier, Type2Witness};
use verange_proof::type2p::{Type2PProver, Type2PStatement, Type2PVerifier, Type2PWitness};
use verange_proof::type3::{Type3Prover, Type3Statement, Type3Verifier, Type3Witness};
use verange_proof::type4_batch::{
    Type4BatchProver, Type4BatchStatement, Type4BatchVerifier, Type4BatchWitness,
};
use verange_proof::ProofError;

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

fn pow_biguint(base: usize, exp: usize) -> BigUint {
    BigUint::from(base as u64).pow(exp as u32)
}

fn random_biguint_below(limit: &BigUint, rng: &mut impl RngCore) -> BigUint {
    if limit == &BigUint::from(0u64) {
        return BigUint::from(0u64);
    }
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    BigUint::from_bytes_be(&bytes) % limit
}

fn assert_invalid_statement<T>(result: Result<T, ProofError>) {
    assert!(matches!(result, Err(ProofError::InvalidStatement(_))));
}

fn assert_rejects_without_panic<F>(verify_fn: F)
where
    F: FnOnce() -> Result<bool, ProofError>,
{
    let result = catch_unwind(AssertUnwindSafe(verify_fn));
    assert!(result.is_ok(), "verifier panicked on malformed input");
    let verdict = result.expect("checked above");
    assert!(matches!(verdict, Err(_) | Ok(false)));
}

#[test]
fn property_tests_single_field_mutation_fails() {
    let mut rng = ChaCha20Rng::from_seed([201u8; 32]);

    let params_1 = sample_params(4);
    let stmt_1 = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let wit_1 = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };
    let mut proof_1 = Type1Prover::prove(
        &stmt_1,
        &wit_1,
        &params_1,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type1");
    proof_1.eta1 += Fr::from(1u64);
    assert!(
        !Type1Verifier::verify(&stmt_1, &proof_1, &params_1, TranscriptMode::JavaCompat)
            .expect("verify type1")
    );

    let params_2 = sample_params(4);
    let stmt_2 = Type2Statement {
        nbits: 8,
        k: 2,
        l: 4,
        b: 4,
        tt: 1,
        aggregated: false,
    };
    let wit_2 = Type2Witness {
        values: vec![BigUint::from(99u32)],
    };
    let mut proof_2 = Type2Prover::prove(
        &stmt_2,
        &wit_2,
        &params_2,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2");
    proof_2.eta2 += Fr::from(1u64);
    assert!(
        !Type2Verifier::verify(&stmt_2, &proof_2, &params_2, TranscriptMode::JavaCompat)
            .expect("verify type2")
    );

    let params_2p = sample_params(4);
    let stmt_2p = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let wit_2p = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let mut proof_2p = Type2PProver::prove(
        &stmt_2p,
        &wit_2p,
        &params_2p,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2p");
    proof_2p.eta1 += Fr::from(1u64);
    assert!(
        !Type2PVerifier::verify(
            &stmt_2p,
            &proof_2p,
            &params_2p,
            TranscriptMode::JavaCompat
        )
            .expect("verify type2p")
    );

    let params_3 = sample_params(4);
    let stmt_3 = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let wit_3 = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };
    let mut proof_3 = Type3Prover::prove(
        &stmt_3,
        &wit_3,
        &params_3,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type3");
    proof_3.eta2 += Fr::from(1u64);
    assert!(
        !Type3Verifier::verify(&stmt_3, &proof_3, &params_3, TranscriptMode::JavaCompat)
            .expect("verify type3")
    );

    let params_4 = sample_params(32);
    let stmt_4 = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let wit_4 = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let mut proof_4 = Type4BatchProver::prove(
        &stmt_4,
        &wit_4,
        &params_4,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type4_batch");
    proof_4.rprime_2 += Fr::from(1u64);
    assert!(
        !Type4BatchVerifier::verify(&stmt_4, &proof_4, &params_4, TranscriptMode::JavaCompat)
            .expect("verify type4_batch")
    );
}

#[test]
fn property_tests_random_valid_instances_verify() {
    let mut rng = ChaCha20Rng::from_seed([221u8; 32]);

    for _ in 0..6 {
        let (params, statement, witness_count, digits_per_witness) = if rng.next_u32() % 2 == 0 {
            (
                sample_params(4),
                Type1Statement {
                    nbits: 8,
                    k: 2,
                    tt: 1,
                    aggregated: false,
                },
                1usize,
                8usize,
            )
        } else {
            (
                sample_params(4),
                Type1Statement {
                    nbits: 16,
                    k: 4,
                    tt: 2,
                    aggregated: true,
                },
                2usize,
                8usize,
            )
        };
        let limit = pow_biguint(2, digits_per_witness);
        let witness = Type1Witness {
            values: (0..witness_count)
                .map(|_| random_biguint_below(&limit, &mut rng))
                .collect(),
        };
        let proof = Type1Prover::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove random type1");
        assert!(
            Type1Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify random type1")
        );
    }

    for _ in 0..6 {
        let (params, statement, witness_count, digits_per_witness) = if rng.next_u32() % 2 == 0 {
            (
                sample_params(4),
                Type2Statement {
                    nbits: 8,
                    k: 2,
                    l: 4,
                    b: 4,
                    tt: 1,
                    aggregated: false,
                },
                1usize,
                8usize,
            )
        } else {
            (
                sample_params(4),
                Type2Statement {
                    nbits: 16,
                    k: 4,
                    l: 4,
                    b: 8,
                    tt: 2,
                    aggregated: true,
                },
                2usize,
                8usize,
            )
        };
        let limit = pow_biguint(statement.b, digits_per_witness);
        let witness = Type2Witness {
            values: (0..witness_count)
                .map(|_| random_biguint_below(&limit, &mut rng))
                .collect(),
        };
        let proof = Type2Prover::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove random type2");
        assert!(
            Type2Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify random type2")
        );
    }

    for _ in 0..6 {
        let statement = if rng.next_u32() % 2 == 0 {
            Type2PStatement {
                nbits: 12,
                k: 3,
                l: 4,
                b: 8,
                tt: 1,
                aggregated: false,
            }
        } else {
            Type2PStatement {
                nbits: 16,
                k: 4,
                l: 4,
                b: 8,
                tt: 1,
                aggregated: false,
            }
        };
        let params = sample_params(statement.l);
        let limit = pow_biguint(statement.b, statement.nbits);
        let witness = Type2PWitness {
            values: vec![random_biguint_below(&limit, &mut rng)],
        };
        let proof = Type2PProver::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove random type2p");
        assert!(
            Type2PVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify random type2p")
        );
    }

    for _ in 0..6 {
        let (params, statement, witness_count, digits_per_witness) = if rng.next_u32() % 2 == 0 {
            (
                sample_params(4),
                Type3Statement {
                    nbits: 8,
                    u: 4,
                    v: 4,
                    b: 4,
                    tt: 1,
                    aggregated: false,
                },
                1usize,
                8usize,
            )
        } else {
            (
                sample_params(4),
                Type3Statement {
                    nbits: 16,
                    u: 4,
                    v: 4,
                    b: 8,
                    tt: 2,
                    aggregated: true,
                },
                2usize,
                8usize,
            )
        };
        let limit = pow_biguint(statement.b, digits_per_witness);
        let witness = Type3Witness {
            values: (0..witness_count)
                .map(|_| random_biguint_below(&limit, &mut rng))
                .collect(),
        };
        let proof = Type3Prover::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove random type3");
        assert!(
            Type3Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify random type3")
        );
    }

    for _ in 0..6 {
        let params = sample_params(32);
        let statement = Type4BatchStatement {
            nbits: 16,
            k: 4,
            l: 32,
            b: 8,
        };
        let limit = pow_biguint(statement.b, statement.nbits);
        let witness = Type4BatchWitness {
            value: random_biguint_below(&limit, &mut rng),
        };
        let proof = Type4BatchProver::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove random type4_batch");
        assert!(
            Type4BatchVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify random type4_batch")
        );
    }
}

#[test]
fn property_tests_malformed_shape_rejects_without_panic() {
    let mut rng = ChaCha20Rng::from_seed([231u8; 32]);

    let params_1 = sample_params(4);
    let stmt_1 = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let wit_1 = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };
    let proof_1 = Type1Prover::prove(
        &stmt_1,
        &wit_1,
        &params_1,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type1");
    let mut bad_1 = proof_1.clone();
    bad_1.cws.pop();
    assert_rejects_without_panic(|| {
        Type1Verifier::verify(&stmt_1, &bad_1, &params_1, TranscriptMode::JavaCompat)
    });

    let params_2 = sample_params(4);
    let stmt_2 = Type2Statement {
        nbits: 8,
        k: 2,
        l: 4,
        b: 4,
        tt: 1,
        aggregated: false,
    };
    let wit_2 = Type2Witness {
        values: vec![BigUint::from(99u32)],
    };
    let proof_2 = Type2Prover::prove(
        &stmt_2,
        &wit_2,
        &params_2,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2");
    let mut bad_2 = proof_2.clone();
    bad_2.us.pop();
    assert_rejects_without_panic(|| {
        Type2Verifier::verify(&stmt_2, &bad_2, &params_2, TranscriptMode::JavaCompat)
    });

    let params_2p = sample_params(4);
    let stmt_2p = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let wit_2p = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let proof_2p = Type2PProver::prove(
        &stmt_2p,
        &wit_2p,
        &params_2p,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2p");
    let mut bad_2p = proof_2p.clone();
    bad_2p.vs.pop();
    assert_rejects_without_panic(|| {
        Type2PVerifier::verify(&stmt_2p, &bad_2p, &params_2p, TranscriptMode::JavaCompat)
    });

    let params_3 = sample_params(4);
    let stmt_3 = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let wit_3 = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };
    let proof_3 = Type3Prover::prove(
        &stmt_3,
        &wit_3,
        &params_3,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type3");
    let mut bad_3 = proof_3.clone();
    bad_3.c_w.pop();
    assert_rejects_without_panic(|| {
        Type3Verifier::verify(&stmt_3, &bad_3, &params_3, TranscriptMode::JavaCompat)
    });

    let params_4 = sample_params(32);
    let stmt_4 = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let wit_4 = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let proof_4 = Type4BatchProver::prove(
        &stmt_4,
        &wit_4,
        &params_4,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type4_batch");
    let mut bad_4 = proof_4.clone();
    bad_4.com_q.pop();
    assert_rejects_without_panic(|| {
        Type4BatchVerifier::verify(&stmt_4, &bad_4, &params_4, TranscriptMode::JavaCompat)
    });
}

#[test]
fn property_tests_statement_invariants_are_enforced() {
    let mut rng = ChaCha20Rng::from_seed([241u8; 32]);

    let params_1 = sample_params(4);
    let valid_1 = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let witness_1 = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };
    let proof_1 = Type1Prover::prove(
        &valid_1,
        &witness_1,
        &params_1,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove valid type1");
    let invalid_1 = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 2,
        aggregated: false,
    };
    assert_invalid_statement(Type1Prover::prove(
        &invalid_1,
        &witness_1,
        &params_1,
        TranscriptMode::JavaCompat,
        &mut rng,
    ));
    assert_invalid_statement(Type1Verifier::verify(
        &invalid_1,
        &proof_1,
        &params_1,
        TranscriptMode::JavaCompat,
    ));

    let params_2 = sample_params(4);
    let valid_2 = Type2Statement {
        nbits: 8,
        k: 2,
        l: 4,
        b: 4,
        tt: 1,
        aggregated: false,
    };
    let witness_2 = Type2Witness {
        values: vec![BigUint::from(99u32)],
    };
    let proof_2 = Type2Prover::prove(
        &valid_2,
        &witness_2,
        &params_2,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove valid type2");
    let invalid_2 = Type2Statement {
        nbits: 8,
        k: 2,
        l: 4,
        b: 4,
        tt: 2,
        aggregated: false,
    };
    assert_invalid_statement(Type2Prover::prove(
        &invalid_2,
        &witness_2,
        &params_2,
        TranscriptMode::JavaCompat,
        &mut rng,
    ));
    assert_invalid_statement(Type2Verifier::verify(
        &invalid_2,
        &proof_2,
        &params_2,
        TranscriptMode::JavaCompat,
    ));

    let params_2p = sample_params(4);
    let valid_2p = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let witness_2p = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let proof_2p = Type2PProver::prove(
        &valid_2p,
        &witness_2p,
        &params_2p,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove valid type2p");
    let invalid_2p = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 2,
        aggregated: false,
    };
    assert_invalid_statement(Type2PProver::prove(
        &invalid_2p,
        &witness_2p,
        &params_2p,
        TranscriptMode::JavaCompat,
        &mut rng,
    ));
    assert_invalid_statement(Type2PVerifier::verify(
        &invalid_2p,
        &proof_2p,
        &params_2p,
        TranscriptMode::JavaCompat,
    ));

    let params_3 = sample_params(4);
    let valid_3 = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let witness_3 = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };
    let proof_3 = Type3Prover::prove(
        &valid_3,
        &witness_3,
        &params_3,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove valid type3");
    let invalid_3 = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: false,
    };
    assert_invalid_statement(Type3Prover::prove(
        &invalid_3,
        &Type3Witness {
            values: vec![BigUint::from(131u32)],
        },
        &params_3,
        TranscriptMode::JavaCompat,
        &mut rng,
    ));
    assert_invalid_statement(Type3Verifier::verify(
        &invalid_3,
        &proof_3,
        &params_3,
        TranscriptMode::JavaCompat,
    ));

    let params_4 = sample_params(32);
    let valid_4 = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let witness_4 = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let proof_4 = Type4BatchProver::prove(
        &valid_4,
        &witness_4,
        &params_4,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove valid type4_batch");
    let invalid_4 = Type4BatchStatement {
        nbits: 16,
        k: 2,
        l: 32,
        b: 8,
    };
    assert_invalid_statement(Type4BatchProver::prove(
        &invalid_4,
        &witness_4,
        &params_4,
        TranscriptMode::JavaCompat,
        &mut rng,
    ));
    assert_invalid_statement(Type4BatchVerifier::verify(
        &invalid_4,
        &proof_4,
        &params_4,
        TranscriptMode::JavaCompat,
    ));
}

#[test]
fn property_tests_type1_witness_boundaries() {
    let params = sample_params(4);
    let statement = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };

    for value in [BigUint::from(0u32), BigUint::from(255u32)] {
        let witness = Type1Witness {
            values: vec![value],
        };
        let mut rng = ChaCha20Rng::from_seed([211u8; 32]);
        let proof = Type1Prover::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove type1 boundary");
        assert!(
            Type1Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify type1 boundary")
        );
    }
}
