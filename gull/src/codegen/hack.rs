use super::docs::{format_docstring, CommentStyle};
use super::shared;
use super::Codegen;
use crate::prelude::*;
use anyhow::Result;

pub struct HackCodegen {
    namespace: &'static str,
}

impl Codegen for HackCodegen {
    fn gen_declarations(declarations: &Declarations) -> Result<String> {
        let mut rc = HackCodegen { namespace: "" };

        let mut declarations_code = String::from("<?hh // strict\n");

        for config in &declarations.config {
            match config {
                DeclarationsConfig::HackNamespace(namespace) => rc.namespace = namespace,
                DeclarationsConfig::FileHeader(header) => {
                    declarations_code.push_str(&format!("{}\n", header));
                }
            }
        }

        for declaration in &declarations.declarations {
            declarations_code.push('\n');
            declarations_code.push_str(&rc.gen_declaration(declaration)?);
            declarations_code.push('\n');
        }

        Ok(declarations_code)
    }
}

impl HackCodegen {
    fn gen_declaration(&self, declaration: &TypeDeclaration) -> Result<String> {
        let name = self.gen_namespaced_name(declaration.name);
        let mut r = match &declaration.value {
            DeclarationValue::TPrimitive(p) => {
                format!("type {} = {};", name, self.gen_primitive_type(p))
            }
            DeclarationValue::TMap(m) => {
                format!("type {} = {};", name, self.gen_map(m))
            }
            DeclarationValue::TTuple(t) => {
                format!("type {} = {};", name, self.gen_tuple(t))
            }
            DeclarationValue::TStruct(s) => {
                format!(
                    "type {}{} = {};",
                    name,
                    self.gen_generic_params(&s.generic_params),
                    self.gen_struct(s, 0)
                )
            }
            DeclarationValue::TEnum(e) => self.gen_enum(&name, e),
            DeclarationValue::Docs => String::new(),
            DeclarationValue::CodeBlock(b) => self.gen_code_block(b),
        };

        if let Some(doc) = format_docstring(declaration.docs, CommentStyle::DoubleSlash, 0) {
            r = format!("{}\n{}", doc, r);
        }

        Ok(r)
    }

    fn gen_map(&self, m: &TMap) -> String {
        let value = match &m.value {
            TMapValue::TPrimitive(p) => self.gen_primitive_type(p),
            TMapValue::TSet(s) => self.gen_set(s),
        };

        format!("dict<{}, {}>", self.gen_primitive_type(&m.key), value)
    }

    fn gen_vec(&self, v: &TVec) -> String {
        let value = match &v {
            TVec::TPrimitive(p) => self.gen_primitive_type(p),
        };
        format!("vec<{}>", value)
    }

    fn gen_set(&self, s: &TSet) -> String {
        let value = match &s {
            TSet::TPrimitive(p) => self.gen_primitive_type(p),
        };

        format!("keyset<{}>", value)
    }

    fn gen_option(&self, o: &TOption) -> String {
        let value = match &o {
            TOption::TPrimitive(p) => self.gen_primitive_type(&p),
            TOption::TMap(m) => self.gen_map(m),
            TOption::TVec(v) => self.gen_vec(v),
            TOption::TSet(s) => self.gen_set(s),
        };
        format!("?{}", value)
    }

    fn gen_struct(&self, s: &TStruct, indent: usize) -> String {
        let mut fields = String::new();

        let prefix = " ".repeat(indent);

        for field in &s.fields {
            let mut field_type = match &field.field_type {
                StructFieldType::TMap(m) => self.gen_map(m),
                StructFieldType::TSet(s) => self.gen_set(s),
                StructFieldType::TOption(o) => self.gen_option(o),
                StructFieldType::TPrimitive(p) => self.gen_primitive_type(&p),
                StructFieldType::TTuple(t) => self.gen_tuple(t),
                StructFieldType::TVec(v) => self.gen_vec(v),
                StructFieldType::TGeneric(TGeneric { name, .. }) => name.to_string(),
            };

            field_type = format!("\n    {}'{}' => {},", &prefix, field.name, field_type);

            if let Some(doc) = format_docstring(field.docs, CommentStyle::DoubleSlash, indent + 4) {
                field_type = format!("\n{}{}", doc, field_type);
            }

            fields.push_str(&field_type);
        }

        format!("shape({}\n{})", fields, prefix)
    }

    fn gen_enum(&self, name: &str, e: &TEnum) -> String {
        let mut variant_types = vec![];

        for variant in &e.variants {
            variant_types.push(format!(
                r#"    {} = "{}";{}"#,
                variant.name.to_uppercase(),
                variant.name,
                "\n"
            ));
        }

        let variant_type_enum_name = format!("{}Type", name);

        let variant_type_hack_enum = format!(
            "enum {}: string as string {{\n{}}}",
            variant_type_enum_name,
            variant_types.join("")
        );

        let mut variants = String::new();

        variants.push_str(&format!("\n    'type' => {},", variant_type_enum_name));

        for variant in &e.variants {
            let mut variant_type = match &variant.variant_type {
                EnumVariantType::Empty => "null".to_string(),
                EnumVariantType::Tuple(t) => self.gen_tuple(t),
                EnumVariantType::Struct(s) => format!(" {}", self.gen_struct(s, 4)),
                EnumVariantType::Primitive(p) => self.gen_primitive_type(p),
            };

            variant_type = format!("\n    ?'{}' => {},", variant.name, variant_type);

            if let Some(doc) = format_docstring(variant.docs, CommentStyle::DoubleSlash, 4) {
                variant_type = format!("\n{}{}", doc, variant_type);
            }

            variants.push_str(&variant_type);
        }

        format!(
            "
{}

type {} = shape({}\n);",
            variant_type_hack_enum, name, variants
        )
    }

    fn gen_tuple(&self, t: &TTuple) -> String {
        let mut values = String::new();

        for (n, item) in t.items.iter().enumerate() {
            let is_last = n == t.items.len() - 1;

            let value = match item {
                TupleItem::TPrimitive(p) => self.gen_primitive_type(p).to_string(),
            };

            values.push_str(&value);
            if !is_last {
                values.push_str(", ");
            }
        }

        format!("({})", values)
    }

    fn gen_primitive_type(&self, ty: &TPrimitive) -> String {
        match ty {
            TPrimitive::String => "string".to_string(),
            TPrimitive::Tbool => "bool".to_string(),
            TPrimitive::Ti64 => "int".to_string(),
            TPrimitive::Tf64 => "float".to_string(),
            TPrimitive::TGeneric(TGeneric { name, .. }) => name.to_string(),
            TPrimitive::TReference { r, generic_params } => {
                format!(
                    "{}{}",
                    self.gen_namespaced_name(r.get_name()),
                    shared::generic_params(&generic_params)
                )
            }
        }
    }

    // Get **namespaced** declaration/reference name
    fn gen_namespaced_name(&self, name: &str) -> String {
        format!("{}{}", self.namespace, name)
    }

    fn gen_generic_params(&self, params: &[TGeneric]) -> String {
        if params.is_empty() {
            String::new()
        } else {
            let p = params
                .iter()
                .map(|g| g.name.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("<{}>", p)
        }
    }

    fn gen_code_block(&self, b: &CodeBlock) -> String {
        match b {
            CodeBlock::Rust(_) => String::new(),
        }
    }
}
