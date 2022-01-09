use proc_macro::TokenStream;
use quote::quote;
use syn;

/// Implements From conversion for enum types that represent
/// typesafe unions. Each variant of the enum must contain
/// a distinct type.
///
/// In particular, this allows combining multiple distinct errors
/// into a single enum error type.
///
/// # Examples
///
/// ```
/// use foonetic_macros;
///
/// #[derive(foonetic_macros::From)]
/// #[derive(Debug)]
/// enum MyError {
///     ErrorTypeOne(i8),
///     ErrorTypeTwo(String),
/// }
///
/// fn flaky_one() -> Result<f32, i8> {
///     Ok(3.14)
/// }
///
/// fn flaky_two() -> Result<f32, String> {
///     Ok(2.72)
/// }
///
/// fn flaky() -> Result<f32, MyError> {
///     Ok(flaky_one()? + flaky_two()?)
/// }
///
/// let val = flaky();
/// assert_eq!(val.unwrap(), 3.14 + 2.72)
/// ```
#[proc_macro_derive(From)]
pub fn derive_from(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let enum_name = &ast.ident;
    let enum_data: &syn::DataEnum;
    match ast.data {
        syn::Data::Enum(ref x) => enum_data = x,
        _ => unimplemented!("From only supports Enum"),
    }

    let result = enum_data.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let unnamed_field: &syn::FieldsUnnamed;
        match variant.fields {
            syn::Fields::Unnamed(ref x) => unnamed_field = x,
            _ => unimplemented!("From only supports unnamed Enum fields"),
        }
        assert_eq!(unnamed_field.unnamed.len(), 1);

        let field_tokens = unnamed_field.unnamed.iter().map(|field| {
            let ty = &field.ty;
            quote! {
                impl std::convert::From<#ty> for #enum_name {
                    fn from(e: #ty) -> Self {
                        Self::#ident(e)
                    }
                }
            }
        });
        quote! {
            #(#field_tokens)*
        }
    });

    quote! {
        #(#result)*
    }
    .into()
}
