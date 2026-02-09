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

fn assert_no_panic_and_rejected(
    outcome: std::thread::Result<Result<bool, ProofError>>,
    rejected_counter: &mut usize,
) {
    assert!(
        outcome.is_ok(),
        "verifier panicked while processing mutated proof"
    );
    let verdict = outcome.expect("checked above");
    if matches!(verdict, Err(_) | Ok(false)) {
        *rejected_counter += 1;
    }
}

#[test]
fn fuzz_like_tests_mutated_proofs_do_not_panic_verifiers() {
    let mut rng = ChaCha20Rng::from_seed([251u8; 32]);

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
    let mut rejected_1 = 0usize;
    for _ in 0..128 {
        let mut mutated = proof_1.clone();
        match rng.next_u32() % 5 {
            0 => mutated.eta1 += Fr::from(rng.next_u64() | 1),
            1 => mutated.eta2 += Fr::from(rng.next_u64() | 1),
            2 => {
                if !mutated.cws.is_empty() {
                    mutated.cws.pop();
                }
            }
            3 => {
                if !mutated.vs.is_empty() {
                    mutated.vs.pop();
                }
            }
            _ => {
                if !mutated.ys.is_empty() {
                    mutated.ys.pop();
                }
            }
        }
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            Type1Verifier::verify(&stmt_1, &mutated, &params_1, TranscriptMode::JavaCompat)
        }));
        assert_no_panic_and_rejected(outcome, &mut rejected_1);
    }
    assert!(rejected_1 > 0, "mutated type1 proofs should be rejected");

    let params_2 = sample_params(4);
    let stmt_2 = Type2Statement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let wit_2 = Type2Witness {
        values: vec![BigUint::from(181u32), BigUint::from(77u32)],
    };
    let proof_2 = Type2Prover::prove(
        &stmt_2,
        &wit_2,
        &params_2,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2");
    let mut rejected_2 = 0usize;
    for _ in 0..128 {
        let mut mutated = proof_2.clone();
        match rng.next_u32() % 5 {
            0 => mutated.eta1 += Fr::from(rng.next_u64() | 1),
            1 => mutated.eta2 += Fr::from(rng.next_u64() | 1),
            2 => {
                if !mutated.ctk.is_empty() {
                    mutated.ctk.pop();
                }
            }
            3 => {
                if !mutated.us.is_empty() {
                    mutated.us.pop();
                }
            }
            _ => {
                if !mutated.ys.is_empty() {
                    mutated.ys.pop();
                }
            }
        }
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            Type2Verifier::verify(&stmt_2, &mutated, &params_2, TranscriptMode::JavaCompat)
        }));
        assert_no_panic_and_rejected(outcome, &mut rejected_2);
    }
    assert!(rejected_2 > 0, "mutated type2 proofs should be rejected");

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
    let mut rejected_2p = 0usize;
    for _ in 0..128 {
        let mut mutated = proof_2p.clone();
        match rng.next_u32() % 5 {
            0 => mutated.eta3 += Fr::from(rng.next_u64() | 1),
            1 => mutated.eta4 += Fr::from(rng.next_u64() | 1),
            2 => {
                if !mutated.ctk_kprime.is_empty() {
                    mutated.ctk_kprime.pop();
                }
            }
            3 => {
                if !mutated.vs.is_empty() {
                    mutated.vs.pop();
                }
            }
            _ => {
                if !mutated.us.is_empty() {
                    mutated.us.pop();
                }
            }
        }
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            Type2PVerifier::verify(&stmt_2p, &mutated, &params_2p, TranscriptMode::JavaCompat)
        }));
        assert_no_panic_and_rejected(outcome, &mut rejected_2p);
    }
    assert!(rejected_2p > 0, "mutated type2p proofs should be rejected");

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
    let mut rejected_3 = 0usize;
    for _ in 0..128 {
        let mut mutated = proof_3.clone();
        match rng.next_u32() % 5 {
            0 => mutated.eta1 += Fr::from(rng.next_u64() | 1),
            1 => mutated.eta2 += Fr::from(rng.next_u64() | 1),
            2 => {
                if !mutated.c_w.is_empty() {
                    mutated.c_w.pop();
                }
            }
            3 => {
                if !mutated.djx.is_empty() {
                    mutated.djx.pop();
                }
            }
            _ => {
                if !mutated.witness_commitments.is_empty() {
                    mutated.witness_commitments.pop();
                }
            }
        }
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            Type3Verifier::verify(&stmt_3, &mutated, &params_3, TranscriptMode::JavaCompat)
        }));
        assert_no_panic_and_rejected(outcome, &mut rejected_3);
    }
    assert!(rejected_3 > 0, "mutated type3 proofs should be rejected");

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
    let mut rejected_4 = 0usize;
    for _ in 0..128 {
        let mut mutated = proof_4.clone();
        match rng.next_u32() % 5 {
            0 => mutated.rprime_2 += Fr::from(rng.next_u64() | 1),
            1 => mutated.rprime_4 += Fr::from(rng.next_u64() | 1),
            2 => {
                if !mutated.com_q.is_empty() {
                    mutated.com_q.pop();
                }
            }
            3 => {
                if !mutated.qv.is_empty() {
                    mutated.qv.pop();
                }
            }
            _ => {
                if !mutated.com_g.is_empty() {
                    mutated.com_g.pop();
                }
            }
        }
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            Type4BatchVerifier::verify(&stmt_4, &mutated, &params_4, TranscriptMode::JavaCompat)
        }));
        assert_no_panic_and_rejected(outcome, &mut rejected_4);
    }
    assert!(rejected_4 > 0, "mutated type4_batch proofs should be rejected");
}
