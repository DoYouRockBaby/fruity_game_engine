use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TraitVecObject)]
pub fn derive_trait_vec_object(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_collections::TraitVecObject for #ident {
            fn encode(&self) -> Vec<u8> {
                unsafe {
                    std::slice::from_raw_parts(
                        (self as *const Self) as *const u8,
                        std::mem::size_of::<Self>(),
                    )
                    .to_vec()
                }
            }

            fn get_decoder(&self) -> fruity_collections::TraitVecObjectDecoder {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to::<Self>() };
                    &body[0]
                }
            }

            fn get_decoder_mut(&self) -> fruity_collections::TraitVecObjectDecoderMut {
                |data| {
                    let (_head, body, _tail) = unsafe { data.align_to_mut::<Self>() };
                    &mut body[0]
                }
            }
        }
    };

    output.into()
}
