extern crate proc_macro;

#[macro_use] extern crate quote;
extern crate darling;
extern crate syn;

mod args;
mod controller;
mod routes;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, ItemMod};


#[proc_macro_derive(HttpActionDescriptionProvider)]
pub fn get_description(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;
    let expanded = quote! {
 
        impl HttpActionDescriptionProvider for #name {
            fn get_description(&self) -> Option<HttpActionDescription> {
                self.action.get_description()
            }
        }
 
    };

    expanded.into()
}

#[proc_macro]
pub fn create_route_action(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);
    routes::generate(input)
}

#[proc_macro_attribute]
pub fn controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let controller = parse_macro_input!(item as ItemMod);
    
    let controller_name = controller.ident;
    let (_, functions )= controller.content.unwrap();

    let expanded = quote! {

        use async_trait::async_trait;
        use my_http_server_controllers::controllers::{actions::{PostAction, DeleteAction, GetAction, PutAction}, documentation::HttpActionDescription};        
        
        pub struct #controller_name;
        impl #controller_name { 
            pub fn new() -> Self {Self{}}
        }
        
        #(#functions)*
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::generate_endpoint("GetAction", attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::generate_endpoint("PostAction", attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::generate_endpoint("PutAction", attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    controller::generate_endpoint("DeleteAction", attr, item)
}
