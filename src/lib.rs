
use proc_macro::TokenStream;
use proc_macro2;

use inflector::Inflector;
use quote::{format_ident, quote};
use syn::{ExprReference, FnArg, Ident, ItemFn, parse_macro_input, WhereClause, Signature};
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
    let mut comp_mut_names = Vec::with_capacity(components.len());
    let mut comp_preds = Vec::with_capacity(components.len());

    for ident in components.iter() {
        let comp = ident.to_string();
        let comp_name = comp.to_snake_case();
        comp_types.push(format_ident!("{}", comp));
        comp_names.push(format_ident!("{}", comp_name));
        comp_mut_names.push(format_ident!("mut_{}", comp_name));
        comp_preds.push(format_ident!("has_{}", comp_name))
    };

    let code = quote! {
        #[derive(Default, Debug)]
        struct Entity {
            #(#comp_names: Option<#comp_types>),*
        }

        impl Entity {
            #(pub fn #comp_names(&self) -> &#comp_types { self.#comp_names.as_ref().unwrap() })*
            #(pub fn #comp_mut_names(&mut self) -> &mut #comp_types { self.#comp_names.as_mut().unwrap() })*
            #(pub fn #comp_preds(&self) -> bool { self.#comp_names.is_some() })*
        }
    };

    TokenStream::from(code)
}


#[proc_macro_attribute]
pub fn secs_system(attr: TokenStream, fn_tokens: TokenStream) -> TokenStream {
    let fn_tokens_clone = fn_tokens.clone();
    let orig_fn = parse_macro_input!(fn_tokens_clone as ItemFn);
    let orig_sig = orig_fn.sig.clone();
    let SystemAttributes { attributes } = parse_macro_input!(attr as SystemAttributes);
    let is_mutable = attributes.iter().any(|attr| { attr.is_mutable });

    let (extra_inputs, extra_arg_names) = gen_extra_inputs(&orig_sig);
    let component_preds = gen_component_predicates(&attributes);
    let entities_ref = gen_entities_ref(is_mutable);
    let iter_call = gen_iter_call(is_mutable);
    let (gen_prefix, gen_where_clause) = gen_generics(orig_fn);
    let (orig_fn_name, wrapper_fn_name) = gen_fn_names(orig_sig);

    let code = quote! {
        fn #wrapper_fn_name#gen_prefix(entities: #entities_ref, #extra_inputs) #gen_where_clause {
            for entity in entities.#iter_call().filter(|entity| { #(entity.#component_preds())&&* }) {
                #orig_fn_name(entity, #extra_arg_names)
            }
        }
    };

    let mut result_tokens = TokenStream::new();
    result_tokens.extend(fn_tokens);
    result_tokens.extend(TokenStream::from(code));
    result_tokens
}

fn gen_component_predicates(attributes: &Vec<ComponentAttribute>) -> Vec<Ident> {
    let preds: Vec<Ident> = attributes
        .iter()
        .map(|attr| { format_ident!("has_{}", attr.ident.to_string().to_snake_case()) })
        .collect();
    preds
}

fn gen_extra_inputs(orig_sig: &Signature) -> (Punctuated<FnArg, syn::Token![,]>, Punctuated<Ident, syn::Token![,]>) {
    let orig_inputs = orig_sig.inputs.clone();
    let mut extra_inputs: Punctuated<FnArg, syn::Token![,]> = Punctuated::new();
    let mut extra_arg_names: Punctuated<Ident, syn::Token![,]> = Punctuated::new();
    for i in 1..orig_inputs.len() {
        let arg = orig_inputs[i].clone();
        for token in quote! {#arg} {
            extra_arg_names.push(format_ident!("{}", token.to_string()));
            break;
        }
        extra_inputs.push(arg);
    }
    (extra_inputs, extra_arg_names)
}

fn gen_entities_ref(is_mut: bool) -> ExprReference {
    let mut_prefix = if is_mut { "mut " } else { "" };
    let tokens: TokenStream = format!("&{}Entities", mut_prefix).parse().unwrap();
    syn::parse(tokens).unwrap()
}

fn gen_iter_call(is_mut: bool) -> Ident {
    let iter_type = if is_mut { "iter_mut" } else { "iter" };
    format_ident!("{}", iter_type)
}

fn gen_fn_names(orig_sig: Signature) -> (Ident, Ident) {
    let orig_fn_name = orig_sig.ident;
    let wrapper_fn_name = format_ident!("sys_{}", orig_fn_name.to_string());
    (orig_fn_name, wrapper_fn_name)
}

fn gen_generics(orig_fn: ItemFn) -> (proc_macro2::TokenStream, Option<WhereClause>) {
    let orig_generics = orig_fn.sig.generics;
    let gen_lt_token = orig_generics.lt_token;
    let gen_params = orig_generics.params;
    let gen_rt_token = orig_generics.gt_token;

    (quote! { #gen_lt_token #gen_params #gen_rt_token }, orig_generics.where_clause)
}