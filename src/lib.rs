use proc_macro::TokenStream;

use inflector::Inflector;
use quote::{format_ident, quote};
use syn::{ExprReference, Ident, ItemFn, parse_macro_input};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

struct ComponentAttribute {
    ident: Ident,
    is_mutable: bool,
}

struct SystemAttributes {
    attributes: Vec<ComponentAttribute>
}

impl Parse for SystemAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = Vec::with_capacity(10);

        loop {
            let mut is_mutable = false;
            if input.peek(syn::Token![mut]) {
                let _ = input.parse::<syn::Token![mut]>();
                is_mutable = true;
            }

            let ident: Ident = input.parse()?;
            attributes.push(ComponentAttribute { ident, is_mutable });

            if input.peek(syn::Token![,]) {
                let _ = input.parse::<syn::Token![,]>();
            } else {
                break;
            }
        }

        Ok(Self { attributes })
    }
}

struct ImplEntityInput {
    components: Punctuated<Ident, syn::Token![,]>
}

impl Parse for ImplEntityInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = input.parse_terminated(Ident::parse)?;
        Ok(Self { components })
    }
}


#[proc_macro]
pub fn secs_impl_entity(args: TokenStream) -> TokenStream {
    let ImplEntityInput { components } = parse_macro_input!(args as ImplEntityInput);

    let mut comp_types = Vec::with_capacity(components.len());
    let mut comp_names = Vec::with_capacity(components.len());
    let mut comp_getters = Vec::with_capacity(components.len());
    let mut comp_mut_getters = Vec::with_capacity(components.len());
    let mut comp_preds = Vec::with_capacity(components.len());

    for ident in components.iter() {
        let comp = ident.to_string();
        let comp_name = comp.to_snake_case();
        comp_types.push(format_ident!("{}", comp));
        comp_names.push(format_ident!("{}", comp_name));
        comp_getters.push(format_ident!("{}", comp_name));
        comp_mut_getters.push(format_ident!("mut_{}", comp_name));
        comp_preds.push(format_ident!("has_{}", comp_name))
    };

    let code = quote! {
        #[derive(Default, Debug)]
        struct Entity {
            #(#comp_names: Option<#comp_types>),*
        }

        impl Entity {
            #(pub fn #comp_getters(&self) -> &#comp_types { self.#comp_names.as_ref().unwrap() })*
            #(pub fn #comp_mut_getters(&mut self) -> &mut #comp_types { self.#comp_names.as_mut().unwrap() })*
            #(pub fn #comp_preds(&self) -> bool { self.#comp_names.is_some() })*
        }
    };

    TokenStream::from(code)
}

#[proc_macro_attribute]
pub fn secs_system(attr: TokenStream, orig_fn_tokens: TokenStream) -> TokenStream {
    let SystemAttributes { attributes } = parse_macro_input!(attr as SystemAttributes);

    let item_copy = orig_fn_tokens.clone();
    let orig_fn = parse_macro_input!(item_copy as ItemFn);
    let orig_fn_name = orig_fn.sig.ident;
    let wrapper_fn_name = format_ident!("sys_{}", orig_fn_name.to_string());

    let preds: Vec<Ident> = attributes
        .iter()
        .map(|attr| { format_ident!("has_{}", attr.ident.to_string().to_snake_case()) })
        .collect();

    let any_mutable = attributes.iter().any(|attr| { attr.is_mutable });

    let mut_prefix = if any_mutable { "mut " } else { "" };
    let iter_type = if any_mutable { "iter_mut" } else { "iter" };
    let tokens: TokenStream = format!("&{}Entities", mut_prefix).parse().unwrap();
    let entities_ref: ExprReference = syn::parse(tokens).unwrap();
    let iter = format_ident!("{}", iter_type);

    let code = quote! {
        fn #wrapper_fn_name(entities: #entities_ref) {
            for entity in entities.#iter().filter(|entity| { #(entity.#preds())&&* }) {
                #orig_fn_name(entity)
            }
        }
    };

    let mut result_tokens = TokenStream::new();
    result_tokens.extend(orig_fn_tokens);
    result_tokens.extend(TokenStream::from(code));
    result_tokens
}

#[proc_macro_attribute]
pub fn log_call(_: TokenStream, orig_fn_tokens: TokenStream) -> TokenStream {
    let orig_fn_tokens_copy = orig_fn_tokens.clone();
    let orig_fn = parse_macro_input!(orig_fn_tokens_copy as ItemFn);
    let orig_vis = orig_fn.vis;
    let orig_block = orig_fn.block;
    let orig_sig = orig_fn.sig;
    let orig_name = orig_sig.ident.clone().to_string();

    let code = quote! {
        #orig_vis #orig_sig {
            println!("LOGGED CALL: {}", #orig_name);
            #orig_block
        }

    };

    println!("log_me code:{}", code);

    TokenStream::from(code)
}

#[proc_macro_attribute]
pub fn system2(attr: TokenStream, orig_fn_tokens: TokenStream) -> TokenStream {
    let SystemAttributes { attributes } = parse_macro_input!(attr as SystemAttributes);

    let item_copy = orig_fn_tokens.clone();
    let orig_fn = parse_macro_input!(item_copy as ItemFn);
    let orig_fn_name = orig_fn.sig.ident;
    let wrapper_fn_name = format_ident!("sys_{}", orig_fn_name.to_string());

    let preds: Vec<Ident> = attributes
        .iter()
        .map(|attr| { format_ident!("has_{}", attr.ident.to_string().to_snake_case()) })
        .collect();

    let getters: Vec<Ident> = attributes
        .iter()
        .map(|attr| {
            let mut_prefix = if attr.is_mutable { "mut_" } else { "" };
            format_ident!("{}{}", mut_prefix, attr.ident.to_string().to_snake_case())
        })
        .collect();

    let any_mutable = attributes.iter().any(|attr| { attr.is_mutable });

    let mut_prefix = if any_mutable { "mut " } else { "" };
    let iter_type = if any_mutable { "iter_mut" } else { "iter" };
    let tokens: TokenStream = format!("&{}Entities", mut_prefix).parse().unwrap();
    let entities_ref: ExprReference = syn::parse(tokens).unwrap();
    let iter = format_ident!("{}", iter_type);

    let code = quote! {
        fn #wrapper_fn_name(entities: #entities_ref) {
            for entity in entities.#iter().filter(|entity| { #(entity.#preds())&&* }) {
                #orig_fn_name(#(entity.#getters()),*)
            }
        }
    };

    let mut result_tokens = TokenStream::new();
    result_tokens.extend(orig_fn_tokens);
    result_tokens.extend(TokenStream::from(code));
    result_tokens
}
