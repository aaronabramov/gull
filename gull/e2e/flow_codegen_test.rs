use crate::project::Project;
use crate::{enums_and_vecs_ast, nested_records_ast};
use anyhow::Result;
use gull::{codegen::Flow, sign_source::sign_source, Codegen};
use k9::*;

#[test]
#[ignore]
fn flow_codegen() -> Result<()> {
    let p = Project::new("flow_codegen_test")?;

    let mut generated = format!("//@flow \n {}", Flow::gen_decls(nested_records_ast()));

    generated.push_str(&Flow::gen_decls(enums_and_vecs_ast()));
    let signed = sign_source(&generated);

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

    p.write_file("types.js", &signed)?;

    p.write_file(
        "index.js",
        r#"
// @flow
import type {WrapsTest, Test, EventHistory, Event} from './types';

let a: WrapsTest = {
    test_inside: {
            name: "hi",
            id: 44,
            age: 55,
    }
};

let test_event_history: EventHistory = {
    history: [{ click: [10, 20] }, { keyPress: ["a"]}]
};

console.log(a);
"#,
    )?;

    p.run("npm install")?.assert_success()?;
    let flow_output = p.run(&p.absolute_path("./node_modules/.bin/flow")?)?;

    snapshot!(
        &flow_output.stdout,
        "No errors!
"
    );
    flow_output.assert_success()?;

    let output = p.run(&p.absolute_path("./node_modules/.bin/flow-node index.js")?)?;

    output.assert_success()?;
    assert_equal!(output.exit_code, Some(0));
    snapshot!(
        output.stdout,
        "{ test_inside: { name: 'hi', id: 44, age: 55 } }
"
    );
    Ok(())
}
