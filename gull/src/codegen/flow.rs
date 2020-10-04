use crate::{Codegen, GullType, GullTypeDecl};

pub struct Flow;

impl Codegen for Flow {
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
                let mut result = format!("\n export type {} = {{\n", name);

                for (name, field_ty) in fields {
                    result.push_str(&format!("  {}: {},\n", name, Self::gen_type(field_ty)));
                }
                result.push_str("};\n");

                result
            }
            GullType::TEnum(variants) => {
                let mut result = format!("\n export type {} = \n", name);

                for (variant, variant_args) in variants {
                    result.push_str(&format!("// variant {}", variant));
                    result.push_str("[");
                    for arg in variant_args {
                        result.push_str(&format!("{},", Self::gen_type(arg)));
                    }

                    result.push_str("]");

                    result.push_str(";\n");
                }
                result.push_str("};\n");

                result
            }
            _ => todo!("can only declare records and enums"),
        }
    }

    fn gen_type(ty: &GullType) -> String {
        match ty {
            GullType::TString => "string".to_string(),
            GullType::Ti32 => "number".to_string(),
            GullType::TVec(vec_ty) => format!("Array<{}>", Self::gen_type(&vec_ty)),
            GullType::TSymbol(sym_name) => sym_name.to_string(),
            // this is different from Rust thing where we still don't have anonymous records
            // code generated because it involves some extra stuff in Rust
            GullType::TRecord(_) => Self::gen_type(ty),
            GullType::TEnum(_) => panic!("I haven't thought about anonymous enums yet"),
        }
    }
}
