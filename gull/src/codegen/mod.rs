mod docs;
mod flow;
mod hack;
mod rust;
mod shared;

use crate::definitions::Declarations;
use anyhow::Result;

pub use flow::FlowCodegen;
pub use hack::HackCodegen;
pub use rust::RustCodegen;

pub trait Codegen {
    fn gen_declarations(declarations: &Declarations) -> Result<String>;
}
