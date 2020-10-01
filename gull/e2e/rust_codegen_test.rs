use crate::project::Project;
use anyhow::Result;
use k9::*;

#[test]
fn rust_codegen() -> Result<()> {
    let p = Project::new("rust_codegen_test")?;

    p.write_default_cargo_toml()?;
    p.write_file(
        "lib.rs",
        r#"
#[test]
fn hello() {
    assert!(true);
}"#,
    )?;

    let output = p.run("cargo test --lib")?;

    assert_equal!(output.exit_code, Some(0));
    assert_equal!(output.success, true);
    assert_matches_inline_snapshot!(
        output.stdout,
        "
running 1 test
test hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

"
    );
    Ok(())
}
