mod declarations;

pub use declarations::Declarations;

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    String,
    Ti32,
    Tbool,
}

#[derive(Debug, Clone)]
pub enum TypeDeclaration {
    PrimitiveType {
        name: &'static str,
        value: PrimitiveType,
    },
    Struct {
        name: &'static str,
        fields: Vec<StructField>,
    },
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: &'static str,
    pub field_type: StructFieldType,
}

#[derive(Debug, Clone)]
pub enum StructFieldType {
    PrimitiveType(PrimitiveType),
    Reference(TypeDeclaration),
}
