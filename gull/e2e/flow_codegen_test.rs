use crate::project::Project;
use anyhow::Result;
use k9::*;

#[test]
fn flow_codegen() -> Result<()> {
    let p = Project::new("flow_codegen_test")?;

    p.write_file(
        "package.json",
        r#"
{
    "devDependencies": {
        "flow-remove-types": "*",
        "flow-bin": "*"
    }
}
"#,
    )?;

    p.write_file(
        ".flowconfig",
        "
[ignore]

[include]

[libs]

[lints]

[options]

[strict]",
    )?;

    p.write_file(
        "index.js",
        r#"
// @flow

let a: number = 5;
console.log("hello");
"#,
    )?;

    p.run("npm install")?.assert_success()?;
    let flow_output = p.run(&p.absolute_path("./node_modules/.bin/flow")?)?;

    assert_matches_inline_snapshot!(
        &flow_output.stdout,
        "No errors!
"
    );
    flow_output.assert_success()?;

    let output = p.run(&p.absolute_path("./node_modules/.bin/flow-node index.js")?)?;

    output.assert_success()?;
    assert_equal!(output.exit_code, Some(0));
    assert_matches_inline_snapshot!(
        output.stdout,
        "hello
"
    );
    Ok(())
}
