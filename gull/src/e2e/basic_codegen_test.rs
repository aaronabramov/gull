use crate::prelude::*;
use anyhow::Result;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    c.add_config(DeclarationsConfig::HackNamespace("GraphiteIngester"));

    c.add(TypeDeclaration {
        name: "",
        docs: "Hello world
        ==========================================================================
        THIS IS AN INDEPENDENT DOCUMENTATION BLOCK
        ==========================================================================",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::Docs,
    });

    let frame = c.add(TypeDeclaration {
        name: "Frame",
        docs: "Frame represents a tuple of an Timestamp (RFC3339) and an ID",
        config: vec![TypeDeclarationConfig::RustAttribute("#[derive(Copy)]")],
        generic_params: vec![],
        value: DeclarationValue::TTuple(TTuple {
            items: vec![
                TupleItem::TPrimitive(TPrimitive::String),
                TupleItem::TPrimitive(TPrimitive::Ti64),
            ],
        }),
    });

    let indexable_str = c.add(TypeDeclaration {
        name: "IndexableStr",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::TDifferentPerLanguege {
            rust: Box::new(TPrimitive::THardcoded("crate::types::IndexableStr")),
            hack: Box::new(TPrimitive::String),
            flow: Box::new(TPrimitive::String),
        }),
    });

    c.add(TypeDeclaration {
        name: "StorageType",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TSimpleEnum(TSimpleEnum {
            variants: vec!["Full", "Delta", "Empty", "Broken"],
        }),
    });

    c.add(TypeDeclaration {
        name: "Operation",
        docs: "Operation is a single unit of transormation logic",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TEnum(TEnum {
            variants: vec![
                EnumVariant {
                    name: "Fetch",
                    docs: "Fetch items by their IDs",
                    variant_type: EnumVariantType::TStruct(TStruct {
                        fields: vec![StructField {
                            name: "items",
                            docs: "item IDs",
                            config: vec![],
                            field_type: StructFieldType::TVec(TVec::TPrimitive(TPrimitive::Ti64)),
                        }],
                    }),
                },
                EnumVariant {
                    name: "Store",
                    docs: "Store graphs to a storage layer",
                    variant_type: EnumVariantType::TStruct(TStruct {
                        fields: vec![StructField {
                            name: "frames",
                            docs: "Destination frames for the storage",
                            config: vec![],
                            field_type: StructFieldType::TVec(TVec::TPrimitive(
                                TPrimitive::TReference(frame),
                            )),
                        }],
                    }),
                },
                EnumVariant {
                    name: "Drop",
                    docs: "Discard all graphs",
                    variant_type: EnumVariantType::TPrimitive(TPrimitive::Tbool),
                },
                EnumVariant {
                    name: "FakeOp",
                    docs: "Not a real operation",
                    variant_type: EnumVariantType::TPrimitive(TPrimitive::Ti64),
                },
            ],
        }),
    });

    let node_id = c.add(TypeDeclaration {
        name: "NodeID",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::Ti64),
    });

    let graph_node = c.add(TypeDeclaration {
        name: "GraphNode",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![StructField {
                name: "node_id",
                docs: "",
                config: vec![],
                field_type: StructFieldType::TPrimitive(TPrimitive::TReference(node_id)),
            }],
        }),
    });

    let graph_data = c.add(TypeDeclaration {
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
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![
                StructField {
                    name: "entry_points",
                    docs: "Root nodes of the graph",
                    config: vec![],
                    field_type: StructFieldType::TVec(TVec::TPrimitive(TPrimitive::Ti64)),
                },
                StructField {
                    name: "nodes",
                    docs: "",
                    config: vec![],
                    field_type: StructFieldType::TMap(TMap {
                        key: TPrimitive::Ti64,
                        value: TMapValue::TPrimitive(TPrimitive::TReference(graph_node)),
                        t: TMapType::BTree,
                    }),
                },
                StructField {
                    name: "string_fields",
                    docs: "A bunch of random string fields
                    that are represented as a map between string and string
                    and other important lines of documentation.",
                    config: vec![StructFieldConfig::RustAttribute(
                        r#"#[serde(skip_serializing_if = "Option::is_none")]"#,
                    )],
                    field_type: StructFieldType::TOption(TOption::TMap(TMap {
                        key: TPrimitive::String,
                        value: TMapValue::TPrimitive(TPrimitive::String),
                        t: TMapType::BTree,
                    })),
                },
            ],
        }),
    });

    let mut graph_data_unindexed = graph_data;
    graph_data_unindexed.generic_params = vec![
        TGeneric::TReference(indexable_str.clone()),
        TGeneric::TReference(indexable_str),
    ];

    c.add(TypeDeclaration {
        name: "GraphDataUnindexed",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::TReference(graph_data_unindexed)),
    });

    c
}

#[test]
fn rust_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(
        declarations.codegen_rust()?,
        r#"
use std::collections::BTreeMap;


// Hello world
// ==========================================================================
// THIS IS AN INDEPENDENT DOCUMENTATION BLOCK
// ==========================================================================


#[derive(Copy)]
/// Frame represents a tuple of an Timestamp (RFC3339) and an ID
pub type Frame = (String, i64);

pub type IndexableStr = crate::types::IndexableStr;

pub enum StorageType {
    Full,
    Delta,
    Empty,
    Broken,
}

/// Operation is a single unit of transormation logic
pub enum Operation {
    /// Fetch items by their IDs
    Fetch {
        /// item IDs
        items: Vec<i64>,
    },
    /// Store graphs to a storage layer
    Store {
        /// Destination frames for the storage
        frames: Vec<Frame>,
    },
    /// Discard all graphs
    Drop(bool),
    /// Not a real operation
    FakeOp(i64),
}

pub type NodeID = i64;

pub struct GraphNode {
    pub node_id: NodeID,
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
pub struct GraphData {
    /// Root nodes of the graph
    pub entry_points: Vec<i64>,
    pub nodes: BTreeMap<i64, GraphNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A bunch of random string fields
    /// that are represented as a map between string and string
    /// and other important lines of documentation.
    pub string_fields: Option<BTreeMap<String, String>>,
}

pub type GraphDataUnindexed = GraphData<IndexableStr, IndexableStr>;

"#
    );

    Ok(())
}

#[test]
fn hack_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(
        declarations.codegen_hack()?,
        r#"
<?hh

// Hello world
// ==========================================================================
// THIS IS AN INDEPENDENT DOCUMENTATION BLOCK
// ==========================================================================


// Frame represents a tuple of an Timestamp (RFC3339) and an ID
type GraphiteIngesterFrame = (string, int);

type GraphiteIngesterIndexableStr = string;

enum GraphiteIngesterStorageType: string as string {
    FULL = "Full";
    DELTA = "Delta";
    EMPTY = "Empty";
    BROKEN = "Broken";
}

// Operation is a single unit of transormation logic

enum GraphiteIngesterOperationType: string as string {
    FETCH = "Fetch";
    STORE = "Store";
    DROP = "Drop";
    FAKEOP = "FakeOp";
}

type GraphiteIngesterOperation = shape(
    // Fetch items by their IDs
    ?'Fetch' =>  shape(
        // item IDs
        'items' => vec<int>,
    ),
    // Store graphs to a storage layer
    ?'Store' =>  shape(
        // Destination frames for the storage
        'frames' => vec<GraphiteIngesterFrame>,
    ),
    // Discard all graphs
    ?'Drop' => bool,
    // Not a real operation
    ?'FakeOp' => int,
);

type GraphiteIngesterNodeID = int;

type GraphiteIngesterGraphNode = shape(
    'node_id' => GraphiteIngesterNodeID,
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
type GraphiteIngesterGraphData = shape(
    // Root nodes of the graph
    'entry_points' => vec<int>,
    'nodes' => dict<int, GraphiteIngesterGraphNode>,
    // A bunch of random string fields
    // that are represented as a map between string and string
    // and other important lines of documentation.
    ?'string_fields' => dict<string, string>,
);

type GraphiteIngesterGraphDataUnindexed = GraphiteIngesterGraphData<GraphiteIngesterIndexableStr, GraphiteIngesterIndexableStr>;

"#
    );

    Ok(())
}

#[test]
fn flow_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(
        declarations.codegen_flow()?,
        r#"

// Hello world
// ==========================================================================
// THIS IS AN INDEPENDENT DOCUMENTATION BLOCK
// ==========================================================================


// Frame represents a tuple of an Timestamp (RFC3339) and an ID
export type Frame = [string, number];

export type IndexableStr = string;

export type StorageType = "Full" | "Delta" | "Empty" | "Broken";

// Operation is a single unit of transormation logic
type OperationType = "Fetch" | "Store" | "Drop" | "FakeOp";

type Operation = {|
    'type': OperationType,
    // Fetch items by their IDs
    'Fetch'?:  {|
        // item IDs
        'items': Array<number>,
    |},
    // Store graphs to a storage layer
    'Store'?:  {|
        // Destination frames for the storage
        'frames': Array<Frame>,
    |},
    // Discard all graphs
    'Drop'?: boolean,
    // Not a real operation
    'FakeOp'?: number,
|};

export type NodeID = number;

export type GraphNode = {|
    'node_id': NodeID,
|};

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
export type GraphData = {|
    // Root nodes of the graph
    'entry_points': Array<number>,
    'nodes': {[key: number]: GraphNode},
    // A bunch of random string fields
    // that are represented as a map between string and string
    // and other important lines of documentation.
    'string_fields': ?{[key: string]: string},
|};

export type GraphDataUnindexed = GraphData<IndexableStr, IndexableStr>;

"#
    );
    Ok(())
}
