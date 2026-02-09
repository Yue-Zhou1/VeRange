use ark_bn254::Fr;
use num_bigint::BigUint;
use verange_core::arith::decompose_to_nary;
use verange_core::vector::ScalarVector;

#[test]
fn vector_tests_elementwise_add_sub_mul() {
    let a = ScalarVector::new(vec![Fr::from(2u64), Fr::from(3u64), Fr::from(5u64)]).expect("a");
    let b = ScalarVector::new(vec![Fr::from(7u64), Fr::from(11u64), Fr::from(13u64)]).expect("b");

    let sum = a.add(&b).expect("sum");
    let diff = b.sub(&a).expect("diff");
    let prod = a.mul_elementwise(&b).expect("prod");

    assert_eq!(sum.values(), &[Fr::from(9u64), Fr::from(14u64), Fr::from(18u64)]);
    assert_eq!(diff.values(), &[Fr::from(5u64), Fr::from(8u64), Fr::from(8u64)]);
    assert_eq!(prod.values(), &[Fr::from(14u64), Fr::from(33u64), Fr::from(65u64)]);
}

#[test]
fn vector_tests_inner_product() {
    let a = ScalarVector::new(vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)]).expect("a");
    let b = ScalarVector::new(vec![Fr::from(4u64), Fr::from(5u64), Fr::from(6u64)]).expect("b");

    let ip = a.inner_product(&b).expect("inner product");

    assert_eq!(ip, Fr::from(32u64));
}

#[test]
fn vector_tests_power_vector_generation() {
    let y = Fr::from(3u64);
    let powers = ScalarVector::power(y, 5);

    assert_eq!(
        powers.values(),
        &[
            Fr::from(1u64),
            Fr::from(3u64),
            Fr::from(9u64),
            Fr::from(27u64),
            Fr::from(81u64)
        ]
    );
}

#[test]
fn vector_tests_nary_decomposition() {
    let value = BigUint::from(13u32);
    let base = BigUint::from(4u32);

    let digits = decompose_to_nary(&value, &base).expect("decompose");

    assert_eq!(digits, vec![BigUint::from(1u32), BigUint::from(3u32)]);
}
