// use gull::codegen::{ Rust};
use crate::project::Project;
use anyhow::Result;
use gull::definitions::*;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    let frame = c.add(TypeDeclaration {
        name: "Frame",
        value: DeclarationValue::TTuple(TTuple {
            items: vec![
                TupleItem::TPrimitive(TPrimitive::String),
                TupleItem::TPrimitive(TPrimitive::Ti64),
            ],
        }),
    });

    c.add(TypeDeclaration {
        name: "Operation",
        value: DeclarationValue::TEnum(TEnum {
            variants: vec![
                EnumVariant {
                    name: "Fetch",
                    variant_type: EnumVariantType::Tuple(TTuple {
                        items: vec![TupleItem::TPrimitive(TPrimitive::Ti64)],
                    }),
                },
                EnumVariant {
                    name: "Store",
                    variant_type: EnumVariantType::Struct(TStruct {
                        fields: vec![StructField {
                            name: "frames",
                            field_type: StructFieldType::TVec(TVec::Reference(frame)),
                        }],
                    }),
                },
                EnumVariant {
                    name: "Drop",
                    variant_type: EnumVariantType::Empty,
                },
            ],
        }),
    });

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
                StructField {
                    name: "string_fields",
                    field_type: StructFieldType::TOption(TOption::TMap(TMap {
                        key: TPrimitive::String,
                        value: TMapValue::TPrimitive(TPrimitive::String),
                    })),
                },
            ],
        }),
    });

    c
}

#[test]
fn rust_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(
        declarations.codegen_rust()?,
        "
use std::collections::BTreeMap;


type Frame = (String, i64);

enum Operation {
  Fetch(i64),
  Store {
    frames: Vec<Frame>,
},
  Drop,
}

type NodeID = i64;

struct GraphNode {
    node_id: NodeID,
}

struct GraphData {
    entry_points: Vec<i64>,
    nodes: BTreeMap<i64, GraphNode>,
    string_fields: Option<BTreeMap<String, String>>,
}

"
    );

    Ok(())
}

#[test]
fn hack_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(declarations.codegen_hack()?, "");

    Ok(())
}
