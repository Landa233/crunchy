extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(DTypeable)]
pub fn generate_dtype(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut field_statements: Vec<proc_macro2::TokenStream> = vec![];

            field_statements.push(quote! {let mut __fields = vec![];});

            for field in fields {
                let field_name = field.ident.as_ref().unwrap();
                let field_type = &field.ty;
                field_statements.push(quote! {
                    let __field_type = <#field_type as crate::DTypeable>::generate_dtype(&self.#field_name);
                    let __field = npyz::Field{
                        name: stringify!(#field_name).to_string(),
                        dtype: __field_type,
                    };
                    __fields.push(__field);
                });
            }

            let expanded = quote! {
                impl crate::DTypeable for #struct_name{
                   fn generate_dtype(&self) -> npyz::DType {
                        #(#field_statements)*

                        let __struct_dtype = npyz::DType::Record(__fields);
                        __struct_dtype

                    }
                }
            };

            expanded.into()
        }
        _ => panic!("PrintFieldNames can only be derived for structs"),
    }
}
