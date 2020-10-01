mod flow_codegen_test;
mod hack_codegen_test;
mod project;
mod rust_codegen_test;

use gull::{StructDef, TypeDef};

// Basic AST fixture for tests
pub fn make_ast() -> Vec<StructDef> {
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
