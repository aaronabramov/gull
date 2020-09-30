use gull::codegen::{Flow, Rust};
use gull::*;
use k9::*;

fn make_ast() -> Vec<StructDef> {
    let test = StructDef {
        name: "Test",
        fields: vec![
            ("name", TypeDef::TString),
            ("id", TypeDef::Ti32),
            ("age", TypeDef::Ti32),
        ],
    };

    let wraps_test = StructDef {
        name: "WrapsTest",
        fields: vec![("test_inside", TypeDef::TStructRef(test.clone()))],
    };

    vec![test, wraps_test]
}

#[test]
fn rust() {
    assert_matches_inline_snapshot!(
        Rust::gen_list(make_ast()),
        "
struct Test {
  pub name: String,
  pub id: i32,
  pub age: i32,
}

struct WrapsTest {
  pub test_inside: Test,
}
"
    );
}

#[test]
fn flow() {
    assert_matches_inline_snapshot!(
        Flow::gen_list(make_ast()),
        "
type Test = {
  name: string,
  id: number,
  age: number,
};

type WrapsTest = {
  test_inside: Test,
};
"
    );
}
