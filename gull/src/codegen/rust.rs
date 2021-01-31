use super::Codegen;
use crate::definitions::*;
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
        let r = match &declaration.value {
            DeclarationValue::TPrimitive(p) => {
                format!("type {} = {};", declaration.name, gen_primitive_type(p)).into()
            }
            DeclarationValue::TMap(m) => format!("type {} = {};", declaration.name, gen_map(m)),
            DeclarationValue::TStruct(s) => gen_struct(declaration.name, s),
            _ => panic!(format!("{:?}", declaration)),
        };
        // TypeDeclaration::TPrimitive { name, value } => {
        //                 format!("type {} = {};", name, gen_primitive_type(value))
        //             }
        //             TypeDeclaration::Struct { name, fields } => {
        //                 format!(
        //                     r#"struct {} {{
        // {}
        // }}"#,
        //                     name,
        //                     gen_struct_fields(fields)
        //                 )
        //             }
        //         };

        Ok(r)
    }
}

fn gen_map(m: &TMap) -> String {
    let value = match &m.value {
        TMapValue::TPrimitive(p) => gen_primitive_type(p),
        TMapValue::Reference(d) => d.name,
    };
    format!("BTreeMap<{}, {}>", gen_primitive_type(&m.key), value)
}

fn gen_vec(v: &TVec) -> String {
    let value = match &v {
        TVec::TPrimitive(p) => gen_primitive_type(p),
        TVec::Reference(d) => d.name,
    };
    format!("Vec<{}>", value)
}

fn gen_struct(name: &str, s: &TStruct) -> String {
    let mut fields = String::new();

    for field in &s.fields {
        let field_type = match &field.field_type {
            StructFieldType::Reference(r) => r.name.into(),
            StructFieldType::TPrimitive(p) => gen_primitive_type(&p).into(),
            StructFieldType::TMap(m) => gen_map(m),
            StructFieldType::TVec(v) => gen_vec(v),
        };

        fields.push_str(&format!("\n    {}: {},", field.name, field_type));
    }

    format!(
        "struct {} {{{}
}}",
        name, fields
    )
}

fn gen_primitive_type(ty: &TPrimitive) -> &'static str {
    match ty {
        TPrimitive::String => "String",
        TPrimitive::Tbool => "bool",
        TPrimitive::Ti64 => "i64",
        TPrimitive::Tf64 => "f64",
    }
}
