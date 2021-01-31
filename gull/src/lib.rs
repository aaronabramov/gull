mod codegen;
mod definitions;
pub mod generator;
pub mod sign_source;

pub use generator::write_generated;

pub mod prelude {
    pub use crate::codegen::*;
    pub use crate::definitions::declarations::*;
    pub use crate::definitions::*;
    pub use crate::generator::*;
}
