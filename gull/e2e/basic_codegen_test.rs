// use gull::codegen::{ Rust};
use crate::project::Project;
use anyhow::Result;
use gull::definitions::*;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    let node_id = c.add(TypeDeclaration {
        name: "NodeID",
        value: DeclarationValue::TPrimitive(TPrimitive::Ti64),
    });

    let graph_node = c.add(TypeDeclaration {
        name: "GraphNode",
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![StructField {
                name: "node_id",
                field_type: StructFieldType::Reference(node_id),
            }],
        }),
    });

    c.add(TypeDeclaration {
        name: "GraphData",
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![
                StructField {
                    name: "entry_points",
                    field_type: StructFieldType::TVec(TVec::TPrimitive(TPrimitive::Ti64)),
                },
                StructField {
                    name: "nodes",
                    field_type: StructFieldType::TMap(TMap {
                        key: TPrimitive::Ti64,
                        value: TMapValue::Reference(graph_node),
                    }),
                },
            ],
        }),
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
