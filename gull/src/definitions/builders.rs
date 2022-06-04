#![allow(clippy::return_self_not_must_use)]

use super::*;
use anyhow::Result;

macro_rules! unwrap_opt {
    ( $builder:ident,  $optional_field:ident ) => {{
        let unwrapped = $builder.$optional_field.clone().expect(&format!(
            "
            ```````````````````````````````````````````````
            Builder is not finished because the value of the
            field `{}` is not set.
            Builder: `{:?}`\n
            ```````````````````````````````````````````````
            ",
            stringify!($optional_field),
            &$builder,
        ));
        unwrapped
    }};
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn decl<'a>(d: &'a mut Declarations, name: &'static str) -> TypeDeclarationBuilder<'a> {
    TypeDeclarationBuilder {
        d,
        name,
        docs: "",
        value: None,
        config: vec![build_derives(vec![
            "serde::Serialize",
            "serde::Deserialize",
            "Debug",
            "Clone",
        ])],
        generic_params: vec![],
    }
}

#[derive(Debug)]
pub struct TypeDeclarationBuilder<'a> {
    d: &'a mut Declarations,
    name: &'static str,
    docs: &'static str,
    value: Option<DeclarationValue>,
    config: Vec<TypeDeclarationConfig>,
    generic_params: Vec<TGeneric>,
}

impl<'a> TypeDeclarationBuilder<'a> {
    pub fn value<T: Into<DeclarationValue>>(mut self, value: T) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn build(self) -> TReference {
        self.d.add(TypeDeclaration {
            name: self.name,
            docs: self.docs,
            value: unwrap_opt!(self, value),
            config: self.config,
            generic_params: self.generic_params,
        })
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

impl From<TStruct> for DeclarationValue {
    fn from(s: TStruct) -> Self {
        DeclarationValue::TStruct(s)
    }
}

impl From<TEnum> for DeclarationValue {
    fn from(s: TEnum) -> Self {
        DeclarationValue::TEnum(s)
    }
}
/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn build_struct() -> TStructBuilder {
    TStructBuilder { fields: vec![] }
}

#[derive(Debug, Clone)]
pub struct TStructBuilder {
    pub fields: Vec<StructField>,
}

impl TStructBuilder {
    pub fn build(self) -> TStruct {
        TStruct {
            fields: self.fields,
        }
    }
}

impl TStructBuilder {
    pub fn field(mut self, f: StructField) -> Self {
        self.fields.push(f);
        self
    }
}

impl TStruct {
    pub fn decl(self) -> DeclarationValue {
        DeclarationValue::TStruct(self)
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn build_struct_field(name: &'static str) -> StructFieldBuilder {
    StructFieldBuilder {
        name,
        ..Default::default()
    }
}

#[derive(Default, Debug)]
pub struct StructFieldBuilder {
    name: &'static str,
    pub docs: &'static str,
    pub field_type: Option<StructFieldType>,
    pub config: Vec<StructFieldConfig>,
}

impl StructFieldBuilder {
    pub fn build(self) -> StructField {
        StructField {
            name: self.name,
            docs: self.docs,
            field_type: unwrap_opt!(self, field_type),
            config: self.config,
        }
    }

    pub fn field_type<T: Into<StructFieldType>>(mut self, t: T) -> Self {
        self.field_type = Some(t.into());
        self
    }

    pub fn optional(mut self) -> Self {
        let field_type = self
            .field_type
            .expect("must set field type before making it optional");
        let field_type = match field_type {
            StructFieldType::TMap(map) => StructFieldType::TOption(TOption::TMap(map)),
            StructFieldType::TSet(set) => StructFieldType::TOption(TOption::TSet(set)),
            StructFieldType::TOption(opt) => StructFieldType::TOption(opt),
            StructFieldType::TPrimitive(p) => StructFieldType::TOption(TOption::TPrimitive(p)),
            StructFieldType::TTuple(t) => StructFieldType::TOption(TOption::TTuple(t)),
            StructFieldType::TVec(v) => StructFieldType::TOption(TOption::TVec(v)),
        };
        self.field_type = Some(field_type);
        self
    }
}

impl From<TPrimitive> for StructFieldType {
    fn from(p: TPrimitive) -> Self {
        StructFieldType::TPrimitive(p)
    }
}

impl From<TVec> for StructFieldType {
    fn from(v: TVec) -> Self {
        StructFieldType::TVec(v)
    }
}

impl From<TMap> for StructFieldType {
    fn from(v: TMap) -> Self {
        StructFieldType::TMap(v)
    }
}

impl From<TSet> for StructFieldType {
    fn from(v: TSet) -> Self {
        StructFieldType::TSet(v)
    }
}

impl From<TReference> for StructFieldType {
    fn from(v: TReference) -> Self {
        StructFieldType::TPrimitive(v.primitive())
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn build_enum() -> TEnumBuilder {
    TEnumBuilder { variants: vec![] }
}

pub struct TEnumBuilder {
    variants: Vec<EnumVariant>,
}

impl TEnumBuilder {
    pub fn build(self) -> TEnum {
        TEnum {
            variants: self.variants,
        }
    }

    pub fn variant(mut self, v: EnumVariant) -> Self {
        self.variants.push(v);
        self
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn build_enum_variant(name: &'static str) -> EnumVariantBuilder {
    EnumVariantBuilder {
        name,
        docs: "",
        variant_type: None,
    }
}

#[derive(Debug)]
pub struct EnumVariantBuilder {
    pub name: &'static str,
    pub docs: &'static str,
    pub variant_type: Option<EnumVariantType>,
}

impl EnumVariantBuilder {
    pub fn build(self) -> EnumVariant {
        EnumVariant {
            name: self.name,
            docs: self.docs,
            variant_type: unwrap_opt!(self, variant_type),
        }
    }

    pub fn variant_type<T: Into<EnumVariantType>>(mut self, t: T) -> Self {
        self.variant_type = Some(t.into());
        self
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

impl From<TPrimitive> for EnumVariantType {
    fn from(p: TPrimitive) -> Self {
        EnumVariantType::TPrimitive(p)
    }
}

impl From<TReference> for EnumVariantType {
    fn from(p: TReference) -> Self {
        EnumVariantType::TPrimitive(p.primitive())
    }
}

impl From<TStruct> for EnumVariantType {
    fn from(s: TStruct) -> Self {
        EnumVariantType::TStruct(s)
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn build_vec<T: Into<TPrimitive>>(t: T) -> TVec {
    TVec::TPrimitive(t.into())
}

pub fn build_map<K: Into<TPrimitive>, V: Into<TPrimitive>>(k: K, v: V) -> TMap {
    TMap {
        key: k.into(),
        value: TMapValue::TPrimitive(v.into()),
        t: TMapType::BTree,
    }
}

pub fn build_set<T: Into<TPrimitive>>(t: T) -> TSet {
    TSet::TPrimitive(t.into())
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

impl From<TReference> for TPrimitive {
    fn from(t: TReference) -> Self {
        t.primitive()
    }
}

/********************************************************************************/
/********************************************************************************/
/********************************************************************************/

pub fn build_derives(derives: Vec<&str>) -> TypeDeclarationConfig {
    let derives = derives
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let s = Box::leak(format!("#[derive({})]", derives).into_boxed_str());
    TypeDeclarationConfig::RustAttribute(s)
}
/********************************************************************************/
/********************************************************************************/
/********************************************************************************/
