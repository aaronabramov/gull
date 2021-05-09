use gull::prelude::*;

pub(super) fn derives(derives: Vec<&str>) -> TypeDeclarationConfig {
    let derives = derives
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let s = Box::leak(format!("#[derive({})]", derives).into_boxed_str());
    TypeDeclarationConfig::RustAttribute(s)
}

pub fn make_declarations() -> Declarations {
    let mut d = Declarations::new();
    d.add_config(DeclarationsConfig::HackNamespace("Unigraph"));

    let op_inline = d
        .add(TypeDeclaration {
            name: "OpInline",
            docs: "",
            config: vec![derives(vec!["serde::Serialize", "serde::Deserialize"])],
            value: DeclarationValue::TStruct(TStruct {
                generic_params: vec![],
                fields: vec![StructField {
                    name: "graphs",
                    docs: "",
                    field_type: StructFieldType::TVec(TVec::TPrimitive(TPrimitive::String)),
                    config: vec![],
                }],
            }),
        })
        .primitive(vec![]);

    let op_fetch = d
        .add(TypeDeclaration {
            name: "OpFetch",
            docs: "",
            config: vec![derives(vec!["serde::Serialize", "serde::Deserialize"])],
            value: DeclarationValue::TStruct(TStruct {
                generic_params: vec![],
                fields: vec![
                    StructField {
                        name: "timeline_key",
                        docs: "",
                        field_type: StructFieldType::TPrimitive(TPrimitive::Ti64),
                        config: vec![],
                    },
                    StructField {
                        name: "graph_ids",
                        docs: "",
                        field_type: StructFieldType::TVec(TVec::TPrimitive(TPrimitive::Ti64)),
                        config: vec![],
                    },
                ],
            }),
        })
        .primitive(vec![]);

    d.add(TypeDeclaration {
        name: "OperationsEnum",
        docs: "",
        config: vec![derives(vec!["serde::Serialize", "serde::Deserialize"])],
        value: DeclarationValue::TEnum(TEnum {
            variants: vec![
                EnumVariant {
                    name: "OpInline",
                    docs: "",
                    variant_type: EnumVariantType::Primitive(op_inline),
                },
                EnumVariant {
                    name: "OpFetch",
                    docs: "",
                    variant_type: EnumVariantType::Primitive(op_fetch),
                },
            ],
        }),
    });

    d
}
