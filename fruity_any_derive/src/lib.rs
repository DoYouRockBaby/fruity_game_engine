use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FruityAny)]
pub fn derive_encodable_any(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_any::FruityAny for #ident {
            fn as_any_ref(&self) -> &fruity_any::FruityAny {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn fruity_any::FruityAny {
                self
            }

            fn as_any_box(self: Box<Self>) -> Box<dyn fruity_any::FruityAny> {
                self
            }
        }
    };

    output.into()
}
