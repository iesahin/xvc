use trycmd;
use which;
use xvc::error::Result;

#[test]
fn doc_tests() -> Result<()> {
    trycmd::TestCases::new()
        .register_bin("git", which::which("git")?)
        .case("docs/start/*.md")
        .case("docs/ref/*.md");

    Ok(())
}
