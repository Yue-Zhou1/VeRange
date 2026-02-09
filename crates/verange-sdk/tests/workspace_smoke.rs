use verange_sdk as _;

#[test]
fn workspace_smoke() {
    // Import-only smoke test to validate workspace wiring.
    let _ = std::any::TypeId::of::<verange_sdk::Prover>();
}
