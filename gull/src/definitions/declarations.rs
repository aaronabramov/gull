use super::TypeDeclaration;
use crate::codegen::{Codegen, RustCodegen};
use anyhow::Result;

#[derive(Debug)]
pub struct Declarations {
    declarations: Vec<TypeDeclaration>,
}

#[allow(renamed_and_removed_lints)]
#[allow(new_without_default)]
impl Declarations {
    pub fn new() -> Self {
        Declarations {
            declarations: vec![],
        }
    }

    pub fn add(&mut self, type_declaration: TypeDeclaration) -> TypeDeclaration {
        self.declarations.push(type_declaration.clone());
        type_declaration
    }

    pub fn codegen_rust(&self) -> Result<String> {
        RustCodegen::gen_declarations(&self.declarations)
    }
}
