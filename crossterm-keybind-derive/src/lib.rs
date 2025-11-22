#[proc_macro_derive(KeyBind, attributes(patch))]
pub fn derive_patch(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
