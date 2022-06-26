//! Drive tests defined in `trycmd/*`.

#[test]
fn trycmd_tests() {
    trycmd::TestCases::new().case("trycmd/*.md");
}
