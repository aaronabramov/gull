#![allow(clippy::new_without_default)]

/*!

`Gull` is a tool that can generate static type definitions for multiple
languages.

Given abstract definitions of types it can generate source code
containing these type definitions in multiple languages.

Although different languages might have different support for types
(e.g. PHP or JS don't support Rust enums) the assumption is made that
they both serialize to the same JSON object and can be deserialized
safely.

This crate is focused on generating developer friendly types that can
be used in the code directly. It does not intend to solve RPC problems
like other frameworks like Thrift or gRPC are trying to solve (e.g
gull does nothing to make sure types a backward compatible)


Example:

```
use gull::prelude::*;

let mut declarations = Declarations::new();

declarations.add(TypeDeclaration {
    name: "Frame",
    docs: "Frame represents a tuple of an Timestamp (RFC3339) and an ID",
    config: vec![TypeDeclarationConfig::RustAttribute("#[derive(Copy)]")],
    value: DeclarationValue::TTuple(TTuple {
        items: vec![
            TupleItem::TPrimitive(TPrimitive::String),
            TupleItem::TPrimitive(TPrimitive::Ti64),
        ],
    }),
});

assert_eq!(
        declarations.codegen_rust().unwrap(),
        "

#[derive(Copy)]
/// Frame represents a tuple of an Timestamp (RFC3339) and an ID
pub type Frame = (String, i64);
"
    );
```
for more examples see `gull/e2e/basic_codegen_test.rs`
 */

mod codegen;
mod definitions;

pub mod generator;
pub mod sign_source;

pub use generator::write_generated;
// pub use macros::EnumSerialization;

pub mod prelude {
    pub use crate::codegen::*;
    pub use crate::definitions::declarations::*;
    pub use crate::definitions::*;
    pub use crate::generator::*;
}

#[cfg(test)]
mod e2e;
