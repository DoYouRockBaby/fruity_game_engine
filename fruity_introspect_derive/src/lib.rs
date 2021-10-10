use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(Introspect)]
pub fn derive_introspect(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let body = match data {
        Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = match data.fields {
                Fields::Named(ref fields) => fields
                    .named
                    .iter()
                    .map(|field| {
                        let ty = &field.ty;
                        match &field.ident {
                            Some(ident) => (quote! { #ident }, quote! { #ty }),
                            None => unimplemented!(),
                        }
                    })
                    .collect(),
                Fields::Unnamed(ref fields) => {
                    // For tuple struct, field name are numbers
                    fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            let ty = &field.ty;
                            let index = Index::from(index);
                            (quote! { #ty }, quote! { #index })
                        })
                        .collect()
                }
                Fields::Unit => {
                    unimplemented!()
                }
            };

            let recurse_infos = fields.iter().map(|(name, ty)| {
                let name_as_string = name.to_string();
                let type_as_string = ty.to_string();
                quote! {
                    fruity_introspect::FieldInfo {
                        name: #name_as_string.to_string(),
                        ty: #type_as_string.to_string(),
                    },
                }
            });

            let recurse_get = fields.iter().map(|(name, _)| {
                let name_as_string = name.to_string();
                quote! {
                    #name_as_string => Some(&self.#name),
                }
            });

            let recurse_set = fields
                .iter()
                .map(|(name, ty)| {
                    let name_as_string = name.to_string();
                    let ty_as_string = ty.to_string();
                    quote! {
                        #name_as_string => match value.downcast_ref::<#ty>() {
                            Some(value) => {
                                self.#name = value.clone();
                            }
                            None => {
                                log::error!("Expected a {} for property {:?}, got {:#?}", #ty_as_string, property, value);
                            }
                        },
                    }
                });
            quote! {
                fn get_field_infos(&self) -> Vec<fruity_introspect::FieldInfo> {
                    vec![
                        #(#recurse_infos)*
                    ]
                }

                fn get_any_field(&self, property: &str) -> Option<&dyn std::any::Any> {
                    match property {
                        #(#recurse_get)*
                        _ => None,
                    }
                }

                fn set_any_field(&mut self, property: &str, value: &dyn std::any::Any) {
                    match property {
                        #(#recurse_set)*
                        _ => log::error!("Trying to access an inexistant property named {} in the component {:#?}", property, self)
                    }
                }
            }
        }
        Data::Union(_) => unimplemented!("Union not supported"),
        Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    let output = quote! {
        impl fruity_introspect::Introspect for #ident {
            #body

            fn get_method_infos(&self) -> Vec<fruity_introspect::MethodInfo> {
                vec![]
            }

            fn call_method(
                &self,
                name: &str,
                args: Vec<Box<dyn std::any::Any>>,
            ) -> Result<Box<dyn std::any::Any>, fruity_introspect::IntrospectError> {
                Err(fruity_introspect::IntrospectError::UnknownMethod(name.to_string()))
            }

            fn call_method_mut(
                &mut self,
                name: &str,
                args: Vec<Box<dyn std::any::Any>>,
            ) -> Result<Box<dyn std::any::Any>, fruity_introspect::IntrospectError> {
                Err(fruity_introspect::IntrospectError::UnknownMethod(name.to_string()))
            }
        }
    };

    output.into()
}
