use crate::prelude::*;

// given a slice of generic params produce <A, B, C> or empty string
pub fn generic_params<F>(params: &[TGeneric], gen_generic: F) -> String
where
    F: Fn(&TGeneric) -> String,
{
    if params.is_empty() {
        String::new()
    } else {
        let params = params
            .iter()
            .map(gen_generic)
            .collect::<Vec<_>>()
            .join(", ");
        format!("<{}>", params)
    }
}
