mod key_bind;

#[proc_macro_derive(KeyBind, attributes(keybindings))]
pub fn derive_patch(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    key_bind::Events::from_ast(syn::parse_macro_input!(item as syn::DeriveInput))
        .unwrap()
        .into_token_stream()
        .unwrap()
}
