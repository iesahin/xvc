use trycmd;

#[test]
fn doc_tests() {
    trycmd::TestCases::new()
        .case("docs/start/ml.md")
        .case("docs/ref/xvc.md");
}
