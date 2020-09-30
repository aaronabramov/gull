use crate::project::Project;
use anyhow::{Context, Result};
use k9::*;
use std::process::Command;

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

    let mut cmd = Command::new("cargo");
    cmd.current_dir(p.root_dir()).arg("test").arg("--lib");

    let output = cmd.output().context("failed to get command's output")?;

    let exit_code = output.status.code();
    let success = output.status.success();

    let stdout = String::from_utf8(output.stdout)?;

    assert_equal!(exit_code, Some(0));
    assert_equal!(success, true);
    assert_matches_inline_snapshot!(
        stdout,
        "
running 1 test
test hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

"
    );
    Ok(())
}
