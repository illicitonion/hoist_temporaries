#[test]
fn examples() {
    let t = trybuild::TestCases::new();
    t.pass("tests/working_examples/*.rs");
    t.compile_fail("tests/failing_examples/*.rs");
}
