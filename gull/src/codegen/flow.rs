use crate::{Codegen, StructDef, TypeDef};

pub struct Flow;

impl Codegen for Flow {
    fn gen_struct(s: &StructDef) -> String {
        let mut result = format!("\ntype {} = {{\n", s.name);

        for (name, def) in &s.fields {
            result.push_str(&format!("  {}: {},\n", name, Self::gen_type(def)));
        }
        result.push_str("};\n");

        result
    }

    fn gen_type(t: &TypeDef) -> String {
        let s = match t {
            TypeDef::TString => "string",
            TypeDef::Ti32 => "number",
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
