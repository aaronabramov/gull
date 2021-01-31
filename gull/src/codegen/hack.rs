use super::Codegen;
use crate::definitions::*;
use anyhow::Result;

pub struct HackCodegen {}

impl Codegen for HackCodegen {
    fn gen_declarations(declarations: &Vec<TypeDeclaration>) -> Result<String> {
        let rc = HackCodegen::new();

        let mut declarations_code = String::new();

        for declaration in declarations {
            declarations_code.push('\n');
            declarations_code.push_str(&rc.gen_declaration(declaration)?);
            declarations_code.push('\n');
        }

        let mut result = String::new();

        result.push('\n');
        result.push_str(&declarations_code);

        Ok(result)
    }
}

impl HackCodegen {
    fn new() -> Self {
        Self {}
    }

    fn gen_declaration(&self, declaration: &TypeDeclaration) -> Result<String> {
        let r = match &declaration.value {
            DeclarationValue::TPrimitive(p) => format!(
                "type {} = {};",
                declaration.name,
                self.gen_primitive_type(p)
            )
            .into(),
            DeclarationValue::TMap(m) => {
                format!("type {} = {};", declaration.name, self.gen_map(m))
            }
            DeclarationValue::TTuple(t) => {
                format!("type {} = {};", declaration.name, self.gen_tuple(t))
            }
            DeclarationValue::TStruct(s) => {
                format!("type {} = {};", declaration.name, self.gen_struct(s, 0))
            }
            DeclarationValue::TEnum(e) => self.gen_enum(declaration.name, e),
        };

        Ok(r)
    }

    fn gen_map(&self, m: &TMap) -> String {
        let value = match &m.value {
            TMapValue::TPrimitive(p) => self.gen_primitive_type(p),
            TMapValue::Reference(d) => d.name,
        };

        format!("dict<{}, {}>", self.gen_primitive_type(&m.key), value)
    }

    fn gen_vec(&self, v: &TVec) -> String {
        let value = match &v {
            TVec::TPrimitive(p) => self.gen_primitive_type(p),
            TVec::Reference(d) => d.name,
        };
        format!("vec<{}>", value)
    }

    fn gen_set(&self, s: &TSet) -> String {
        let value = match &s {
            TSet::TPrimitive(p) => self.gen_primitive_type(p),
            TSet::Reference(d) => d.name,
        };

        format!("keyset<{}>", value)
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

        let indent = " ".repeat(indent);

        for field in &s.fields {
            let field_type = match &field.field_type {
                StructFieldType::Reference(r) => r.name.into(),
                StructFieldType::TMap(m) => self.gen_map(m),
                StructFieldType::TOption(o) => self.gen_option(o),
                StructFieldType::TPrimitive(p) => self.gen_primitive_type(&p).into(),
                StructFieldType::TTuple(t) => self.gen_tuple(t),
                StructFieldType::TVec(v) => self.gen_vec(v),
            };

            fields.push_str(&format!(
                "\n    {}'{}' => {},",
                &indent, field.name, field_type
            ));
        }

        format!("shape({}\n{})", fields, indent)
    }

    fn gen_enum(&self, name: &str, e: &TEnum) -> String {
        let mut variant_types = vec![];

        for variant in &e.variants {
            variant_types.push(format!(
                r#"    {} = "{}",{}"#,
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
            let variant_type = match &variant.variant_type {
                EnumVariantType::Empty => None,
                EnumVariantType::Tuple(t) => Some(self.gen_tuple(t)),
                EnumVariantType::Struct(s) => Some(format!(" {}", self.gen_struct(s, 4))),
            };

            if let Some(variant_type) = variant_type {
                variants.push_str(&format!("\n    ?'{}' => ?{},", variant.name, variant_type));
            }
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
                TupleItem::Reference(d) => d.name,
                TupleItem::TPrimitive(p) => self.gen_primitive_type(p),
            };

            values.push_str(value);
            if !is_last {
                values.push_str(", ");
            }
        }

        format!("tuple({})", values)
    }

    fn gen_primitive_type(&self, ty: &TPrimitive) -> &'static str {
        match ty {
            TPrimitive::String => "string",
            TPrimitive::Tbool => "bool",
            TPrimitive::Ti64 => "int",
            TPrimitive::Tf64 => "float",
        }
    }
}
