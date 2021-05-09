#[proc_macro_derive(EnumSerialization)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    // let output: proc_macro2::TokenStream = { /* transform input */ };

    proc_macro::TokenStream::from(input)
}
