use crate::{Codegen, Codegen1, GullType, GullTypeDecl, StructDef, TypeDef};

pub struct Rust;

impl Codegen for Rust {
    fn gen_struct(s: &StructDef) -> String {
        let mut result = format!("\n#[derive(Debug)]\npub struct {} {{\n", s.name);

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

impl Codegen1 for Rust {
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
                let mut result = format!("\nstruct {} {{\n", name);

                for (name, field_ty) in fields {
                    result.push_str(&format!("  pub {}: {},\n", name, Self::gen_type_(field_ty)));
                }
                result.push_str("}\n");

                result
            }
            GullType::TEnum(variants) => {
                let mut result = format!("\nenum {} {{\n", name);

                for (variant, variant_args) in variants {
                    result.push_str(&format!("  {}(", variant));
                    for arg in variant_args {
                        result.push_str(&format!("{},", Self::gen_type_(arg)));
                    }

                    result.push_str(")\n");
                }
                result.push_str("}\n");

                result
            }
            _ => todo!("can only declare records and enums"),
        }
    }

    fn gen_type_(ty: &GullType) -> String {
        match ty {
            GullType::TString => "String".to_string(),
            GullType::Ti32 => "i32".to_string(),
            GullType::TVec(vec_ty) => format!("Vec<{}>", Self::gen_type_(vec_ty)),
            GullType::TSymbol(sym_name) => sym_name.to_string(),

            GullType::TRecord(_) => panic!("no anonymous records allowed"),
            GullType::TEnum(_) => panic!("no anonymous records allowed"),
        }
    }
}
