//! `tfmt` macros

extern crate proc_macro;

mod parser;
use parser::{mk_ident, Piece};

mod write_gen;
use write_gen::write;

mod debug_gen;
use debug_gen::debug_gen;

use proc_macro::TokenStream;

#[proc_macro]
pub fn uwrite(input: TokenStream) -> TokenStream {
    write(input, false)
}

#[proc_macro]
pub fn uwriteln(input: TokenStream) -> TokenStream {
    write(input, true)
}

#[proc_macro_derive(uDebug)]
pub fn debug(input: TokenStream) -> TokenStream {
    debug_gen(input)
}
