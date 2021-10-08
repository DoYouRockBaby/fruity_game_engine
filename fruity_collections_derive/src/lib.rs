use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Encodable)]
pub fn derive_encodable_vec_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_collections::encodable::Encodable for #ident {
            fn type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }

            fn encode_size(&self) -> usize {
                std::mem::size_of::<Self>()
            }

            fn encode(&self, buffer: &mut [u8]) {
                let mut result = unsafe {
                    std::slice::from_raw_parts(
                        (self as *const Self) as *const u8,
                        std::mem::size_of::<Self>(),
                    )
                };
                fruity_collections::slice::copy(buffer, &result);
            }

            fn get_decoder(&self) -> fruity_collections::encodable::Decoder {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
                    &body[0]
                }
            }

            fn get_decoder_mut(&self) -> fruity_collections::encodable::DecoderMut {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to_mut::<Self>() };
                    &mut body[0]
                }
            }
        }
    };

    output.into()
}
