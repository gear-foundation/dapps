use trybuild::TestCases;

#[test]
fn derive() {
    let t = TestCases::new();
    t.pass("tests/derive/0-single-field.rs");
    t.pass("tests/derive/1-multiple-fields.rs");
    t.compile_fail("tests/derive/2-no-fields.rs");
    t.compile_fail("tests/derive/3-not-struct.rs");
    t.compile_fail("tests/derive/4-union-struct.rs");
}
