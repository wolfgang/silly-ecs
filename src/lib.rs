
use proc_macro::{TokenStream};
use quote::{quote, format_ident};
use inflector::Inflector;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro]
pub fn impl_entity(item: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(item as syn::);
    // println!("{:?}", input);
    // "fn answer2() -> u32 { 43 }".parse().unwrap()


    let mut comps = Vec::new();
    for comp in item {
        if comp.to_string() != "," {
            comps.push(comp.to_string());
        }
    }
    println!("{:?}", comps);

    let member_decls: String = comps.iter()
        .map(|c| {
            let name = c.to_ascii_lowercase();
            format!("{}:Option<{}>", name, c)
        })
        .collect::<Vec<String>>()
        .join(",");

    let getters: String = comps.iter()
        .map(|c| {
            let name = c.to_ascii_lowercase();
            format!("fn get_{}(&self) -> &{} {{ self.{}.as_ref().unwrap() }} ", name, c, name)
        })
        .collect::<Vec<String>>()
        .join("\n");


    println!("{:?}", member_decls);


    let decl = format!("struct Entity3 {{ {} }}", member_decls);
    let imp = format!("impl Entity3 {{ {} }}", getters);

    let code = format!("{}\n{}", decl, imp);

    println!("{}", code);

    code.parse().unwrap()
}

#[proc_macro]
pub fn impl_entity2(item: TokenStream) -> TokenStream {
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
        struct Entity4 {
            #(#comp_names: Option<#comp_types>),*
        }

        impl Entity4 {
            #(pub fn #comp_getters(&self) -> &#comp_types { self.#comp_names.as_ref().unwrap() })*
            #(pub fn #comp_mut_getters(&mut self) -> &mut #comp_types { self.#comp_names.as_mut().unwrap() })*
            #(pub fn #comp_preds(&self) -> bool { self.#comp_names.is_some() })*
        }
    };

    println!("{}", code);

    TokenStream::from(code)
}

