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
                let mut result = format!("\nexport type {} = {{\n", name);

                for (name, field_ty) in fields {
                    result.push_str(&format!("  {}: {},\n", name, Self::gen_type(field_ty)));
                }
                result.push_str("};\n");

                result
            }
            GullType::TEnum(variants) => {
                let mut result = format!("\nexport type {} = ", name);

                let mut variant_iter = variants.iter();

                if let Some((first_variant, _)) = variant_iter.next() {
                    result.push_str(first_variant);
                }

                for (variant, _) in variant_iter {
                    result.push_str(&format!(" | {}", variant));
                }
                result.push_str(";\n");

                for (variant, variant_args) in variants {
                    let lower_cased = {
                        let mut r = variant.clone();
                        if let Some(r) = r.get_mut(0..1) {
                            r.make_ascii_lowercase();
                        };
                        r
                    };

                    result.push_str(&format!("export type {} = ", variant));
                    result.push_str(&format!("{{| {}: ", lower_cased));
                    result.push('[');

                    let mut variant_args_iter = variant_args.iter();

                    if let Some(arg) = variant_args_iter.next() {
                        result.push_str(&Self::gen_type(arg));
                    }

                    for arg in variant_args_iter {
                        result.push_str(&format!(", {}", Self::gen_type(arg)));
                    }

                    result.push(']');

                    result.push_str(" |};\n");
                }

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
