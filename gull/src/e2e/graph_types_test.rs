use crate::prelude::*;
use anyhow::Result;

fn make_declarations() -> Declarations {
    let mut c = Declarations::new();

    c.add_config(DeclarationsConfig::HackNamespace("GraphiteIngester"));

    c.add(TypeDeclaration {
        name: "",
        docs: "
        ==========================================================================
        Simple file defining various graph data types
        ==========================================================================",
        config: vec![],
        value: DeclarationValue::Docs,
    });

    c.add(TypeDeclaration {
        name: "NodeID",
        docs: "",
        config: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::Ti64),
    });

    c.add(TypeDeclaration {
        name: "NodeName",
        docs: "",
        config: vec![],
        value: DeclarationValue::TPrimitive(TPrimitive::String),
    });

    c.add(TypeDeclaration {
        name: "GraphNode",
        docs: "",
        config: vec![],
        value: DeclarationValue::TStruct(TStruct {
            generic_params: vec![TGeneric("T")],
            fields: vec![StructField {
                name: "name",
                docs: "",
                field_type: StructFieldType::TGeneric(TGeneric("T")),
                config: vec![],
            }],
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


// ==========================================================================
// Simple file defining various graph data types
// ==========================================================================


pub type NodeID = i64;

pub type NodeName = String;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GraphNode<T> {
    name: T,
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
        r"
<?hh // strict

// ==========================================================================
// Simple file defining various graph data types
// ==========================================================================


type GraphiteIngesterNodeID = int;

type GraphiteIngesterNodeName = string;

type GraphiteIngesterGraphNode<T> = shape(
    'name' => T,
);

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


// ==========================================================================
// Simple file defining various graph data types
// ==========================================================================


export type NodeID = number;

export type NodeName = string;

export type GraphNode<T> = {|
    'name': T,
|};

"
    );
    Ok(())
}
