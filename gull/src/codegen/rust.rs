use super::docs::{format_docstring, CommentStyle};
use super::shared;
use super::Codegen;
use crate::definitions::*;
use anyhow::Result;
use std::cell::RefCell;
use std::collections::BTreeSet;

pub struct RustCodegen {
    imports: RefCell<BTreeSet<&'static str>>,
}

impl Codegen for RustCodegen {
    fn gen_declarations(declarations: &Declarations) -> Result<String> {
        let rc = RustCodegen::new();

        let mut declarations_code = String::new();

        for declaration in &declarations.declarations {
            declarations_code.push('\n');
            declarations_code.push_str(&rc.gen_declaration(declaration)?);
            declarations_code.push('\n');
        }

        let mut result = String::new();

        for import in rc.imports.borrow().iter() {
            result.push_str(&format!("{}\n", import));
        }

        result.push('\n');
        result.push_str(&declarations_code);

        Ok(result)
    }
}

impl RustCodegen {
    fn new() -> Self {
        Self {
            imports: RefCell::new(BTreeSet::new()),
        }
    }

    fn add_import(&self, import: &'static str) {
        self.imports.borrow_mut().insert(import);
    }

    fn gen_declaration(&self, declaration: &TypeDeclaration) -> Result<String> {
        let mut prefix = String::new();

        for config in &declaration.config {
            match config {
                TypeDeclarationConfig::RustAttribute(attr) => {
                    prefix.push_str(attr);
                    prefix.push('\n')
                }
            }
        }

        let mut r = match &declaration.value {
            DeclarationValue::TPrimitive(p) => format!(
                "pub type {} = {};",
                declaration.name,
                self.gen_primitive_type(p)
            ),
            DeclarationValue::TMap(m) => {
                format!("pub type {} = {};", declaration.name, self.gen_map(m))
            }
            DeclarationValue::TTuple(t) => {
                format!("pub type {} = {};", declaration.name, self.gen_tuple(t))
            }
            DeclarationValue::TStruct(s) => {
                format!(
                    "pub struct {}{} {}",
                    declaration.name,
                    self.gen_generic_param_definitions(&s.generic_params),
                    self.gen_struct(s, 0, true)
                )
            }
            DeclarationValue::TEnum(e) => {
                format!("pub enum {} {}", declaration.name, self.gen_enum(e))
            }
            DeclarationValue::TSimpleEnum(e) => {
                format!("pub enum {} {}", declaration.name, self.gen_simple_enum(e))
            }
            DeclarationValue::Docs => String::new(),
            DeclarationValue::CodeBlock(b) => self.gen_code_block(b),
        };

        let comment_style = if let DeclarationValue::Docs = declaration.value {
            CommentStyle::DoubleSlash
        } else {
            CommentStyle::TripleSlash
        };

        if let Some(doc) = format_docstring(declaration.docs, comment_style, 0) {
            r = format!("{}\n{}", doc, r);
        }

        Ok(format!("{}{}", prefix, r))
    }

    fn gen_map(&self, m: &TMap) -> String {
        let value = match &m.value {
            TMapValue::TPrimitive(p) => self.gen_primitive_type(p),
            TMapValue::TSet(s) => self.gen_set(s),
        };

        let map_type = match m.t {
            TMapType::Hash => {
                self.add_import("use std::collections::HashMap;");
                "HashMap"
            }
            TMapType::BTree => {
                self.add_import("use std::collections::BTreeMap;");
                "BTreeMap"
            }
        };

        format!(
            "{}<{}, {}>",
            map_type,
            self.gen_primitive_type(&m.key),
            value
        )
    }

    fn gen_vec(&self, v: &TVec) -> String {
        let value = match &v {
            TVec::TPrimitive(p) => self.gen_primitive_type(p),
        };
        format!("Vec<{}>", value)
    }

    fn gen_set(&self, s: &TSet) -> String {
        let value = match &s {
            TSet::TPrimitive(p) => self.gen_primitive_type(p),
        };

        self.add_import("use std::collections::BTreeSet;");
        format!("BTreeSet<{}>", value)
    }

    fn gen_option(&self, o: &TOption) -> String {
        let value = match &o {
            TOption::TPrimitive(p) => self.gen_primitive_type(&p),
            TOption::TMap(m) => self.gen_map(m),
            TOption::TVec(v) => self.gen_vec(v),
            TOption::TSet(s) => self.gen_set(s),
            TOption::TTuple(t) => self.gen_tuple(t),
        };
        format!("Option<{}>", value)
    }

    fn gen_struct(&self, s: &TStruct, indent_level: usize, pub_fields: bool) -> String {
        let mut fields = String::new();

        let indent = " ".repeat(indent_level);

        for field in s.fields.iter() {
            let mut field_prefix = String::new();
            let mut value_override = None;

            for config in &field.config {
                match config {
                    StructFieldConfig::RustAttribute(attr) => {
                        field_prefix.push_str(&format!("\n    {}{}", indent, attr))
                    }
                    StructFieldConfig::RustOverride(o) => value_override = Some(o.to_string()),
                }
            }

            let field_type = value_override.unwrap_or_else(|| match &field.field_type {
                StructFieldType::TMap(m) => self.gen_map(m),
                StructFieldType::TSet(s) => self.gen_set(s),
                StructFieldType::TOption(o) => self.gen_option(o),
                StructFieldType::TPrimitive(p) => self.gen_primitive_type(&p),
                StructFieldType::TTuple(t) => self.gen_tuple(t),
                StructFieldType::TVec(v) => self.gen_vec(v),
            });

            let visibility = if pub_fields { "pub " } else { "" };
            let mut field_str = format!(
                "\n    {}{}{}: {},",
                visibility, &indent, field.name, field_type
            );

            if let Some(doc) =
                format_docstring(field.docs, CommentStyle::TripleSlash, indent_level + 4)
            {
                field_str = format!("\n{}{}", doc, field_str);
            }

            fields.push_str(&format!("{}{}", field_prefix, field_str));
        }

        format!("{{{}\n{}}}", fields, indent)
    }

    fn gen_enum(&self, e: &TEnum) -> String {
        let mut variants = String::new();

        for variant in &e.variants {
            let mut variant_type = match &variant.variant_type {
                EnumVariantType::TStruct(s) => format!(" {}", self.gen_struct(s, 4, false)),
                EnumVariantType::TPrimitive(p) => format!("({})", self.gen_primitive_type(p)),
            };

            variant_type = format!("\n    {}{},", variant.name, variant_type);
            if let Some(doc) = format_docstring(variant.docs, CommentStyle::TripleSlash, 4) {
                variant_type = format!("\n{}{}", doc, variant_type);
            }

            variants.push_str(&variant_type);
        }

        format!("{{{}\n}}", variants)
    }

    fn gen_simple_enum(&self, e: &TSimpleEnum) -> String {
        let mut variants = String::new();
        for variant in &e.variants {
            variants.push_str(&format!("\n    {},", variant));
        }

        format!("{{{}\n}}", variants)
    }

    fn gen_tuple(&self, t: &TTuple) -> String {
        let mut values = String::new();

        for (n, item) in t.items.iter().enumerate() {
            let is_last = n == t.items.len() - 1;

            let value = match item {
                TupleItem::TPrimitive(p) => self.gen_primitive_type(p),
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
            TPrimitive::String => "String".to_string(),
            TPrimitive::Tbool => "bool".to_string(),
            TPrimitive::Ti64 => "i64".to_string(),
            TPrimitive::Tf64 => "f64".to_string(),
            TPrimitive::THardcoded(s) => s.to_string(),
            TPrimitive::TDifferentPerLanguege { rust, .. } => self.gen_primitive_type(&rust),
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

    fn gen_generic(&self, g: &TGeneric) -> String {
        match g {
            TGeneric::TDefinition { name, .. } => name.to_string(),
            TGeneric::TReference(r, ..) => {
                self.gen_primitive_type(&TPrimitive::TReference(r.clone()))
            }
        }
    }

    fn gen_generic_param_definitions(&self, params: &[TGeneric]) -> String {
        if params.is_empty() {
            String::new()
        } else {
            let p = params
                .iter()
                .map(|g| {
                    if let TGeneric::TDefinition { name, bounds } = g {
                        format!(
                            "{}{}",
                            name,
                            bounds.map_or(String::new(), |b| format!(": {}", b))
                        )
                    } else {
                        panic!("Generic param definitiens only accept TGeneric::TDefinition!");
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("<{}>", p)
        }
    }

    fn gen_code_block(&self, b: &CodeBlock) -> String {
        match b {
            CodeBlock::Rust(lines) => lines
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}
