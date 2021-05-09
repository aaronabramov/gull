#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpInline {
    pub graphs: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpFetch {
    pub timeline_key: i64,
    pub graph_ids: Vec<i64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "variant")]
pub enum OperationsEnum {
    OpInline(OpInline),
    OpFetch(OpFetch),
}
