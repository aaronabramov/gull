pub mod declarations;
pub use declarations::Declarations;

#[derive(Debug, Clone)]
pub enum TPrimitive {
    String,
    Ti64,
    Tf64,
    Tbool,
    TGeneric(TGeneric),
}

#[derive(Debug, Clone)]
pub enum TOption {
    Reference(TypeDeclaration),
    TMap(TMap),
    TPrimitive(TPrimitive),
    TSet(TSet),
    TVec(TVec),
}

#[derive(Debug, Clone)]
pub enum TVec {
    TPrimitive(TPrimitive),
    Reference(TypeDeclaration),
}

#[derive(Debug, Clone)]
pub enum TSet {
    TPrimitive(TPrimitive),
    Reference(TypeDeclaration),
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

#[derive(Debug, Clone, Copy)]
pub struct TGeneric {
    pub name: &'static str,
    pub bounds: Option<&'static str>,
}

#[derive(Debug, Clone)]
pub enum TMapValue {
    TPrimitive(TPrimitive),
    Reference(TypeDeclaration),
    TSet(TSet),
}

#[derive(Debug, Clone)]
pub struct TTuple {
    pub items: Vec<TupleItem>,
}

#[derive(Debug, Clone)]
pub enum TupleItem {
    Reference(TypeDeclaration),
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
    TMap(Box<TMap>),
    TPrimitive(TPrimitive),
    TStruct(TStruct),
    TTuple(TTuple),
    Docs,
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
}

#[derive(Debug, Clone)]
pub enum StructFieldType {
    Reference(TypeDeclaration),
    TMap(TMap),
    TSet(TSet),
    TOption(TOption),
    TPrimitive(TPrimitive),
    TTuple(TTuple),
    TVec(TVec),
    TGeneric(TGeneric),
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
    Empty,
    Tuple(TTuple),
    Struct(TStruct),
    Primitive(TPrimitive),
}
