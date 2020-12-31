
use proc_macro::{TokenStream};
use quote::{quote, format_ident};
use inflector::Inflector;
use syn::punctuated::Punctuated;
use syn::parse::Parser;
use syn::{Token, Ident};

#[proc_macro]
pub fn impl_entity(args: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_separated_nonempty;
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
        #[derive(Default)]
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

#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_separated_nonempty;
    let attr_idents = parser.parse(attr).unwrap();
    println!("attr_idents {:?}", attr_idents.first());
    item

}
