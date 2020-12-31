
use proc_macro::{TokenStream};
use quote::{quote, format_ident};
use inflector::Inflector;

#[proc_macro]
pub fn impl_entity(item: TokenStream) -> TokenStream {
    let mut comps = Vec::new();
    for comp in item {
        if comp.to_string() != "," {
            comps.push(comp.to_string());
        }
    }

    let mut comp_types = Vec::new();
    let mut comp_names = Vec::new();
    let mut comp_getters = Vec::new();
    let mut comp_mut_getters = Vec::new();
    let mut comp_preds = Vec::new();
    for comp in comps.iter() {
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

    println!("{}", code);

    TokenStream::from(code)
}

