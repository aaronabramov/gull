use crate::prelude::*;

// given a slice of generic params produce <A, B, C> or empty string
pub fn generic_params(params: &[TGeneric]) -> String {
    if params.is_empty() {
        String::new()
    } else {
        let params = params
            .iter()
            .map(|g| g.name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("<{}>", params)
    }
}
