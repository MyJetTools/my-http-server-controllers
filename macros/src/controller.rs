use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};
use crate::args::{MethodMacroArgs, parse_args};

pub fn implement_controller_action(args: MethodMacroArgs, func: TokenStream)  -> TokenStream {

    // get argument params
    let action = args.action.unwrap();
    let route = args.route;
    let controller_name = args.controller_name;

    // get code logic
    let func = parse_macro_input!(func as syn::ItemFn);
    let inputs = func.sig.inputs;

    let output = 
        match func.sig.output {
            syn::ReturnType::Type(_, ty) => ty,
            syn::ReturnType::Default => panic!("Not used as we always must return a value"),
        };

    let block = func.block;

    // generate 
    let expanded = quote! {

        #[async_trait]
        impl #action for #controller_name {
            async fn handle_request(&self, #inputs) -> #output {
                #block
            }
        
            fn get_description(&self) -> Option<HttpActionDescription> {
                None
            }
        
            fn get_route(&self) -> &str {
                #route
            }
        }

    };

    expanded.into()
}

pub fn generate_endpoint(action_name: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let mut args = parse_args(&attr_args);
    args.set_action(action_name);

    implement_controller_action(args, item)
}