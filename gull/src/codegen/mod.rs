mod hack;
mod rust;

use crate::definitions::TypeDeclaration;
use anyhow::Result;

pub use hack::HackCodegen;
pub use rust::RustCodegen;

pub trait Codegen {
    fn gen_declarations(declarations: &Vec<TypeDeclaration>) -> Result<String>;
}
