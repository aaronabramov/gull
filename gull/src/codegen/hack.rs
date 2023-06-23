use super::docs::{format_docstring, CommentStyle};
use super::shared;
use super::Codegen;
use crate::prelude::*;
use anyhow::Result;
use convert_case::{Case, Casing};

pub struct HackCodegen {
    namespace: &'static str,
}

impl Codegen for HackCodegen {
    fn gen_declarations(declarations: &Declarations) -> Result<String> {
        let mut rc = HackCodegen { namespace: "" };

        let mut declarations_code = String::from("<?hh\n");

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
                format!(
                    "type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_primitive_type(p)
                )
            }
            DeclarationValue::TMap(m) => {
                format!(
                    "type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_map(m)
                )
            }
            DeclarationValue::TVec(v) => {
                format!(
                    "type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_vec(v)
                )
            }
            DeclarationValue::TOption(o) => {
                format!(
                    "type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_option(o)
                )
            }
            DeclarationValue::TTuple(t) => {
                format!(
                    "type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_tuple(t)
                )
            }
            DeclarationValue::TStruct(s) => {
                format!(
                    "type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_struct(s, 0)
                )
            }
            DeclarationValue::TEnum(e) => self.gen_enum(&name, &declaration.generic_params, e),
            DeclarationValue::TSimpleEnum(e) => self.gen_simple_enum(&name, &e.variants),
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
        format!("?{}", self.gen_option_value(o))
    }

    fn gen_option_value(&self, o: &TOption) -> String {
        match &o {
            TOption::TPrimitive(p) => self.gen_primitive_type(&p),
            TOption::TMap(m) => self.gen_map(m),
            TOption::TVec(v) => self.gen_vec(v),
            TOption::TSet(s) => self.gen_set(s),
            TOption::TTuple(t) => self.gen_tuple(t),
        }
    }

    fn gen_struct(&self, s: &TStruct, indent: usize) -> String {
        let mut fields = String::new();

        let prefix = " ".repeat(indent);

        let mut is_option = "";

        for field in &s.fields {
            let mut field_type = match &field.field_type {
                StructFieldType::TMap(m) => self.gen_map(m),
                StructFieldType::TSet(s) => self.gen_set(s),
                StructFieldType::TPrimitive(p) => self.gen_primitive_type(&p),
                StructFieldType::TTuple(t) => self.gen_tuple(t),
                StructFieldType::TVec(v) => self.gen_vec(v),
                StructFieldType::TOption(o) => {
                    is_option = "?";
                    self.gen_option_value(o)
                }
            };

            field_type = format!(
                "\n    {}{}'{}' => {},",
                &prefix, is_option, field.name, field_type
            );
            // reset option so that other fields after it don't all become options
            is_option = "";

            if let Some(doc) = format_docstring(field.docs, CommentStyle::DoubleSlash, indent + 4) {
                field_type = format!("\n{}{}", doc, field_type);
            }

            fields.push_str(&field_type);
        }

        format!("shape({}\n{})", fields, prefix)
    }

    fn gen_simple_enum(&self, name: &str, variants: &[&str]) -> String {
        let mut variant_lines = vec![];

        for name in variants {
            variant_lines.push(format!(
                r#"    {} = "{}";{}"#,
                name.to_case(Case::ScreamingSnake),
                name,
                "\n"
            ));
        }

        format!(
            "enum {}: string as string {{\n{}}}",
            name,
            variant_lines.join("")
        )
    }

    fn gen_enum(&self, name: &str, generic_params: &[TGeneric], e: &TEnum) -> String {
        let variant_type_enum_name = format!("{}Type", name);

        let variant_names = e.variants.iter().map(|v| v.name).collect::<Vec<_>>();
        let simple_enum = self.gen_simple_enum(&variant_type_enum_name, &variant_names);

        let mut variants = String::new();

        for variant in &e.variants {
            let mut variant_type = match &variant.variant_type {
                EnumVariantType::TStruct(s) => format!(" {}", self.gen_struct(s, 4)),
                EnumVariantType::TPrimitive(p) => self.gen_primitive_type(p),
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

type {}{} = shape({}\n);",
            simple_enum,
            name,
            shared::generic_params(generic_params, |g| self.gen_generic(g)),
            variants
        )
    }

    fn gen_tuple(&self, t: &TTuple) -> String {
        let mut values = String::new();

        for (n, item) in t.items.iter().enumerate() {
            let is_last = n == t.items.len() - 1;

            let value = match item {
                TupleItem::TPrimitive(p) => self.gen_primitive_type(p).to_string(),
                TupleItem::TOption(o) => self.gen_option(o),
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
            TPrimitive::Ti32 => "int".to_string(),
            TPrimitive::Tu32 => "int".to_string(),
            TPrimitive::Tusize => "int".to_string(),
            TPrimitive::THardcoded(s) => s.to_string(),
            TPrimitive::TVec(v) => self.gen_vec(v),
            TPrimitive::TMap(m) => self.gen_map(m),
            TPrimitive::TOption(o) => self.gen_option(o),
            TPrimitive::TDifferentPerLanguage { hack, .. } => self.gen_primitive_type(&hack),
            TPrimitive::TGeneric(g) => self.gen_generic(g),
            TPrimitive::TReference(r) => {
                format!(
                    "{}{}",
                    self.gen_namespaced_name(r.get_name()),
                    shared::generic_params(&r.generic_params, |g| self.gen_generic(g))
                )
            }
        }
    }

    // Get **namespaced** declaration/reference name
    fn gen_namespaced_name(&self, name: &str) -> String {
        format!("{}{}", self.namespace, name)
    }

    fn gen_generic(&self, g: &TGeneric) -> String {
        match g {
            TGeneric::TDefinition { name, .. } => name.to_string(),
            TGeneric::TReference(r, ..) => {
                self.gen_primitive_type(&TPrimitive::TReference(r.clone()))
            }
        }
    }

    fn gen_code_block(&self, b: &CodeBlock) -> String {
        match b {
            CodeBlock::Rust(_) => String::new(),
            CodeBlock::Flow(_) => String::new(),
            CodeBlock::Hack(lines) => lines
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}
