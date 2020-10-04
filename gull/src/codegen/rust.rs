use crate::{Codegen, GullType, GullTypeDecl};

pub struct Rust;

impl Codegen for Rust {
    fn gen_decls(decls: Vec<GullTypeDecl>) -> String {
        let mut result = String::from("");

        for decl in decls {
            result.push_str(&Self::gen_type_decl(&decl));
        }

        result
    }

    fn gen_type_decl(decl: &GullTypeDecl) -> String {
        let GullTypeDecl { name, gull_type } = decl;

        match gull_type {
            GullType::TRecord(fields) => {
                let mut result = format!("\n#[derive(Debug)]\npub struct {} {{\n", name);

                for (name, field_ty) in fields {
                    result.push_str(&format!("  pub {}: {},\n", name, Self::gen_type(field_ty)));
                }
                result.push_str("}\n");

                result
            }
            GullType::TEnum(variants) => {
                let mut result = format!("\n#[derive(Debug)]\npub enum {} {{\n", name);

                for (variant, variant_args) in variants {
                    result.push_str(&format!("  {}(", variant));
                    for arg in variant_args {
                        result.push_str(&format!("{},", Self::gen_type(arg)));
                    }

                    result.push_str("),\n");
                }
                result.push_str("}\n");

                result
            }
            _ => todo!("can only declare records and enums"),
        }
    }

    fn gen_type(ty: &GullType) -> String {
        match ty {
            GullType::TString => "String".to_string(),
            GullType::Ti32 => "i32".to_string(),
            GullType::TVec(vec_ty) => format!("Vec<{}>", Self::gen_type(vec_ty)),
            GullType::TSymbol(sym_name) => sym_name.to_string(),
            GullType::TRecord(_) => panic!("no anonymous records allowed in our rust codegen yet"),
            GullType::TEnum(_) => panic!("I haven't thought about anonymous enums yet"),
        }
    }
}
