use proc_macro::TokenStream;

use inflector::Inflector;
use quote::{format_ident, quote};
use syn::{Block, ExprReference, Ident, ItemFn, parse_macro_input};
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;

struct ForComponentsInput {
    comp: Ident,
    block: Block,
}

impl Parse for ForComponentsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let comp = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let block = input.parse()?;

        Ok(Self { comp, block })
    }
}

struct SystemAttributes {
    components: Vec<Ident>,
    is_mutable: bool,
}

impl Parse for SystemAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut components = Vec::with_capacity(10);
        let mut is_mutable = false;
        loop {
            if input.peek(syn::Token![mut]) {
                let _ = input.parse::<syn::Token![mut]>();
                is_mutable = true;
            }

            let ident: Ident = input.parse()?;

            components.push(ident);
            if input.peek(syn::Token![,]) {
                let _ = input.parse::<syn::Token![,]>();
            } else {
                break;
            }
        }

        Ok(Self { components, is_mutable })
    }
}


#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let SystemAttributes { components, is_mutable } = parse_macro_input!(attr as SystemAttributes);

    let item_copy = item.clone();
    let orig_fn = parse_macro_input!(item_copy as ItemFn);
    let orig_fn_name = orig_fn.sig.ident;
    let wrapper_fn_name = format_ident!("sys_{}", orig_fn_name.to_string());

    let preds: Vec<Ident> = components
        .iter()
        .map(|ident| { format_ident!("has_{}", ident.to_string().to_snake_case()) })
        .collect();

    let mut_prefix = if is_mutable { "mut " } else {""};
    let iter_type = if is_mutable { "iter_mut" } else {"iter"};
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
    result_tokens.extend(item);
    result_tokens.extend(TokenStream::from(code));
    result_tokens
}

#[proc_macro]
pub fn for_components(args: TokenStream) -> TokenStream {
    // let parser = Punctuated::<Block, Token![;]>::parse_separated_nonempty;
    let ForComponentsInput { comp, block } = parse_macro_input!(args);

    let pred = format_ident!("has_{}", comp.to_string().to_snake_case());

    let code = quote! {
        let tmp_fun = |entity| #block;

        for entity in entities
            .iter()
            .filter(|entity| { entity.#pred() }) {
            tmp_fun(entity)
        }
    };
    TokenStream::from(code)
}

#[proc_macro]
pub fn impl_entity(args: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, syn::Token![,]>::parse_separated_nonempty;
    let comp_idents = parser.parse(args).unwrap();

    let mut comp_types = Vec::with_capacity(comp_idents.len());
    let mut comp_names = Vec::with_capacity(comp_idents.len());
    let mut comp_getters = Vec::with_capacity(comp_idents.len());
    let mut comp_mut_getters = Vec::with_capacity(comp_idents.len());
    let mut comp_preds = Vec::with_capacity(comp_idents.len());

    for ident in comp_idents.iter() {
        let comp = ident.to_string();
        let comp_name = comp.to_snake_case();
        comp_types.push(format_ident!("{}", comp));
        comp_names.push(format_ident!("{}", comp_name));
        comp_getters.push(format_ident!("get_{}", comp_name));
        comp_mut_getters.push(format_ident!("get_mut_{}", comp_name));
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

    // println!("{}", code);

    TokenStream::from(code)
}
