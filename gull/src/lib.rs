use std::collections::BTreeMap;

pub mod codegen;
pub mod sign_source;

#[derive(Clone)]
pub struct StructDef {
    pub name: &'static str,
    pub fields: Vec<(&'static str, TypeDef)>,
}

#[derive(Clone)]
pub enum TypeDef {
    TString,
    Ti32,
    TStructRef(StructDef),
}

pub trait Codegen {
    fn gen_struct(s: &StructDef) -> String;
    fn gen_type(t: &TypeDef) -> String;
    fn gen_list(l: Vec<StructDef>) -> String;
}

#[derive(Clone)]
pub enum GullType {
    TString,
    Ti32,
    TRecord(BTreeMap<String, GullType>),
    TEnum(BTreeMap<String, Vec<GullType>>),
    TVec(Box<GullType>),
    TSymbol(&'static str),
}

// a Type declaration is a pairing of a name with a gull type,
// currently, we only allow new record and enum types to be declared
pub struct GullTypeDecl {
    pub name: String,
    pub gull_type: GullType,
}

pub trait Codegen1 {
    // from a list of type declarations, generate the struct/shapes declarations
    // in the target language
    fn gen_decls(decls: Vec<GullTypeDecl>) -> String;

    // how to print a single type declaration
    fn gen_type_decl(t: &GullTypeDecl) -> String;

    // how to print a type (without a name)
    // currently neither records or enums should be printable here
    fn gen_type_(t: &GullType) -> String;
}
