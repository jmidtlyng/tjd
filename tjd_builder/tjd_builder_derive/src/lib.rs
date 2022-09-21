// extern crate proc_macro;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use proc_macro::TokenStream;

/*
#[proc_macro_derive(TjdBuilder)]
pub fn tjd_builder(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    
    let name = &ast.ident;
    // println!("{:#?}", &ast.body);
    let expanded = quote! {
        impl TjdBuilder for #name {
            fn tjd_builder(self){
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    
    println!("in macro");
    
    // return generated impl
    TokenStream::from(expanded)
}
*/

#[proc_macro]
pub fn tjd_builder(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    
    let name = &ast.ident;
    // println!("{:#?}", &ast.body);
    let expanded = quote! {
        impl TjdBuilder for #name {
            fn tjd_builder(self){
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    
    println!("in macro");
    
    // return generated impl
    TokenStream::from(expanded)
}