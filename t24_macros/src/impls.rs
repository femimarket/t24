use proc_macro::TokenStream;

use quote::*;
use syn::*;
use syn::parse::Parser;

use crate::*;

impl EnumI64 {

    pub fn _from(input:TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let _id_name = &input.ident;
        let idents = ident_enum_variants(&input);
        let q = quote! {
            impl #_id_name {
                pub fn from_i64(v:i64) -> Self {
                    match v {
                         #(x if x == #_id_name::#idents as i64 => #_id_name::#idents),*,
                        _ => panic!("unknown enum {} {v}",stringify!(_id_name)),
                    }
                }
            }
        };
        q.into()
    }

}
