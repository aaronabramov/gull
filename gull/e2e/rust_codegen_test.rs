use crate::project::Project;
use crate::{enums_and_vecs_ast, nested_records_ast};
use anyhow::Result;
use gull::{codegen::Rust, sign_source::sign_source, Codegen};
use k9::*;

#[test]
fn rust_codegen() -> Result<()> {
    let p = Project::new("rust_codegen_test")?;

    let mut generated = Rust::gen_decls(nested_records_ast());
    generated.push_str(&Rust::gen_decls(enums_and_vecs_ast()));
    let signed = sign_source(&generated);

    p.write_default_cargo_toml()?;
    p.write_file("types.rs", &signed)?;

    p.write_file(
        "lib.rs",
        r#"
mod types;
use types::{Test, WrapsTest, EventHistory, Event};

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

    let test_event_history = EventHistory {
        history: vec![Event::KeyPress("a".to_string()), Event::Click(1, 2)]
    };

    println!("{:?}", test_event_history);
    assert!(true);
}"#,
    )?;

    let output = p.run("cargo test --lib -- --nocapture")?;

    output.assert_success()?;

    assert_equal!(output.exit_code, Some(0));
    snapshot!(
        sanitize_cargo_test_output(&output.stdout),
        r#"

running 1 test
WrapsTest { test_inside: Test { age: 55, id: 44, name: "hi" } }
EventHistory { history: [KeyPress("a"), Click(1, 2)] }
test hello ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


"#
    );
    Ok(())
}

pub fn sanitize_cargo_test_output(s: &str) -> String {
    let regex = regex::Regex::new(r#"; finished in \d+\.\d\ds"#).unwrap();
    regex.replace_all(s, "").to_string()
}
