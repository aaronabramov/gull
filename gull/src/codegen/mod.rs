mod rust;

use crate::definitions::TypeDeclaration;
use anyhow::Result;

pub use rust::RustCodegen;

pub trait Codegen {
    fn gen_declarations(declarations: &Vec<TypeDeclaration>) -> Result<String>;
    fn gen_declaration(declaration: &TypeDeclaration) -> Result<String>;
}
