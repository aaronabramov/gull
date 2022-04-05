use super::docs::{format_docstring, CommentStyle};
use super::{shared, Codegen};
use crate::prelude::*;
use anyhow::Result;
use convert_case::{Case, Casing};

pub struct FlowCodegen {}

impl Codegen for FlowCodegen {
    fn gen_declarations(declarations: &Declarations) -> Result<String> {
        let rc = FlowCodegen {};

        let mut declarations_code = String::new();

        for config in &declarations.config {
            match config {
                DeclarationsConfig::HackNamespace(_) => {
                    // no op
                }
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

impl FlowCodegen {
    fn gen_declaration(&self, declaration: &TypeDeclaration) -> Result<String> {
        let name = self.gen_name(&declaration);
        let mut r = match &declaration.value {
            DeclarationValue::TPrimitive(p) => {
                format!(
                    "export type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_primitive_type(p)
                )
            }
            DeclarationValue::TMap(m) => {
                format!(
                    "export type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_map(m)
                )
            }
            DeclarationValue::TVec(v) => {
                format!(
                    "export type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_vec(v)
                )
            }
            DeclarationValue::TOption(o) => {
                format!(
                    "export type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_option(o),
                )
            }
            DeclarationValue::TTuple(t) => {
                format!(
                    "export type {}{} = {};",
                    name,
                    shared::generic_params(&declaration.generic_params, |g| self.gen_generic(g)),
                    self.gen_tuple(t)
                )
            }
            DeclarationValue::TStruct(s) => {
                format!(
                    "export type {}{} = {};",
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

        format!("{{[key: {}]: {}}}", self.gen_primitive_type(&m.key), value)
    }

    fn gen_vec(&self, v: &TVec) -> String {
        let value = match &v {
            TVec::TPrimitive(p) => self.gen_primitive_type(p),
        };
        format!("Array<{}>", value)
    }

    fn gen_set(&self, s: &TSet) -> String {
        let value = match &s {
            TSet::TPrimitive(p) => self.gen_primitive_type(p),
        };

        format!("Array<{}>", value)
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

        let indent_prefix = " ".repeat(indent);
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
                "\n    {}'{}'{}: {},",
                &indent_prefix, field.name, is_option, field_type
            );

            // reset option so that other fields after it don't all become options
            is_option = "";

            if let Some(doc) = format_docstring(field.docs, CommentStyle::DoubleSlash, indent + 4) {
                field_type = format!("\n{}{}", doc, field_type);
            }

            fields.push_str(&field_type);
        }

        format!("{{{}\n{}}}", fields, indent_prefix)
    }

    fn gen_simple_enum(&self, name: &str, variants: &[&str]) -> String {
        let ty_def = variants
            .iter()
            .map(|n| format!("\"{}\"", n))
            .collect::<Vec<_>>()
            .join(" | ");
        let ty = format!("export type {} = {};", name, ty_def);

        let value_def = variants
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let value = format!(
            "export enum {}Enum {{{}}};",
            name.to_case(Case::UpperCamel),
            value_def,
        );
        format!("{}\n\n{}", ty, value)
    }

    fn gen_enum(&self, name: &str, generic_params: &[TGeneric], e: &TEnum) -> String {
        let variant_types = e
            .variants
            .iter()
            .map(|v| format!(r#""{}""#, v.name))
            .collect::<Vec<_>>()
            .join(" | ");

        let variant_type_enum_name = format!("{}Type", name);

        let variant_type_hack_enum = format!(
            "export type {} = {};",
            variant_type_enum_name, variant_types
        );

        let mut variants = String::new();

        for variant in &e.variants {
            let mut variant_type = match &variant.variant_type {
                EnumVariantType::TStruct(s) => format!(" {}", self.gen_struct(s, 4)),
                EnumVariantType::TPrimitive(p) => self.gen_primitive_type(p),
            };

            variant_type = format!("\n    '{}'?: {},", variant.name, variant_type);

            if let Some(doc) = format_docstring(variant.docs, CommentStyle::DoubleSlash, 4) {
                variant_type = format!("\n{}{}", doc, variant_type);
            }

            variants.push_str(&variant_type);
        }

        format!(
            "{}

export type {}{} = {{{}\n}};",
            variant_type_hack_enum,
            name,
            shared::generic_params(&generic_params, |g| self.gen_generic(g)),
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

        format!("[{}]", values)
    }

    fn gen_primitive_type(&self, ty: &TPrimitive) -> String {
        match ty {
            TPrimitive::String => "string".to_string(),
            TPrimitive::Tbool => "boolean".to_string(),
            TPrimitive::Ti64 => "number".to_string(),
            TPrimitive::Tf64 => "number".to_string(),
            TPrimitive::THardcoded(s) => s.to_string(),
            TPrimitive::TVec(v) => self.gen_vec(v),
            TPrimitive::TMap(m) => self.gen_map(m),
            TPrimitive::TOption(o) => self.gen_option(o),
            TPrimitive::TDifferentPerLanguage { flow, .. } => self.gen_primitive_type(&flow),
            TPrimitive::TGeneric(g) => self.gen_generic(g),
            TPrimitive::TReference(r) => {
                format!(
                    "{}{}",
                    r.get_name(),
                    shared::generic_params(&r.generic_params, |g| self.gen_generic(g))
                )
            }
        }
    }

    fn gen_name(&self, d: &TypeDeclaration) -> String {
        d.name.to_string()
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
            CodeBlock::Hack(_) => String::new(),
            CodeBlock::Flow(lines) => lines
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}
