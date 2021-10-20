use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
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

            fn duplicate(&self) -> Box<dyn fruity_core::component::component::Component> {
                Box::new(self.clone())
            }

            fn encode(&self, buffer: &mut [u8]) {
                let encoded = unsafe {
                    std::slice::from_raw_parts(
                        (&*self as *const Self) as *const u8,
                        std::mem::size_of::<Self>(),
                    )
                };

                fruity_collections::slice::copy(buffer, encoded);
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
