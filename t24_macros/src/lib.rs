extern crate proc_macro2;

use proc_macro::TokenStream;

use heck::*;
use quote::*;
use syn::*;
use syn::parse::*;
use syn::punctuated::*;

use fns::*;
use structs::*;
use traits::to_lowercase::*;
use traits::to_strings::*;

mod fns;
mod impls;
mod structs;
mod traits;

#[proc_macro_derive(EnumI64Derive)]
pub fn instrument_derive(input: TokenStream) -> TokenStream {
    let mut ts = quote!();
    let q5 = EnumI64::_from(input.clone());
    let ts5 = proc_macro2::TokenStream::from(q5);
    ts.extend(quote! {
        #ts5
    });
    let q = quote! {
        #ts
    };
    q.into()
}

