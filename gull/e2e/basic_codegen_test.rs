use crate::{enums_and_vecs_ast, nested_records_ast};
use gull::codegen::{Flow, Rust};
use gull::*;
use k9::*;

#[test]
fn rust() {
    assert_matches_inline_snapshot!(
        Rust::gen_decls(nested_records_ast()),
        "
#[derive(Debug)]
pub struct Test {
  pub age: i32,
  pub id: i32,
  pub name: String,
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
        Flow::gen_decls(nested_records_ast()),
        "
 export type Test = {
  age: number,
  id: number,
  name: string,
};

 export type WrapsTest = {
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
#[derive(Debug)]
pub struct Test {
  pub age: i32,
  pub id: i32,
  pub name: String,
}

#[derive(Debug)]
pub struct WrapsTest {
  pub test_inside: Test,
}
"
    );
}

#[test]
fn rust_enums_and_vecs() {
    assert_matches_inline_snapshot!(
        Rust::gen_decls(enums_and_vecs_ast()),
        "
#[derive(Debug)]
pub enum Event {
  Click(i32,i32,),
  KeyPress(String,),
}

#[derive(Debug)]
pub struct EventHistory {
  pub history: Vec<Event>,
}
"
    );
}
