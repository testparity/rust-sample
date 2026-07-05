#[test]
fn applies_standard_discount() {
    assert_eq!(crate::apply_discount(100.0, false), 90.0);
}
