#![doc = include_str!("../README.md")]

extern crate proc_macro;

mod parser;
use parser::{mk_ident, Piece};

mod write_gen;
use write_gen::write;

mod debug_gen;
use debug_gen::debug_gen;

use proc_macro::TokenStream;

/// Parse format string and generate code
#[proc_macro]
pub fn uwrite(input: TokenStream) -> TokenStream {
    write(input, false)
}

/// Like uwrite, additionally generates a line feed
#[proc_macro]
pub fn uwriteln(input: TokenStream) -> TokenStream {
    write(input, true)
}

/// Automatically derive the `uDebug` trait for a `struct` or `enum`
///
/// Supported items
///
/// - `struct`-s
/// - `enum`-s
///
/// `union`-s are not supported
#[proc_macro_derive(uDebug)]
pub fn debug(input: TokenStream) -> TokenStream {
    debug_gen(input)
}
