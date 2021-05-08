use super::{TReference, TypeDeclaration};
use crate::codegen::{Codegen, FlowCodegen, HackCodegen, RustCodegen};
use anyhow::Result;

#[derive(Debug)]
pub struct Declarations {
    pub(crate) declarations: Vec<TypeDeclaration>,
    pub(crate) config: Vec<DeclarationsConfig>,
}

impl Declarations {
    pub fn new() -> Self {
        Declarations {
            declarations: vec![],
            config: vec![],
        }
    }

    pub fn add_config(&mut self, config: DeclarationsConfig) {
        self.config.push(config)
    }

    pub fn add(&mut self, type_declaration: TypeDeclaration) -> TReference {
        let reference = TReference {
            name: type_declaration.name,
        };
        self.declarations.push(type_declaration);
        reference
    }

    pub fn codegen_rust(&self) -> Result<String> {
        RustCodegen::gen_declarations(&self)
    }

    pub fn codegen_hack(&self) -> Result<String> {
        HackCodegen::gen_declarations(&self)
    }

    pub fn codegen_flow(&self) -> Result<String> {
        FlowCodegen::gen_declarations(&self)
    }
}

#[derive(Debug)]
pub enum DeclarationsConfig {
    FileHeader(&'static str),
    HackNamespace(&'static str),
}
