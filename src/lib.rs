// for enum,we'd like to generate From impls for each variant
use proc_macro::TokenStream;
use quote::quote;
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{:#?}", input);
    // get the enum name
    let enum_name = input.ident;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("EnumFrom derive only works on enums"),
    };
    let from_impls = variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;
        let fields = &variant.fields;
        match fields {
            syn::Fields::Unnamed(unnamed) => {
                if let Some(field) = unnamed.unnamed.first() {
                    let inner_type = &field.ty;
                    Some(quote! {
                        impl From<#inner_type> for #enum_name {
                            fn from(v: #inner_type) -> Self {
                                Self::#variant_name(v)
                            }
                        }
                    })
                } else {
                    None
                }
            }
            syn::Fields::Unit => Some(quote! {}),
            _ => None,
        }
    });
    quote! { #(#from_impls)* }.into()
}
