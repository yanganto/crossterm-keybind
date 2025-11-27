use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{DeriveInput, Error, Fields, Ident, Meta, Result, Variant};

struct Event {
    name: Ident,
    attrs: Vec<syn::Attribute>,
    // TODO use KeyBindings, and verify
    default_keybindings: String,
}

impl Event {
    pub fn from_variant(
        Variant {
            attrs,
            ident,
            fields,
            ..
        }: Variant,
    ) -> Result<Event> {
        if fields != Fields::Unit {
            return Err(Error::new(
                ident.span(),
                "KeyBind derive only use for enum without fields",
            ));
        }
        let mut new_attrs = Vec::new();
        let mut default_keybindings = "[]".to_string();
        for attr in attrs.into_iter() {
            if attr.path().is_ident("keybindings") {
                let Meta::List(ref meta_list) = attr.meta else {
                    return Err(Error::new(
                        ident.span(),
                        "Keybindings is incorrect, for example correct format is #[keybings[\"Control+c\",\"Q\"]]",
                    ));
                };
                default_keybindings = format!("{}", meta_list.to_token_stream())[11..]
                    .trim()
                    .to_string();
            } else {
                new_attrs.push(attr)
            }
        }
        Ok(Event {
            name: ident,
            attrs: new_attrs,
            default_keybindings,
        })
    }
}

pub(crate) struct Events {
    name: Ident,
    inner: Vec<Event>,
}

impl Events {
    /// Generate the token stream for the patch struct and it resulting implementations
    pub fn into_token_stream(self) -> Result<TokenStream> {
        let Events { name, inner } = self;
        let mut fields = Vec::new();
        let mut lowers = Vec::new();
        let mut uppers = Vec::new();
        let mut attrs = Vec::new();
        let mut defaults = Vec::new();

        for e in inner.into_iter() {
            let name = e.name.to_string();
            fields.push(syn::Ident::new(&name, Span::call_site().into()));
            lowers.push(syn::Ident::new(
                &name.to_lowercase(),
                Span::call_site().into(),
            ));
            uppers.push(syn::Ident::new(
                &name.to_uppercase(),
                Span::call_site().into(),
            ));
            attrs.push(e.attrs);
            let default_stream: proc_macro2::TokenStream = e.default_keybindings.parse().unwrap();
            defaults.push(default_stream);
        }

        Ok(quote! {
            use crossterm_keybind::toml_example;
            use crossterm_keybind::toml_example::TomlExample;
            use crossterm_keybind::struct_patch;
            use crossterm_keybind::struct_patch::Patch;
            use crossterm_keybind::toml;

            #[derive(crossterm_keybind::struct_patch::Patch, crossterm_keybind::toml_example::TomlExample, serde::Deserialize)]
            #[patch(name = "KeyBinding")]
            #[patch(attribute(derive(serde::Deserialize)))]
            struct DefaultBinding {
                #(
                    #( #attrs )*
                    #[toml_example(default=#defaults)]
                    #lowers: crossterm_keybind::KeyBindings,
                )*

            }
            static BINDING_INIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
            #(
                static mut #uppers: core::mem::MaybeUninit<crossterm_keybind::KeyBindings> = core::mem::MaybeUninit::uninit();
            )*

            impl crossterm_keybind::KeyBindTrait for #name {
                fn init_and_load(patch_path: Option<std::path::PathBuf>) -> Result<(), crossterm_keybind::Error>{
                    if BINDING_INIT.load(std::sync::atomic::Ordering::Relaxed) {
                        return Err(crossterm_keybind::Error::ConfigDoubleInitError);
                    } else {
                        BINDING_INIT.store(true, std::sync::atomic::Ordering::Release);
                    }

                    let mut key_config: DefaultBinding =
                        toml::from_str(&DefaultBinding::toml_example()).map_err(|e|crossterm_keybind::Error::DefaultConfigError(e.to_string()))?;
                    if let Some(p) = patch_path {
                        let contents = std::fs::read_to_string(p).map_err(crossterm_keybind::Error::ReadConfigError)?;
                        let patch: KeyBinding =
                            toml::from_str(&contents).map_err(|e|crossterm_keybind::Error::LoadConfigError(e.to_string()))?;
                        key_config.apply(patch);
                    }

                    unsafe {
                        #(
                            #uppers = core::mem::MaybeUninit::new(key_config.#lowers);
                        )*
                    }

                    Ok(())
                }

                fn match_any(&self, key_event: &crossterm_keybind::event::KeyEvent) -> bool {
                    if !BINDING_INIT.load(std::sync::atomic::Ordering::Acquire) {
                        // TODO deubg message here?
                        return false;
                    }
                    use #name as E;
                    match self {
                        #(
                            E::#fields => unsafe { (*(&raw mut #uppers)).assume_init_mut() }.match_any(key_event),
                        )*
                    }
                }

                fn config_example() -> String {
                    DefaultBinding::toml_example()
                }

                fn key_bindings_display(&self) -> String {
                    use #name as E;
                    match self {
                        #(
                            E::#fields => format!("{}", unsafe { (*(&raw mut #uppers)).assume_init_mut() }),
                        )*
                    }
                }
            }

        }.into())
    }
    /// Parse enum to Events
    pub fn from_ast(DeriveInput { ident, data, .. }: DeriveInput) -> Result<Events> {
        let syn::Data::Enum(syn::DataEnum { variants, .. }) = data else {
            return Err(syn::Error::new(
                ident.span(),
                "KeyBind derive only use for enum",
            ));
        };
        let mut inner = Vec::new();

        for v in variants.into_iter() {
            inner.push(Event::from_variant(v)?);
        }

        Ok(Events { name: ident, inner })
    }
}
