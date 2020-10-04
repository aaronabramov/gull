use crate::nested_records_ast;
use crate::project::Project;
use anyhow::Result;
use gull::{codegen::Rust, sign_source::sign_source, Codegen};
use k9::*;

#[test]
fn rust_codegen() -> Result<()> {
    let p = Project::new("rust_codegen_test")?;

    let generated = Rust::gen_decls(nested_records_ast());
    let signed = sign_source(&generated);

    p.write_default_cargo_toml()?;
    p.write_file("types.rs", &signed)?;

    p.write_file(
        "lib.rs",
        r#"
mod types;
use types::{Test, WrapsTest};

#[test]
fn hello() {
    let test_struct = WrapsTest {
        test_inside: Test {
            name: String::from("hi"),
            id: 44,
            age: 55,
        }
    };

    println!("{:?}", test_struct);
    assert!(true);
}"#,
    )?;

    let output = p.run("cargo test --lib -- --nocapture")?;

    output.assert_success()?;

    assert_equal!(output.exit_code, Some(0));
    assert_matches_inline_snapshot!(
        output.stdout,
        "
running 1 test
WrapsTest { test_inside: Test { age: 55, id: 44, name: \"hi\" } }
test hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

"
    );
    Ok(())
}
