mod declarations;

pub use declarations::Declarations;

#[derive(Debug, Clone)]
pub enum TPrimitive {
    String,
    Ti64,
    Tf64,
    Tbool,
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

#[derive(Debug, Clone)]
pub struct TMap {
    pub key: TPrimitive,
    pub value: TMapValue,
}

#[derive(Debug, Clone)]
pub enum TMapValue {
    TPrimitive(TPrimitive),
    Reference(TypeDeclaration),
}

#[derive(Debug, Clone)]
pub struct TypeDeclaration {
    pub name: &'static str,
    pub value: DeclarationValue,
}

#[derive(Debug, Clone)]
pub enum DeclarationValue {
    TPrimitive(TPrimitive),
    TStruct(TStruct),
    TVec(Box<TVec>),
    TSet(Box<TSet>),
    TMap(Box<TMap>),
}

#[derive(Debug, Clone)]
pub struct TStruct {
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: &'static str,
    pub field_type: StructFieldType,
}

#[derive(Debug, Clone)]
pub enum StructFieldType {
    TMap(TMap),
    TVec(TVec),
    TPrimitive(TPrimitive),
    Reference(TypeDeclaration),
}
