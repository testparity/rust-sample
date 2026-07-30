use sample::{double, increment};

#[test]
fn covers_the_core_library_operations() {
    assert_eq!(double(4), 8);
    assert_eq!(increment(4), 5);
}
