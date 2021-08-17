use crate::prelude::*;
use anyhow::Result;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    let skip_serializing_none =
        StructFieldConfig::RustAttribute(r#"#[serde(skip_serializing_if = "Option::is_none")]"#);

    let struct_derives = TypeDeclarationConfig::RustAttribute("#[derive(Default, Clone)]");

    let struct_derives_eq_ord = TypeDeclarationConfig::RustAttribute(
        "#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq)]",
    );

    c.add_config(DeclarationsConfig::HackNamespace("NS"));

    c.add(TypeDeclaration {
        name: "",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::CodeBlock(CodeBlock::Rust(vec![
            "use chrono::{DateTime, Utc};",
            "use crate::types::{IndexedStr as _IndexedStr, ID as _ID};",
        ])),
    });

    c.add(TypeDeclaration {
        name: "ID",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::TDifferentPerLanguege {
            rust: Box::new(TPrimitive::THardcoded("_ID")),
            hack: Box::new(TPrimitive::Ti64),
            flow: Box::new(TPrimitive::Ti64),
        }),
    });

    c.add(TypeDeclaration {
        name: "",
        docs: "
        ==========================================================================
        Simple file defining various graph data types
        ==========================================================================",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::Docs,
    });

    c.add(TypeDeclaration {
        name: "NodeID",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::Ti64),
    });

    c.add(TypeDeclaration {
        name: "NodeName",
        docs: "",
        config: vec![],
        generic_params: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::String),
    });

    let t_generic = TGeneric::TDefinition {
        name: "T",
        bounds: None,
    };

    let ts_generic = TGeneric::TDefinition {
        name: "TS",
        bounds: Some("Ord"),
    };

    let tn_generic = TGeneric::TDefinition {
        name: "TN",
        bounds: Some("Ord"),
    };

    let dynamic_edge = c.add(TypeDeclaration {
        name: "DynamicEdge",
        docs: "",
        config: vec![struct_derives_eq_ord],
        generic_params: vec![ts_generic.clone(), tn_generic.clone()],
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![
                StructField {
                    name: "branches",
                    docs: "",
                    field_type: StructFieldType::TMap(TMap {
                        key: TPrimitive::TGeneric(ts_generic.clone()),
                        value: TMapValue::TSet(TSet::TPrimitive(TPrimitive::TGeneric(
                            tn_generic.clone(),
                        ))),
                        t: TMapType::BTree,
                    }),
                    config: vec![],
                },
                StructField {
                    name: "properties",
                    docs: "",
                    field_type: StructFieldType::TOption(TOption::TMap(TMap {
                        key: TPrimitive::TGeneric(ts_generic.clone()),
                        value: TMapValue::TSet(TSet::TPrimitive(TPrimitive::TGeneric(
                            ts_generic.clone(),
                        ))),
                        t: TMapType::BTree,
                    })),
                    config: vec![],
                },
            ],
        }),
    });

    let node_edges = c.add(TypeDeclaration {
        name: "NodeEdges",
        docs: "",
        config: vec![struct_derives],
        generic_params: vec![ts_generic.clone(), tn_generic.clone()],
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![
                StructField {
                    name: "directed",
                    docs: "",
                    field_type: StructFieldType::TSet(TSet::TPrimitive(TPrimitive::TGeneric(
                        tn_generic.clone(),
                    ))),
                    config: vec![],
                },
                StructField {
                    name: "dynamic",
                    docs: "",
                    field_type: StructFieldType::TPrimitive(TPrimitive::TReference(dynamic_edge)),
                    config: vec![],
                },
                StructField {
                    name: "tagged",
                    docs: "",
                    field_type: StructFieldType::TOption(TOption::TMap(TMap {
                        key: TPrimitive::TGeneric(ts_generic),
                        value: TMapValue::TSet(TSet::TPrimitive(TPrimitive::TGeneric(tn_generic))),
                        t: TMapType::BTree,
                    })),
                    config: vec![],
                },
            ],
        }),
    });

    let node = c.add(TypeDeclaration {
        name: "GraphNode",
        docs: "",
        config: vec![struct_derives],
        generic_params: vec![t_generic.clone()],
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![
                StructField {
                    name: "name",
                    docs: "",
                    field_type: StructFieldType::TPrimitive(TPrimitive::TGeneric(
                        t_generic.clone(),
                    )),
                    config: vec![],
                },
                StructField {
                    name: "edges",
                    docs: "",
                    field_type: StructFieldType::TOption(TOption::TPrimitive(
                        TPrimitive::TReference(node_edges),
                    )),
                    config: vec![skip_serializing_none],
                },
            ],
        }),
    });

    let graph = c.add(TypeDeclaration {
        name: "Graph",
        docs: "",
        config: vec![struct_derives],
        generic_params: vec![t_generic.clone()],
        value: DeclarationValue::TStruct(TStruct {
            fields: vec![
                StructField {
                    name: "nodes",
                    docs: "",
                    field_type: StructFieldType::TMap(TMap {
                        key: TPrimitive::TGeneric(t_generic.clone()),
                        value: TMapValue::TPrimitive(TPrimitive::TReference(node)),
                        t: TMapType::Hash,
                    }),
                    config: vec![],
                },
                StructField {
                    name: "timestamp",
                    docs: "",
                    field_type: StructFieldType::TPrimitive(TPrimitive::String),
                    config: vec![StructFieldConfig::RustOverride("DateTime<Utc>")],
                },
            ],
        }),
    });

    let mut generic_graph = graph;
    generic_graph.generic_params = vec![t_generic.clone()];

    c.add(TypeDeclaration {
        name: "GraphProxyType",
        docs: "",
        config: vec![],
        generic_params: vec![t_generic],
        value: DeclarationValue::TPrimitive(generic_graph.primitive()),
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
use std::collections::BTreeSet;
use std::collections::HashMap;


use chrono::{DateTime, Utc};
use crate::types::{IndexedStr as _IndexedStr, ID as _ID};

pub type ID = _ID;

// ==========================================================================
// Simple file defining various graph data types
// ==========================================================================


pub type NodeID = i64;

pub type NodeName = String;

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct DynamicEdge<TS: Ord, TN: Ord> {
    pub branches: BTreeMap<TS, BTreeSet<TN>>,
    pub properties: Option<BTreeMap<TS, BTreeSet<TS>>>,
}

#[derive(Default, Clone)]
pub struct NodeEdges<TS: Ord, TN: Ord> {
    pub directed: BTreeSet<TN>,
    pub dynamic: DynamicEdge,
    pub tagged: Option<BTreeMap<TS, BTreeSet<TN>>>,
}

#[derive(Default, Clone)]
pub struct GraphNode<T> {
    pub name: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edges: Option<NodeEdges>,
}

#[derive(Default, Clone)]
pub struct Graph<T> {
    pub nodes: HashMap<T, GraphNode>,
    pub timestamp: DateTime<Utc>,
}

pub type GraphProxyType<T> = Graph<T>;

"#
    );

    Ok(())
}

#[test]
fn hack_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(
        declarations.codegen_hack()?,
        r"
<?hh // strict



type NSID = int;

// ==========================================================================
// Simple file defining various graph data types
// ==========================================================================


type NSNodeID = int;

type NSNodeName = string;

type NSDynamicEdge<TS, TN> = shape(
    'branches' => dict<TS, keyset<TN>>,
    ?'properties' => dict<TS, keyset<TS>>,
);

type NSNodeEdges<TS, TN> = shape(
    'directed' => keyset<TN>,
    'dynamic' => NSDynamicEdge,
    ?'tagged' => dict<TS, keyset<TN>>,
);

type NSGraphNode<T> = shape(
    'name' => T,
    ?'edges' => NSNodeEdges,
);

type NSGraph<T> = shape(
    'nodes' => dict<T, NSGraphNode>,
    'timestamp' => string,
);

type NSGraphProxyType<T> = NSGraph<T>;

"
    );

    Ok(())
}

#[test]
fn flow_test() -> Result<()> {
    let declarations = make_declarations();
    k9::snapshot!(
        declarations.codegen_flow()?,
        r"
/*
 * @flow
 * @nolint
 */




export type ID = number;

// ==========================================================================
// Simple file defining various graph data types
// ==========================================================================


export type NodeID = number;

export type NodeName = string;

export type DynamicEdge<TS, TN> = {|
    'branches': {[key: TS]: Array<TN>},
    'properties': ?{[key: TS]: Array<TS>},
|};

export type NodeEdges<TS, TN> = {|
    'directed': Array<TN>,
    'dynamic': DynamicEdge,
    'tagged': ?{[key: TS]: Array<TN>},
|};

export type GraphNode<T> = {|
    'name': T,
    'edges': ?NodeEdges,
|};

export type Graph<T> = {|
    'nodes': {[key: T]: GraphNode},
    'timestamp': string,
|};

export type GraphProxyType<T> = Graph<T>;

"
    );
    Ok(())
}
