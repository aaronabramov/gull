use super::docs::{format_docstring, CommentStyle};
use super::Codegen;
use crate::prelude::*;
use anyhow::Result;

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

        let mut docblock = docblock::SourceFile::from_source(&declarations_code);
        docblock.set_directive("flow", None);
        docblock.set_directive("nolint", None);

        Ok(docblock.to_source())
    }
}

impl FlowCodegen {
    fn gen_declaration(&self, declaration: &TypeDeclaration) -> Result<String> {
        let name = self.gen_name(&declaration);
        let mut r = match &declaration.value {
            DeclarationValue::TPrimitive(p) => {
                format!("export type {} = {};", name, self.gen_primitive_type(p)).into()
            }
            DeclarationValue::TMap(m) => {
                format!("export type {} = {};", name, self.gen_map(m))
            }
            DeclarationValue::TTuple(t) => {
                format!("export type {} = {};", name, self.gen_tuple(t))
            }
            DeclarationValue::TStruct(s) => {
                format!(
                    "export type {}{} = {};",
                    name,
                    self.gen_generic_params(&s.generic_params),
                    self.gen_struct(s, 0)
                )
            }
            DeclarationValue::TEnum(e) => self.gen_enum(&name, e),
            DeclarationValue::Docs => String::new(),
        };

        if let Some(doc) = format_docstring(declaration.docs, CommentStyle::DoubleSlash, 0) {
            r = format!("{}\n{}", doc, r);
        }

        Ok(r)
    }

    fn gen_map(&self, m: &TMap) -> String {
        let value = match &m.value {
            TMapValue::TPrimitive(p) => self.gen_primitive_type(p).to_string(),
            TMapValue::Reference(d) => self.gen_name(d),
            TMapValue::TSet(s) => self.gen_set(s),
        };

        format!("{{[key: {}]: {}}}", self.gen_primitive_type(&m.key), value)
    }

    fn gen_vec(&self, v: &TVec) -> String {
        let value = match &v {
            TVec::TPrimitive(p) => self.gen_primitive_type(p).to_string(),
            TVec::Reference(d) => self.gen_name(d),
        };
        format!("Array<{}>", value)
    }

    fn gen_set(&self, s: &TSet) -> String {
        let value = match &s {
            TSet::TPrimitive(p) => self.gen_primitive_type(p).to_string(),
            TSet::Reference(d) => self.gen_name(d),
        };

        format!("Array<{}>", value)
    }

    fn gen_option(&self, o: &TOption) -> String {
        let value = match &o {
            TOption::Reference(r) => r.name.into(),
            TOption::TPrimitive(p) => self.gen_primitive_type(&p).into(),
            TOption::TMap(m) => self.gen_map(m),
            TOption::TVec(v) => self.gen_vec(v),
            TOption::TSet(s) => self.gen_set(s),
        };
        format!("?{}", value)
    }

    fn gen_struct(&self, s: &TStruct, indent: usize) -> String {
        let mut fields = String::new();

        let indent_prefix = " ".repeat(indent);

        for field in &s.fields {
            let mut field_type = match &field.field_type {
                StructFieldType::Reference(r) => r.name.into(),
                StructFieldType::TMap(m) => self.gen_map(m),
                StructFieldType::TSet(s) => self.gen_set(s),
                StructFieldType::TOption(o) => self.gen_option(o),
                StructFieldType::TPrimitive(p) => self.gen_primitive_type(&p).into(),
                StructFieldType::TTuple(t) => self.gen_tuple(t),
                StructFieldType::TVec(v) => self.gen_vec(v),
                StructFieldType::TGeneric(TGeneric { name, .. }) => name.to_string(),
            };

            field_type = format!("\n    {}'{}': {},", &indent_prefix, field.name, field_type);

            if let Some(doc) = format_docstring(field.docs, CommentStyle::DoubleSlash, indent + 4) {
                field_type = format!("\n{}{}", doc, field_type);
            }

            fields.push_str(&field_type);
        }

        format!("{{|{}\n{}|}}", fields, indent_prefix)
    }

    fn gen_enum(&self, name: &str, e: &TEnum) -> String {
        let variant_types = e
            .variants
            .iter()
            .map(|v| format!(r#""{}""#, v.name))
            .collect::<Vec<_>>()
            .join(" | ");

        let variant_type_enum_name = format!("{}Type", name);

        let variant_type_hack_enum =
            format!("type {} = {};", variant_type_enum_name, variant_types);

        let mut variants = String::new();

        variants.push_str(&format!("\n    'type': {},", variant_type_enum_name));

        for variant in &e.variants {
            let mut variant_type = match &variant.variant_type {
                EnumVariantType::Empty => "null".to_string(),
                EnumVariantType::Tuple(t) => self.gen_tuple(t),
                EnumVariantType::Struct(s) => format!(" {}", self.gen_struct(s, 4)),
                EnumVariantType::Primitive(p) => format!("{}", self.gen_primitive_type(p)),
            };

            variant_type = format!("\n    '{}'?: {},", variant.name, variant_type);

            if let Some(doc) = format_docstring(variant.docs, CommentStyle::DoubleSlash, 4) {
                variant_type = format!("\n{}{}", doc, variant_type);
            }

            variants.push_str(&variant_type);
        }

        format!(
            "{}

type {} = {{|{}\n|}};",
            variant_type_hack_enum, name, variants
        )
    }

    fn gen_tuple(&self, t: &TTuple) -> String {
        let mut values = String::new();

        for (n, item) in t.items.iter().enumerate() {
            let is_last = n == t.items.len() - 1;

            let value = match item {
                TupleItem::Reference(d) => self.gen_name(d),
                TupleItem::TPrimitive(p) => self.gen_primitive_type(p).to_string(),
            };

            values.push_str(&value);
            if !is_last {
                values.push_str(", ");
            }
        }

        format!("[{}]", values)
    }

    fn gen_primitive_type(&self, ty: &TPrimitive) -> &'static str {
        match ty {
            TPrimitive::String => "string",
            TPrimitive::Tbool => "boolean",
            TPrimitive::Ti64 => "number",
            TPrimitive::Tf64 => "number",
            TPrimitive::TGeneric(TGeneric { name, .. }) => name,
        }
    }

    fn gen_name(&self, d: &TypeDeclaration) -> String {
        d.name.to_string()
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
}
