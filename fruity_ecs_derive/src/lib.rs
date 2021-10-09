use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let struct_name = ident.to_string();

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
                fn get_component_type(&self) -> String {
                    #struct_name.to_string()
                }
                fn get_untyped_field(&self, property: &str) -> Option<&dyn std::any::Any> {
                    match property {
                        #(#recurse_get)*
                        _ => None,
                    }
                }
                fn set_untyped_field(&mut self, property: &str, value: &dyn std::any::Any) {
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
        impl fruity_ecs::component::component::Component for #ident {
            #body

            fn encode_size(&self) -> usize {
                std::mem::size_of::<Self>()
            }

            fn encode(self: Box<Self>, buffer: &mut [u8]) {
                let encoded = unsafe {
                    std::slice::from_raw_parts(
                        (&*self as *const Self) as *const u8,
                        std::mem::size_of::<Self>(),
                    )
                };

                fruity_collections::slice::copy(buffer, encoded);
            }

            fn get_decoder(&self) -> fruity_ecs::component::component::ComponentDecoder {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
                    &body[0]
                }
            }

            fn get_decoder_mut(&self) -> fruity_ecs::component::component::ComponentDecoderMut {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to_mut::<Self>() };
                    &mut body[0]
                }
            }
        }
    };

    output.into()
}
