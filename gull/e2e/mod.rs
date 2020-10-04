mod basic_codegen_test;
mod flow_codegen_test;
mod hack_codegen_test;
mod project;
mod rust_codegen_test;

use gull::{GullType, GullTypeDecl};

pub fn nested_records_ast() -> Vec<GullTypeDecl> {
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

pub fn enums_and_vecs_ast() -> Vec<GullTypeDecl> {
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
