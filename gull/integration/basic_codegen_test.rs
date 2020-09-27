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
#[derive(Debug)]
pub struct Test {
  pub name: String,
  pub id: i32,
  pub age: i32,
}

#[derive(Debug)]
pub struct WrapsTest {
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

#[test]
fn rust_nested_records() {
    assert_matches_inline_snapshot!(
        Rust::gen_decls(nested_records_ast()),
        "
struct Test {
  pub age: i32,
  pub id: i32,
  pub name: String,
}

struct WrapsTest {
  pub test_inside: Test,
}
"
    );
}

fn nested_records_ast() -> Vec<GullTypeDecl> {
    use GullType::*;

    let test = GullTypeDecl {
        name: "Test".to_string(),
        gull_type: TRecord(
            vec![
                ("name".to_string(), TString),
                ("id".to_string(), Ti32),
                ("age".to_string(), Ti32),
            ]
            .into_iter()
            .collect(),
        ),
    };

    let wraps_test = GullTypeDecl {
        name: "WrapsTest".to_string(),
        gull_type: TRecord(
            vec![("test_inside".to_string(), TSymbol("Test"))]
                .into_iter()
                .collect(),
        ),
    };
    vec![test, wraps_test]
}

#[test]
fn rust_enums_and_vecs() {
    assert_matches_inline_snapshot!(
        Rust::gen_decls(enums_and_vecs_ast()),
        "
enum Event {
  Click(i32,i32,)
  KeyPress(String,)
}

struct EventHistory {
  pub history: Vec<Event>,
}
"
    );
}

fn enums_and_vecs_ast() -> Vec<GullTypeDecl> {
    use GullType::*;

    let event_decl = GullTypeDecl {
        name: "Event".to_string(),
        gull_type: TEnum(
            vec![
                ("KeyPress".to_string(), vec![TString]),
                ("Click".to_string(), vec![Ti32, Ti32]),
            ]
            .into_iter()
            .collect(),
        ),
    };

    let vec_decl = GullTypeDecl {
        name: "EventHistory".to_string(),
        gull_type: TRecord(
            vec![(
                "history".to_string(),
                GullType::TVec(Box::new(TSymbol("Event"))),
            )]
            .into_iter()
            .collect(),
        ),
    };

    vec![event_decl, vec_decl]
}
