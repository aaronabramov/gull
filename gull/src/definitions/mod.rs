pub mod declarations;
pub use declarations::Declarations;

#[derive(Debug, Clone)]
pub enum TPrimitive {
    String,
    Ti64,
    Tf64,
    Tbool,
    TGeneric(TGeneric),
    TReference(TReference),
    THardcoded(&'static str),
    TDifferentPerLanguege {
        hack: Box<TPrimitive>,
        rust: Box<TPrimitive>,
        flow: Box<TPrimitive>,
    },
}

#[derive(Debug, Clone)]
pub struct TReference {
    // private field. must be returned by declarations
    name: &'static str,
    pub generic_params: Vec<TGeneric>,
}

impl TReference {
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn primitive(&self) -> TPrimitive {
        TPrimitive::TReference(self.clone())
    }
}

#[derive(Debug, Clone)]
pub enum TOption {
    TMap(TMap),
    TPrimitive(TPrimitive),
    TSet(TSet),
    TVec(TVec),
}

#[derive(Debug, Clone)]
pub enum TVec {
    TPrimitive(TPrimitive),
}

#[derive(Debug, Clone)]
pub enum TSet {
    TPrimitive(TPrimitive),
}

#[derive(Debug, Clone, Copy)]
pub enum TMapType {
    Hash,
    BTree,
}

#[derive(Debug, Clone)]
pub struct TMap {
    pub key: TPrimitive,
    pub value: TMapValue,
    pub t: TMapType,
}

#[derive(Debug, Clone)]
pub enum TGeneric {
    // Params that are defined and used within struct/type. no namespacing
    // needed etc.
    TDefinition {
        name: &'static str,
        bounds: Option<&'static str>,
    },
    // generic param that references ather thing. e.g. Vec<OtherTypeReference>
    TReference(TReference),
}

#[derive(Debug, Clone)]
pub enum TMapValue {
    TPrimitive(TPrimitive),
    TSet(TSet),
}

#[derive(Debug, Clone)]
pub struct TTuple {
    pub items: Vec<TupleItem>,
}

#[derive(Debug, Clone)]
pub enum TupleItem {
    TPrimitive(TPrimitive),
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: &'static str,
    pub docs: &'static str,
    pub value: DeclarationValue,
    pub config: Vec<TypeDeclarationConfig>,
}

#[derive(Debug, Clone, Copy)]
pub enum TypeDeclarationConfig {
    RustAttribute(&'static str),
}

#[derive(Debug, Clone)]
pub enum DeclarationValue {
    TEnum(TEnum),
    TSimpleEnum(TSimpleEnum),
    TMap(Box<TMap>),
    TPrimitive(TPrimitive),
    TStruct(TStruct),
    TTuple(TTuple),
    Docs,
    CodeBlock(CodeBlock),
}

#[derive(Debug, Clone)]
pub enum CodeBlock {
    Rust(Vec<&'static str>),
}

#[derive(Debug, Clone)]
pub struct TStruct {
    pub generic_params: Vec<TGeneric>,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: &'static str,
    pub docs: &'static str,
    pub field_type: StructFieldType,
    pub config: Vec<StructFieldConfig>,
}

#[derive(Debug, Clone)]
pub enum StructFieldConfig {
    RustAttribute(&'static str),
    RustOverride(&'static str),
}

#[derive(Debug, Clone)]
pub enum StructFieldType {
    TMap(TMap),
    TSet(TSet),
    TOption(TOption),
    TPrimitive(TPrimitive),
    TTuple(TTuple),
    TVec(TVec),
}

#[derive(Debug, Clone)]
pub struct TEnum {
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: &'static str,
    pub docs: &'static str,
    pub variant_type: EnumVariantType,
}

#[derive(Debug, Clone)]
pub enum EnumVariantType {
    // WARNING: This enum is pretty limited in what it can represent. Since hack
    // and flow don't have enums, we need to be careful at how we represent the
    // serialization of these enums on rust size. E.g.
    // see https://serde.rs/enum-representations.html for details.
    //
    // Default representation puts current varariant as a key in a JSON object
    // (e.g. {"MyVariant" => {"data": 1}})
    //
    // We can't have empty variants, since they would be represented as a single
    // string in json by deault. e.g. `MyEnum::EmptyVariant` will serialize as
    // just "EmtyVariant" string, which will mess up hack static typing. (it has
    // no disjoint/distriminating unions)
    //
    // If we want to represent an empty enum on rust side, it HAS to have some
    // data in it. so en Empty variant should be a primitive type with, e.g. a
    // boolean type inside, so it serializes into `{"EmptyVariant": true}` and
    // can be still represented as a shape/object on hack/js side.
    TStruct(TStruct),
    TPrimitive(TPrimitive),
}

/// This enum is an enum that has NO data associated with its variants.
/// Basically like a C-like enum, where enum values are strings.
/// It will serialize in a single string value (instead of the object/string
/// stuff that we get with full featured enums). This easily maps to hack enums
/// and flow string union types.
#[derive(Debug, Clone)]
pub struct TSimpleEnum {
    pub variants: Vec<&'static str>,
}
