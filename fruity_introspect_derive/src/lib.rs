use proc_macro::{self, TokenStream};
use quote::format_ident;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(IntrospectFields)]
pub fn derive_introspect_fields(input: TokenStream) -> TokenStream {
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
                let type_converter_as_string = format_ident!("as_{}", &type_as_string);

                quote! {
                    fruity_introspect::FieldInfo::<fruity_ecs::serialize::serialized::Serialized> {
                        name: #name_as_string.to_string(),
                        ty: #type_as_string.to_string(),
                        getter: |this| fruity_ecs::serialize::serialize::serialize_any(&this.downcast_ref::<#ident>().unwrap().#name).unwrap(),
                        setter: |this, value| {
                            let this = this.downcast_mut::<#ident>().unwrap();

                            match value.#type_converter_as_string () {
                                Some(value) => {
                                    this.#name = value.clone();
                                }
                                None => {
                                    log::error!(
                                        "Expected a {} for property {:?}, got {:#?}",
                                        #type_as_string,
                                        #name_as_string,
                                        value
                                    );
                                }
                            }
                        },
                    },
                }
            });

            quote! {
                fn get_field_infos(&self) -> Vec<fruity_introspect::FieldInfo<fruity_ecs::serialize::serialized::Serialized>> {
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
        impl fruity_introspect::IntrospectFields<fruity_ecs::serialize::serialized::Serialized> for #ident {
            #body
        }
    };

    output.into()
}
