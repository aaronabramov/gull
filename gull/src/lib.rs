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
