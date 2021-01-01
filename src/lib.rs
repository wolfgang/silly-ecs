use proc_macro::TokenStream;

use inflector::Inflector;
use quote::{format_ident, quote};
use syn::{Block, Ident, parse_macro_input, ItemFn};
use syn::parse::{Parser, Parse, ParseStream};
use syn::punctuated::Punctuated;

struct ForComponentsInput {
    comp: Ident,
    block: Block
}

impl Parse for ForComponentsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let comp = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let block = input.parse()?;

        Ok(Self {comp, block})
    }
}

struct SystemInput {
    entity_fn: ItemFn
}


#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, syn::Token![,]>::parse_separated_nonempty;
    let attr_idents = parser.parse(attr).unwrap();
    println!("attr_idents {:?}", attr_idents.first());
    let comp = attr_idents.first();

    let orig_tokens = item.clone();

    let orig_fn = parse_macro_input!(orig_tokens as ItemFn);

    let orig_fn_name = orig_fn.sig.ident;
    let wrapper_fn_name = format_ident!("{}_all", orig_fn_name.to_string());

    let code = quote! {
        fn #wrapper_fn_name() {
            #orig_fn_name()
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
    let ForComponentsInput {comp, block} = parse_macro_input!(args);

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
