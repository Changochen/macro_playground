//use proc_macro2::TokenStream;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parser, parse_macro_input, Data, DeriveInput, Fields, ImplItem, ItemImpl};

#[proc_macro]
pub fn my_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
#[proc_macro_attribute]
pub fn add_extra_field(_attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("input: {:#?}", input);
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    println!("input here: {:#?}", input);
    // Check if the input is a struct
    if let Data::Struct(ref data) = input.data {
        // Check if the struct has a field named "extra"
        let has_extra_field = data.fields.iter().any(|field| {
            if let Some(ident) = &field.ident {
                ident.to_string() == "extra"
            } else {
                false
            }
        });

        // If the struct doesn't have an "extra" field, add it
        if !has_extra_field {
            println!("No extra field found, adding one...");
            let extra_field = quote! {
                extra: i32
            };

            // Generate the updated struct definition
            let updated_struct = match &data.fields {
                Fields::Named(fields) => {
                    let mut named_fields = fields.named.clone();
                    named_fields.push(syn::Field::parse_named.parse2(extra_field).unwrap());
                    let input_ident = &input.ident;
                    let res = quote! {
                        struct #input_ident {
                            #named_fields
                        }
                    };
                    println!("Input.ident: {:#?}", input.ident);
                    println!("res: {}", res);
                    res
                }
                Fields::Unnamed(fields) => {
                    let mut unnamed_fields = fields.unnamed.clone();
                    unnamed_fields.push(syn::Field::parse_named.parse2(extra_field).unwrap());
                    quote! {
                        struct #input.ident (
                            #unnamed_fields
                        );
                    }
                }
                Fields::Unit => {
                    quote! {
                        struct #input.ident {
                            extra: i32
                        }
                    }
                }
            };

            // Return the updated struct definition as TokenStream
            return updated_struct.into();
        }
    }

    // If the struct already has an "extra" field or is not a struct, return the original input tokens
    input.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn mockable_derive(_attr: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(item_impl) = syn::parse::<ItemImpl>(input.clone()) {
        println!("item_impl: {:#?}", item_impl);
        // Parse the input tokens into a syntax tree
        // Create a vector to hold the new mock functions
        let mut mock_functions = Vec::new();

        // Iterate over the items in the impl block
        for item in item_impl.items {
            println!("item: {:#?}", item);
            if let ImplItem::Fn(method) = item {
                // Get the method name
                let method_name = &method.sig.ident;
                // Create the mock method name
                let mock_method_name =
                    syn::Ident::new(&format!("{}_mock", method_name), method_name.span());

                // Generate the mock method with the exact signature, but with a different body.
                // The body of the mock method just calls the original method with the same arguments.
                let original_signature = &method.sig;
                // Replace the name of the original method with the mock method name
                let mock_signature = syn::Signature {
                    ident: mock_method_name.clone(),
                    ..original_signature.clone()
                };

                let original_args = original_signature.inputs.iter().map(|arg| {
                    // get the argument name
                    let arg_name = match arg {
                        syn::FnArg::Receiver(_) => quote! { self },
                        syn::FnArg::Typed(pat_type) => {
                            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                                let ident = pat_ident.ident.clone();
                                quote! { #ident }
                            } else {
                                panic!("Expected an identifier for the argument");
                            }
                        }
                    };
                    arg_name
                });
                // Generate the mock method
                let mock_method = quote! {
                    #mock_signature {
                        Self::#method_name(#(#original_args),*)
                    }
                };

                /*
                // Generate the mock method
                let mock_method = quote! {
                    fn #mock_method_name(&self) {
                        // Mock implementation
                        println!("{} called", stringify!(#mock_method_name));
                    }
                };
                */

                // Add the mock method to the vector
                mock_functions.push(mock_method);
            }
        }

        // Generate the final token stream
        let input_tokens: proc_macro2::TokenStream = input.into();
        let struct_name = &item_impl.self_ty;
        let expanded = quote! {
                #input_tokens
                impl #struct_name {
                    #(#mock_functions)*
                }
        };

        // Return the generated impl block as a TokenStream
        TokenStream::from(expanded)
    } else {
        input
    }
}
