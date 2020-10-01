use crate::project::Project;
use anyhow::Result;
use k9::*;

// This test is disabled on CI because i don't think it's easily possible
// to install HHVM and Hack support on github actions servers.
//
// To test locally (on mac) run:
//      brew tap hhvm/hhvm
//      brew install hhvm
//
// This will install `hhvm` and `hh_client` binaries
#[test]
#[ignore]
fn hack_codegen() -> Result<()> {
    let p = Project::new("hack_codegen_test")?;

    p.write_file(".hhconfig", "")?;

    p.write_file(
        "main.hack",
        r#"

type Test = shape('hello' => vec<int>);

<<__EntryPoint>>
function main(): void {
  var_dump(make_test());
}

function make_test(): Test {
  return shape("hello" => vec[1, 2, 3]);
}

"#,
    )?;

    let hh_output = p.run("hh_client")?;

    assert_matches_inline_snapshot!(
        &hh_output.stdout,
        "No errors!
"
    );
    hh_output.assert_success()?;

    let output = p.run("hhvm main.hack")?;

    output.assert_success()?;
    assert_equal!(output.exit_code, Some(0));
    assert_matches_inline_snapshot!(
        output.stdout,
        "array(1) {
  [\"hello\"]=>
  vec(3) {
    int(1)
    int(2)
    int(3)
  }
}
"
    );
    Ok(())
}
