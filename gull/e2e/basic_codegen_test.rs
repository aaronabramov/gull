use anyhow::Result;
use gull::definitions::*;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    let frame = c.add(TypeDeclaration {
        name: "Frame",
        docs: "Frame represents a tuple of an Timestamp (RFC3339) and an ID",
        value: DeclarationValue::TTuple(TTuple {
            items: vec![
                TupleItem::TPrimitive(TPrimitive::String),
                TupleItem::TPrimitive(TPrimitive::Ti64),
            ],
        }),
    });

    c.add(TypeDeclaration {
        name: "Operation",
        docs: "Operation is a single unit of transormation logic",
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
        docs: "",
        value: DeclarationValue::TPrimitive(TPrimitive::Ti64),
    });

    let graph_node = c.add(TypeDeclaration {
        name: "GraphNode",
        docs: "",
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![StructField {
                name: "node_id",
                field_type: StructFieldType::Reference(node_id),
            }],
        }),
    });

    c.add(TypeDeclaration {
        name: "GraphData",
        docs: r#"Wrapper value that represents a graph. It contains various top level
        data about the graph as well as a collection of nodes. This is a long
        multiline documentaino block that is here for testing purposes only. I'll also
        add some Ascii diagram just to make sure nothing gets misalligned.

            $> SELECT name, age, hometown, credit_card_number FROM users

            +----------+-----+-------------+----------------+
            | Name     | Age | Location    | Credit Card    |
            +----------+-----+-------------+----------------+
            | Bobby    | 17  | El Paso, TX | 1234 4294 2492 |
            | Hannah   | 20  | Memphis, TN | 9494 2492 4024 |
            +----------+-----+-------------+----------------+
        
        Some more docs after the ASCII drawings.

        Maybe some extra line after a newline.
        "#,
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
        r"
use std::collections::BTreeMap;


/// Frame represents a tuple of an Timestamp (RFC3339) and an ID
type Frame = (String, i64);

/// Operation is a single unit of transormation logic
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

/// Wrapper value that represents a graph. It contains various top level
/// data about the graph as well as a collection of nodes. This is a long
/// multiline documentaino block that is here for testing purposes only. I'll also
/// add some Ascii diagram just to make sure nothing gets misalligned.
/// 
///     $> SELECT name, age, hometown, credit_card_number FROM users
/// 
///     +----------+-----+-------------+----------------+
///     | Name     | Age | Location    | Credit Card    |
///     +----------+-----+-------------+----------------+
///     | Bobby    | 17  | El Paso, TX | 1234 4294 2492 |
///     | Hannah   | 20  | Memphis, TN | 9494 2492 4024 |
///     +----------+-----+-------------+----------------+
/// 
/// Some more docs after the ASCII drawings.
/// 
/// Maybe some extra line after a newline.
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
    k9::snapshot!(
        declarations.codegen_hack()?,
        r#"


// Frame represents a tuple of an Timestamp (RFC3339) and an ID
type Frame = tuple(string, int);

// Operation is a single unit of transormation logic

enum OperationType: string as string {
    FETCH = "Fetch",
    STORE = "Store",
    DROP = "Drop",
}

type Operation = shape(
    'type' => OperationType,
    ?'Fetch' => ?tuple(int),
    ?'Store' => ? shape(
        'frames' => vec<Frame>,
    ),
);

type NodeID = int;

type GraphNode = shape(
    'node_id' => NodeID,
);

// Wrapper value that represents a graph. It contains various top level
// data about the graph as well as a collection of nodes. This is a long
// multiline documentaino block that is here for testing purposes only. I'll also
// add some Ascii diagram just to make sure nothing gets misalligned.
// 
//     $> SELECT name, age, hometown, credit_card_number FROM users
// 
//     +----------+-----+-------------+----------------+
//     | Name     | Age | Location    | Credit Card    |
//     +----------+-----+-------------+----------------+
//     | Bobby    | 17  | El Paso, TX | 1234 4294 2492 |
//     | Hannah   | 20  | Memphis, TN | 9494 2492 4024 |
//     +----------+-----+-------------+----------------+
// 
// Some more docs after the ASCII drawings.
// 
// Maybe some extra line after a newline.
type GraphData = shape(
    'entry_points' => vec<int>,
    'nodes' => dict<int, GraphNode>,
    'string_fields' => ?dict<string, string>,
);

"#
    );

    Ok(())
}
