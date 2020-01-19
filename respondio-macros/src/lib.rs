extern crate proc_macro;
use proc_macro2::{Ident, Span};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, FnArg, NestedMeta, Pat};

use respondio_core::Route;

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
    let route = Route::new(&path);

    let arg_names: Vec<Ident> = (0..parsed.sig.inputs.len())
        .map(|index| Ident::new(&format!("__arg{}", index), Span::call_site()))
        .collect();

    let arg_expressions: Vec<proc_macro2::TokenStream> = parsed.sig.inputs.iter().enumerate().map(|(index,input)| {
        match input {
            FnArg::Receiver(_) => panic!("Route handler cannot have receiver"),
            FnArg::Typed(pat) => {
                match &*pat.pat {
                    Pat::Ident(ident) => {
                        let name = ident.ident.to_string();
                        let arg_name = arg_names[index].clone();
                        if let Some(path_var_index) = route.index_of_name(&name) {
                            quote! {
                                let #arg_name = if let Ok(__parse_result) = path_vars[#path_var_index].parse() {
                                    __parse_result
                                } else {
                                    return Box::pin(async { Ok(respondio::response::parse_failure(#name)) })
                                };
                            }
                        } else {
                            unimplemented!("Currently only handles route args")
                        }
                    },
                    _ => unimplemented!("Currently only handles route args")
                }
            },
        }
    }).collect();

    let stream = quote! {
        mod #method_name {
            #parsed

            fn __handler(request: respondio::Request, path_vars: Vec<String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<respondio::Response, std::convert::Infallible>> + Send>> {
                #(#arg_expressions)*

                Box::pin(async move {
                    let response = #method_name(#(#arg_names),*).await;
                    Ok(respondio::IntoResponse::into_response(response).await)
                })
            }

            inventory::submit! {
                respondio::RouteHandler::new(
                        stringify!(#method_name).to_string(),
                        respondio::Method::#method_ident,
                        #path.to_string(),
                        __handler)
            }
        }
    };
    stream.into()
}
