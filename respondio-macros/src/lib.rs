extern crate proc_macro;
use proc_macro2::{Ident, Span};

use proc_macro::{TokenStream};
use syn::{parse, parse_macro_input, NestedMeta};
use quote::{quote};

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_route(attr, item, "GET")
}

fn generate_route(attr: TokenStream, item: TokenStream, method: &str) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let path = if let Some(NestedMeta::Lit(syn::Lit::Str(path))) = args.get(0) {
        path.value()
    } else {
        panic!("Expecting single string literal in route arguments");
    };
    let parsed = parse::<syn::ItemFn>(item).expect("Methods should be functions");
    let method_ident = Ident::new(method, Span::call_site());
    let method_name = parsed.sig.ident.clone();

    let stream = quote! {
        mod #method_name {
            #parsed

            fn __handler(request: respondio::Request, path_vars: Vec<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<respondio::Response, std::convert::Infallible>> + Send>> {
                let args = ();
                Box::pin(async {
                    let response = #method_name().await;
                    Ok(respondio::IntoResponse::into_response(response).await)
                })
            }

            inventory::submit! {
                respondio::RouteHandler::new(
                        "test_name_asdf".to_string(),
                        respondio::Method::#method_ident,
                        #path.to_string(),
                        __handler)
            }
        }
    };
    stream.into()
}

