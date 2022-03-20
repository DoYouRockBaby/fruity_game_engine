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
    let derive_introspect_object_trait = derive_introspect_object_trait(input.clone());
    let derive_instantiable_object_trait = derive_component_instantiable_object_trait(input);

    let derive_component_trait = proc_macro2::TokenStream::from(derive_component_trait);
    let derive_introspect_object_trait = proc_macro2::TokenStream::from(derive_introspect_object_trait);
    let derive_instantiable_object_trait = proc_macro2::TokenStream::from(derive_instantiable_object_trait);

    let output = quote! {
        #derive_component_trait
        #derive_introspect_object_trait
        #derive_instantiable_object_trait
    };

    output.into()
}


fn derive_component_trait(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let struct_name = ident.to_string();

    let output = quote! {
        impl fruity_ecs::component::component::Component for #ident {
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

            fn get_decoder(&self) -> fruity_ecs::component::component::ComponentDecoder {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
                    &body[0]
                }
            }

            fn duplicate(&self) -> Box<dyn fruity_ecs::component::component::Component> {
                Box::new(self.clone())
            }
        }

        impl fruity_ecs::component::component::StaticComponent for #ident {
            fn get_component_name() -> String {
                #struct_name.to_string()
            }
        }
    };

    output.into()
}

#[proc_macro_derive(IntrospectObject)]
pub fn derive_introspect_object_trait(input: TokenStream)  -> TokenStream {
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

            let recurse_infos = fields.iter().map(|(name, ty)| {
                let name_as_string = name.to_string();
                let type_as_string = ty.to_string();

                quote! {
                    fruity_core::introspect::FieldInfo {
                        name: #name_as_string.to_string(),
                        serializable: true,
                        getter: std::sync::Arc::new(|this| this.downcast_ref::<#ident>().unwrap().#name.clone().fruity_into()),
                        setter: fruity_core::introspect::SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                            fn convert<
                                T: fruity_core::convert::FruityTryFrom<fruity_core::serialize::serialized::Serialized>,
                            >(
                                value: fruity_core::serialize::serialized::Serialized,
                            ) -> Result<
                                T,
                                <T as fruity_core::convert::FruityTryFrom<
                                    fruity_core::serialize::serialized::Serialized,
                                >>::Error,
                            > {
                                T::fruity_try_from(value)
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
                fn get_field_infos(&self) -> Vec<fruity_core::introspect::FieldInfo> {
                    use fruity_core::convert::FruityInto;

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
        impl fruity_core::introspect::IntrospectObject for #ident {
            fn get_class_name(&self) -> String {
                #struct_name.to_string()
            }

            #body

            fn get_method_infos(&self) -> Vec<fruity_core::introspect::MethodInfo> {
                vec![]
            }
        }
    };

    output.into()
}

fn derive_component_instantiable_object_trait(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_core::introspect::InstantiableObject for #ident {
            fn get_constructor() -> fruity_core::introspect::Constructor {
                use fruity_core::introspect::IntrospectObject;

                std::sync::Arc::new(|_resource_container: std::sync::Arc<fruity_core::resource::resource_container::ResourceContainer>, mut args: Vec<fruity_core::serialize::serialized::Serialized>| {
                    let mut new_object = #ident::default();

                    if args.len() > 0 {
                        let serialized = args.remove(0);
                        let new_object_fields = new_object.get_field_infos();

                        if let fruity_core::serialize::serialized::Serialized::SerializedObject { fields, .. } =
                            serialized
                        {
                            fields.into_iter().for_each(|(key, value)| {
                                let field_info = new_object_fields
                                    .iter()
                                    .find(|field_info| field_info.name == *key);

                                if let Some(field_info) = field_info {
                                    match &field_info.setter {
                                        fruity_core::introspect::SetterCaller::Const(call) => {
                                            call(new_object.as_any_ref(), value);
                                        }
                                        fruity_core::introspect::SetterCaller::Mut(call) => {
                                            call(new_object.as_any_mut(), value);
                                        }
                                        fruity_core::introspect::SetterCaller::None => (),
                                    }
                                }
                            })
                        };
                    };
        
                    Ok(fruity_core::serialize::serialized::Serialized::NativeObject(Box::new(fruity_ecs::component::component::AnyComponent::new(new_object))))
                })
            }
        }
    };

    output.into()
}

#[proc_macro_derive(InstantiableObject)]
pub fn derive_instantiable_object_trait(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_core::introspect::InstantiableObject for #ident {
            fn get_constructor() -> fruity_core::introspect::Constructor {
                use fruity_core::convert::FruityInto;
                use fruity_core::introspect::IntrospectObject;

                std::sync::Arc::new(|_resource_container: std::sync::Arc<fruity_core::resource::resource_container::ResourceContainer>, mut args: Vec<fruity_core::serialize::serialized::Serialized>| {
                    let mut new_object = #ident::default();

                    if args.len() > 0 {
                        let serialized = args.remove(0);
                        let new_object_fields = new_object.get_field_infos();

                        if let fruity_core::serialize::serialized::Serialized::SerializedObject { fields, .. } =
                            serialized
                        {
                            fields.into_iter().for_each(|(key, value)| {
                                let field_info = new_object_fields
                                    .iter()
                                    .find(|field_info| field_info.name == *key);

                                if let Some(field_info) = field_info {
                                    match &field_info.setter {
                                        fruity_core::introspect::SetterCaller::Const(call) => {
                                            call(new_object.as_any_ref(), value);
                                        }
                                        fruity_core::introspect::SetterCaller::Mut(call) => {
                                            call(new_object.as_any_mut(), value);
                                        }
                                        fruity_core::introspect::SetterCaller::None => (),
                                    }
                                }
                            })
                        };
                    };
        
                    Ok(new_object.fruity_into())
                })
            }
        }
    };

    output.into()
}

#[proc_macro_derive(SerializableObject)]
pub fn derive_serializable_object(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let ident_as_string = ident.to_string();

    let output = quote! {
        impl fruity_core::serialize::serialized::SerializableObject for #ident {
            fn duplicate(&self) -> Box<dyn fruity_core::serialize::serialized::SerializableObject> {
                Box::new(self.clone())
            }
        }
        
        impl fruity_core::convert::FruityTryFrom<fruity_core::serialize::serialized::Serialized> for #ident {
            type Error = String;
        
            fn fruity_try_from(value: fruity_core::serialize::serialized::Serialized) -> Result<Self, Self::Error> {
                match value {
                    fruity_core::serialize::serialized::Serialized::NativeObject(value) => {
                        match value.as_any_box().downcast::<#ident>() {
                            Ok(value) => Ok(*value),
                            Err(_) => Err(format!(
                                "Couldn't convert a {} to native object", #ident_as_string
                            )),
                        }
                    }
                    _ => Err(format!("Couldn't convert {:?} to native object", value)),
                }
            }
        }
        
        impl fruity_core::convert::FruityInto<fruity_core::serialize::serialized::Serialized> for #ident {
            fn fruity_into(self) -> fruity_core::serialize::serialized::Serialized {
                fruity_core::serialize::serialized::Serialized::NativeObject(Box::new(self))
            }
        }
    };

    output.into()
}