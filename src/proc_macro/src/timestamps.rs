use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Field, Fields, FieldsNamed, ItemStruct};

/// This macro will automatically append the following fields to a struct:
///
/// ```ignore
/// pub created_by: String,
/// pub created_at: NaiveDateTime,
/// pub updated_by: String,
/// pub updated_at: NaiveDateTime,
/// ```
///
/// Within the framework, this macro is only applied to database models.
///
/// # Examples
///
/// ```ignore
/// #[timestamps]
/// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
/// pub struct User {
///     pub id: String,
///     pub first_name: String,
///     pub last_name: String,
///     pub email: String,
///     pub password: String,
/// }
/// ```
pub fn add(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    if let Fields::Named(ref mut fields) = input.fields {
        add_field(fields, quote! { pub created_by: String });
        add_field(fields, quote! { pub created_at: NaiveDateTime });
        add_field(fields, quote! { pub updated_by: String });
        add_field(fields, quote! { pub updated_at: NaiveDateTime });
    }

    TokenStream::from(quote! { #input })
}

pub(crate) fn add_field(fields: &mut FieldsNamed, field: TokenStream2) {
    Field::parse_named
        .parse2(field.clone())
        .map(|new_field| {
            fields.named.push(new_field);
        })
        .expect(&format!("Could not parse field {}", field));
}
