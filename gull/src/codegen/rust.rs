use crate::{Codegen, StructDef, TypeDef};

pub struct Rust;

impl Codegen for Rust {
    fn gen_struct(s: &StructDef) -> String {
        let mut result = format!("\nstruct {} {{\n", s.name);

        for (name, def) in &s.fields {
            result.push_str(&format!("  pub {}: {},\n", name, Self::gen_type(def)));
        }
        result.push_str("}\n");

        result
    }

    fn gen_type(t: &TypeDef) -> String {
        let s = match t {
            TypeDef::TString => "String",
            TypeDef::Ti32 => "i32",
            TypeDef::TStructRef(s) => &s.name,
        };

        s.to_string()
    }

    fn gen_list(l: Vec<StructDef>) -> String {
        let mut result = String::from("");

        for s in l {
            result.push_str(&Self::gen_struct(&s));
        }

        result
    }
}
