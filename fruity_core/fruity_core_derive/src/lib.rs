use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::Index;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let derive_component_trait = derive_component_trait(input.clone());
    let derive_introspect_object_trait = derive_introspect_object_trait(input);

    let derive_component_trait = proc_macro2::TokenStream::from(derive_component_trait);
    let derive_introspect_object_trait = proc_macro2::TokenStream::from(derive_introspect_object_trait);

    let output = quote! {
        #derive_component_trait
        #derive_introspect_object_trait
    };

    output.into()
}


fn derive_component_trait(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let struct_name = ident.to_string();

    let output = quote! {
        impl fruity_core::component::component::Component for #ident {
            fn get_component_type(&self) -> String {
                #struct_name.to_string()
            }

            fn encode_size(&self) -> usize {
                std::mem::size_of::<Self>()
            }

            fn encode(&self, buffer: &mut [u8]) {
                let encoded = unsafe {
                    std::slice::from_raw_parts(
                        (&*self as *const Self) as *const u8,
                        std::mem::size_of::<Self>(),
                    )
                };

                fruity_core::utils::slice::copy(buffer, encoded);
            }

            fn get_decoder(&self) -> fruity_core::component::component::ComponentDecoder {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
                    &body[0]
                }
            }

            fn get_decoder_mut(&self) -> fruity_core::component::component::ComponentDecoderMut {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to_mut::<Self>() };
                    &mut body[0]
                }
            }
        }
    };

    output.into()
}

#[proc_macro_derive(IntrospectObject)]
pub fn derive_introspect_object_trait(input: TokenStream)  -> TokenStream {
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
                        getter: std::sync::Arc::new(|this| fruity_introspect::serialize::serialize::serialize_any(&this.downcast_ref::<#ident>().unwrap().#name).unwrap()),
                        setter: fruity_introspect::SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                            fn convert<
                                T: std::convert::TryFrom<fruity_introspect::serialize::serialized::Serialized>,
                            >(
                                value: fruity_introspect::serialize::serialized::Serialized,
                            ) -> Result<
                                T,
                                <T as std::convert::TryFrom<
                                    fruity_introspect::serialize::serialized::Serialized,
                                >>::Error,
                            > {
                                T::try_from(value)
                            }
        
                            let this = this.downcast_mut::<#ident>().unwrap();

                            match convert::<#ty>(value) {
                                Ok(value) => {
                                    this.#name = value
                                }
                                Err(_) => {
                                    log::error!(
                                        "Expected a {} for property {:?}",
                                        #type_as_string,
                                        #name_as_string,
                                    );
                                }
                            }
                        })),
                    },
                }
            });

            quote! {
                fn get_field_infos(&self) -> Vec<fruity_introspect::FieldInfo> {
                    vec![
                        #(#recurse_infos)*
                    ]
                }
            }
        }
        Data::Union(_) => unimplemented!("Union not supported"),
        Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    let output = quote! {
        impl fruity_introspect::IntrospectObject for #ident {
            #body

            fn get_method_infos(&self) -> Vec<fruity_introspect::MethodInfo> {
                vec![]
            }
        }
    };

    output.into()
}
