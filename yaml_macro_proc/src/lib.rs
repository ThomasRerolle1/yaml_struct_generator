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

    let (fields, nested_structs) = generate_struct("GeneratedStruct", &yaml_data);
    // Générer les champs de la structure à partir du YAML
    // Générer le code pour la structure
    let main_struct = quote! {
        #[derive(Debug, serde::Deserialize)]
        pub struct #struct_name {
            #(#fields)*
        }

    };

    let expanded = quote! {
        #main_struct
        #(#nested_structs)*
    };

    TokenStream::from(expanded)
}

fn generate_struct(
    parent_name: &str,
    yaml_data: &Value,
) -> (Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>) {
    let mut fields = Vec::new();
    let mut nested_structs = Vec::new();

    match yaml_data {
        Value::Mapping(mapping) => {
            for (k, v) in mapping {
                let key = format_ident!("{}", k.as_str().unwrap());
                let value_type = match v {
                    Value::String(_) => quote! { String },
                    Value::Number(_) => quote! { i32 },
                    Value::Bool(_) => quote! { bool },
                    Value::Mapping(_) => {
                        let nested_struct_name =
                            format_ident!("{}{}", parent_name, k.as_str().unwrap());

                        let (nested_struct, nested_nested_struct) =
                            generate_struct(&nested_struct_name.to_string(), v);

                        nested_structs.push(quote! {
                            #[derive(Debug, serde::Deserialize)]
                            pub struct #nested_struct_name {
                                #(#nested_struct)*
                            }
                        });

                        nested_structs.extend(nested_nested_struct);
                        // fields.push(quote! {
                        //     pub #key: #nested_struct_name,
                        // });

                        quote! { #nested_struct_name}
                    }
                    // JAMA: c'est ici
                    // Value::Sequence(seq) => {
                    //     // Déterminer le type des éléments dans la séquence
                    //     if let Some(first_value) = seq.first() {
                    //         match first_value {
                    //             Value::String(_) => quote! { Vec<String> },
                    //             Value::Number(_) => quote! { Vec<i32> },
                    //             Value::Bool(_) => quote! { Vec<bool> },
                    //             Value::Mapping(_) => {
                    //                 // Si la séquence contient des structures imbriquées
                    //                 let nested_struct_name =
                    //                     format_ident!("{}{}Item", parent_name, k.as_str().unwrap());

                    //                 // Générer la sous-structure pour les éléments de la séquence
                    //                 let (nested_fields, nested_nested_structs) = generate_struct(
                    //                     &nested_struct_name.to_string(),
                    //                     first_value,
                    //                 );

                    //                 nested_structs.push(quote! {
                    //                     #[derive(Debug, serde::Deserialize)]
                    //                     pub struct #nested_struct_name {
                    //                         #(#nested_fields)*
                    //                     }
                    //                 });

                    //                 // Ajouter les sous-structures imbriquées
                    //                 nested_structs.extend(nested_nested_structs);

                    //                 quote! { Vec<#nested_struct_name> }
                    //             }
                    //             _ => quote! { Vec<serde_yaml::Value> }, // Si le type est inconnu ou mixte
                    //         }
                    //     } else {
                    //         // Si la séquence est vide, utiliser un type générique
                    //         quote! { Vec<serde_yaml::Value> }
                    //     }
                    // }
                    _ => quote! { serde_yaml::Value }, // Pour les types non pris en charge
                };
                fields.push(quote! {
                    pub #key: #value_type,
                });
            }
        }
        _ => panic!("YAML file must contain a mapping"),
    };
    (fields, nested_structs)
}
