mod docs;
mod hack;
mod rust;

use crate::definitions::Declarations;
use anyhow::Result;

pub use hack::HackCodegen;
pub use rust::RustCodegen;

pub trait Codegen {
    fn gen_declarations(declarations: &Declarations) -> Result<String>;
}
