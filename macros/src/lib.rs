//! `nfmt` macros

extern crate proc_macro;

mod parser;
use parser::{mk_ident, Piece};

mod code_gen;
use code_gen::write;

use proc_macro::TokenStream;

#[proc_macro]
pub fn uwrite(input: TokenStream) -> TokenStream {
    write(input, false)
}

#[proc_macro]
pub fn uwriteln(input: TokenStream) -> TokenStream {
    write(input, true)
}
