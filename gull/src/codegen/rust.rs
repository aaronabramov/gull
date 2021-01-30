use super::Codegen;
use crate::definitions::{PrimitiveType, StructField, StructFieldType, TypeDeclaration};
use anyhow::Result;

pub struct RustCodegen;

impl Codegen for RustCodegen {
    fn gen_declarations(declarations: &Vec<TypeDeclaration>) -> Result<String> {
        let mut r = String::new();

        for declaration in declarations {
            r.push('\n');

            r.push_str(&Self::gen_declaration(declaration)?);

            r.push('\n');
        }

        Ok(r)
    }
    fn gen_declaration(declaration: &TypeDeclaration) -> Result<String> {
        let r = match declaration {
            TypeDeclaration::PrimitiveType { name, value } => {
                format!("type {} = {};", name, gen_primitive_type(value))
            }
            TypeDeclaration::Struct { name, fields } => {
                format!(
                    r#"struct {} {{ 
{}
}}"#,
                    name,
                    gen_struct_fields(fields)
                )
            }
        };

        Ok(r)
    }
}

fn gen_primitive_type(ty: &PrimitiveType) -> &'static str {
    match ty {
        PrimitiveType::String => "String",
        PrimitiveType::Tbool => "bool",
        PrimitiveType::Ti32 => "i32",
    }
}

fn gen_struct_fields(fields: &[StructField]) -> String {
    let mut r = String::new();

    for field in fields {
        let field_type = match field {};

        r.push_str(&format!("    {}: {},", field.name, field_type));
    }

    r
}
