// use gull::codegen::{ Rust};
use crate::project::Project;
use anyhow::Result;
use gull::definitions::*;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    let node_id = c.add(TypeDeclaration::PrimitiveType {
        name: "NodeID",
        value: PrimitiveType::String,
    });

    c.add(TypeDeclaration::Struct {
        name: "GraphNode",
        value: Struct {
            fields: vec![StructField {
                name: "node_id",
                field_type: StructFieldType::Reference(node_id),
            }],
        },
    });

    c
}

#[test]
fn basic_test() -> Result<()> {
    let p = Project::new("basic_codegen_test")?;

    let declarations = make_declarations();
    k9::snapshot!(declarations.codegen_rust()?, "");

    k9::snapshot!(
        declarations,
        r#"
Declarations {
    declarations: [
        PrimitiveType {
            name: "NodeID",
            value: String,
        },
        Struct {
            name: "GraphNode",
            value: Struct {
                fields: [
                    StructField {
                        name: "node_id",
                        field_type: Reference(
                            PrimitiveType {
                                name: "NodeID",
                                value: String,
                            },
                        ),
                    },
                ],
            },
        },
    ],
}
"#
    );

    Ok(())
}
