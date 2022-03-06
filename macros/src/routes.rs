use proc_macro::TokenStream;
use syn::Ident;

pub fn generate(input: Ident) -> TokenStream {

    let name = Ident::new(&format!("{}RouteAction", input), input.span());
    let action = Ident::new(&format!("{}Action", input), input.span());
    let route = Ident::new(&format!("{}Route", input), input.span());

    let expanded = quote! {

        use std::{collections::HashMap, sync::Arc};

        use my_http_server::HttpOkResult;
        use my_http_server::{http_path::PathSegments, HttpContext, HttpFailResult};

        use crate::controllers::{
            actions::#action,
            documentation::{HttpActionDescription, HttpActionDescriptionProvider},
        };

        #[derive(HttpActionDescriptionProvider)]
        pub struct #name {
            pub route: PathSegments,
            pub action: Arc<dyn #action + Send + Sync + 'static>,
        }


        pub struct #route {
            pub no_keys: HashMap<String, #name>,
            pub with_keys: Vec<#name>,
        }

        impl #route {
            pub fn new() -> Self {
                Self {
                    no_keys: HashMap::new(),
                    with_keys: Vec::new(),
                }
            }

            pub fn register(&mut self, action: Arc<dyn #action + Send + Sync + 'static>) {
                let route = action.get_route();
                let route = PathSegments::new(route);

                let action = #name { route, action };

                if action.route.keys_amount == 0 {
                    self.no_keys
                        .insert(action.route.path.to_lowercase(), action);
                } else {
                    self.with_keys.push(action);
                }
            }

            pub async fn handle_request(
                &self,
                ctx: &mut HttpContext,
            ) -> Result<Option<HttpOkResult>, HttpFailResult> {
                let path = ctx.request.get_path_lower_case();
                if let Some(route_action) = self.no_keys.get(path) {
                    let result = route_action.action.handle_request(ctx).await?;
                    return Ok(Some(result));
                }

                for route_action in &self.with_keys {
                    if route_action
                        .route
                        .is_my_path(ctx.request.get_path_lower_case())
                    {
                        ctx.request.route = Some(route_action.route.clone());
                        let result = route_action.action.handle_request(ctx).await?;
                        return Ok(Some(result));
                    }
                }

                Ok(None)
            }
        }
    };

    expanded.into()
}