#![allow(clippy::new_without_default)]
#![allow(clippy::needless_borrow)]

/*!

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
![Rust CI](https://github.com/aaronabramov/gull/workflows/Rust%20CI/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/gull.svg
[crates-url]: https://crates.io/crates/gull
[docs-badge]: https://docs.rs/gull/badge.svg
[docs-url]: https://docs.rs/gull

![gull_header](https://user-images.githubusercontent.com/940133/94375072-8dfb7380-00d6-11eb-8611-a2d8d794ef3b.png)

`Gull` is a tool that takes abstract static type definitions and generates
static types definitions into multiple languages.
Currently supported languages: Rust, Hack (PHP), Flow (unstable)

The core assumption is that serializing this type to JSON in any language
produces a JSON string that can be safely parsed into the same type in another
language.

The goal is to generate user friendly types that can be used in application logic
directly. It also copies all associated documentation for each type or struct field
to every single destination target.

NOTE: This is not an RPC framework and does not support any kind of message passing.
It only generates types and fully relies on languages implementations of JSON
serialization while being agnostic to how these strings are passed between environments.
Resulting JSON strings can be passed through a tmp file on a filesystem, intermediate storage
in a database, STDIO, JSON string over http or any other RPC protocols (e.g. Thrift) or any
other methods.


Example:

```
use gull::prelude::*;
use k9::snapshot;

let mut declarations = Declarations::new();

declarations.add(TypeDeclaration {
    name: "Frame",
    docs: "Frame represents a tuple of an Timestamp (RFC3339) and an ID",
    config: vec![TypeDeclarationConfig::RustAttribute("#[derive(Copy, Clone)]")],
    generic_params: vec![],
    value: DeclarationValue::TTuple(TTuple {
        items: vec![
            TupleItem::TPrimitive(TPrimitive::String),
            TupleItem::TPrimitive(TPrimitive::Ti64),
        ],
    }),
});

snapshot!(
        declarations.codegen_rust().unwrap().trim(),
        "
#[derive(Copy, Clone)]
/// Frame represents a tuple of an Timestamp (RFC3339) and an ID
pub type Frame = (String, i64);
"
);

snapshot!(
        declarations.codegen_hack().unwrap().trim(),
        "
<?hh

// Frame represents a tuple of an Timestamp (RFC3339) and an ID
type Frame = (string, int);
"
);

snapshot!(
        declarations.codegen_flow().unwrap().trim(),
        "
// Frame represents a tuple of an Timestamp (RFC3339) and an ID
export type Frame = [string, number];
"
    );
```

for more examples see `gull/e2e/basic_codegen_test.rs`

These types can be safely passed across the boundaries when de\serialized to
and from JSON.

```dont_run
// In rust
use generated_types::Frame;
let thing: Frame = ("2020-01-01:00:00:00Z", 1);
let json = serde_json::to_string(&thing).unwrap();
write_to_file("/tmp/my_thing.json", &json);

// in JS
import type {Frame} from './generated_types.js';
const getFrame = (file_path: string): Frame => {
    return JSON.parse(read_file(file_path));
}
const json: Frame = getFrame("/tmp/my_thing.json");

```
 */

mod codegen;
mod definitions;

pub mod generator;
#[cfg(feature = "sign_source")]
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
