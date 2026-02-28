use proc_macro::TokenStream;
use quote::quote;

pub fn process_enum_from(input: &syn::DeriveInput) -> TokenStream {
    let enum_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let variants = match &input.data {
        syn::Data::Enum(data_enum) => &data_enum.variants,
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
                        impl #impl_generics From<#inner_type> for #enum_name #ty_generics #where_clause {
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
