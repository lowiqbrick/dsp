use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(EnumIter)]
pub fn macro_implementation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(enum_data) = &input.data else {
        panic!("EnumIter can only be derived for enums");
    };

    let variants: Vec<Ident> = enum_data
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();

    let iterator_impl = quote! {
        impl #name {
            pub fn variants() -> Vec<Self> {
                vec![
                    #(Self::#variants),*
                ]
            }
        }
    };

    iterator_impl.into()
}
