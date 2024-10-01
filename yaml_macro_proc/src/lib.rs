//use proc_macro::TokenStream;
//use quote::quote;
//use serde_yaml::Value;
//use std::fs;
//use syn::{parse_macro_input, DeriveInput};
//
//#[proc_macro]
//pub fn yaml_struct(input: TokenStream) -> TokenStream {
//    let file_path = parse_macro_input!(input as syn::LitStr).value();
//
//    let yaml_content = fs::read_to_string(file_path).expect("Unable to read the YAML file");
//
//    let yaml_data: serde_yaml::Value =
//        serde_yaml::from_str(&yaml_content).expect("Unable to parse YAML");
//
//    let struct_name = "Generated_struct";
//    let mut fields = Vec::new();
//
//    match yaml_data {
//        Value::Mapping(mapping) => {
//            for (k, v) in mapping {
//                let key = k.as_str().unwrap();
//                let value_type = match v {
//                    Value::String(_) => quote! { String },
//                    Value::Number(_) => quote! { i32 },
//                    Value::Bool(_) => quote! { bool },
//                    _ => quote! { serde_yaml::Value }, // Pour les types non pris en charge
//                };
//
//                fields.push(quote! {
//                    pub #key: #value_type,
//                });
//            }
//        }
//        _ => panic!("YAML file must contain a mapping"),
//    };
//
//    let expanded = quote! {
//        #[derive(Debug, serde::Deserialize)]
//        pub struct #struct_name {
//            #(#fields),*
//        }
//    };
//
//    TokenStream::from(expanded)
//}

//use proc_macro::{Ident, TokenStream};
//use quote::quote;
//use serde_yaml::Value;
//use std::fs;
//use syn::{parse_macro_input, LitStr};
//
//#[proc_macro]
//pub fn yaml_struct(input: TokenStream) -> TokenStream {
//    let file_path = parse_macro_input!(input as LitStr).value();
//
//    // Lire le fichier YAML
//    let yaml_content = fs::read_to_string(file_path).expect("Unable to read the YAML file");
//
//    // Parser le YAML
//    let yaml_data: Value = serde_yaml::from_str(&yaml_content).expect("Unable to parse YAML");
//
//    // Définir le nom de la structure
//    let struct_name = Ident::new("GeneratedStruct", proc_macro2::Span::call_site());
//    let mut fields = Vec::new();
//
//    // Générer les champs de la structure à partir du YAML
//    match yaml_data {
//        Value::Mapping(mapping) => {
//            for (k, v) in mapping {
//                let key = k.as_str().unwrap();
//                let value_type = match v {
//                    Value::String(_) => quote! { String },
//                    Value::Number(_) => quote! { i32 },
//                    Value::Bool(_) => quote! { bool },
//                    _ => quote! { serde_yaml::Value }, // Pour les types non pris en charge
//                };
//
//                fields.push(quote! {
//                    pub #key: #value_type,
//                });
//            }
//        }
//        _ => panic!("YAML file must contain a mapping"),
//    };
//
//    // Générer le code pour la structure
//    let expanded = quote! {
//        #[derive(Debug, serde::Deserialize)]
//        pub struct #struct_name {
//            #(#fields)*
//        }
//    };
//
//    TokenStream::from(expanded)
//}

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use serde_yaml::Value;
use std::fs;
use syn::{parse_macro_input, LitStr}; // Ajoutez cet import

#[proc_macro]
pub fn yaml_struct(input: TokenStream) -> TokenStream {
    let file_path = parse_macro_input!(input as LitStr).value();

    // Lire le fichier YAML
    let yaml_content = fs::read_to_string(file_path).expect("Unable to read the YAML file");

    // Parser le YAML
    let yaml_data: Value = serde_yaml::from_str(&yaml_content).expect("Unable to parse YAML");

    // Définir le nom de la structure
    let struct_name = Ident::new("GeneratedStruct", proc_macro2::Span::call_site()); // Utilisez proc_macro2::Span ici
    let mut fields = Vec::new();

    // Générer les champs de la structure à partir du YAML
    match yaml_data {
        Value::Mapping(mapping) => {
            for (k, v) in mapping {
                let key = format_ident!("{}", k.as_str().unwrap());
                let value_type = match v {
                    Value::String(_) => quote! { String },
                    Value::Number(_) => quote! { i32 },
                    Value::Bool(_) => quote! { bool },
                    _ => quote! { serde_yaml::Value }, // Pour les types non pris en charge
                };

                fields.push(quote! {
                    pub #key: #value_type,
                });
            }
        }
        _ => panic!("YAML file must contain a mapping"),
    };

    // Générer le code pour la structure
    let expanded = quote! {
        #[derive(Debug, serde::Deserialize)]
        pub struct #struct_name {
            #(#fields)*
        }
    };

    TokenStream::from(expanded)
}
