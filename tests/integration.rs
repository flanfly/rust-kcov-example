extern crate kcov_example as kcov;

#[test]
fn integration_test() {
    assert_eq!(-42,kcov::tested1(42));
}
