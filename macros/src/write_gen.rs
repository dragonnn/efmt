use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::cmp::Ordering;

use crate::{
    mk_ident,
    parser::{self, Alignment},
    Piece,
};

use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, LitStr, Token,
};

struct Input {
    formatter: Expr,
    _comma: Token![,],
    literal: LitStr,
    _comma2: Option<Token![,]>,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let formatter = input.parse()?;
        let _comma = input.parse()?;
        let literal = input.parse()?;

        if input.is_empty() {
            Ok(Input {
                formatter,
                _comma,
                literal,
                _comma2: None,
                args: Punctuated::new(),
            })
        } else {
            Ok(Input {
                formatter,
                _comma,
                literal,
                _comma2: input.parse()?,
                args: Punctuated::parse_terminated(input)?,
            })
        }
    }
}

pub fn write(input: TokenStream, newline: bool) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    let formatter = &input.formatter;
    let literal = input.literal;

    let mut format = literal.value();
    if newline {
        format.push('\n');
    }
    let pieces = match parser::parse(&format, literal.span()) {
        Err(e) => return e.to_compile_error().into(),
        Ok(pieces) => pieces,
    };

    let required_args = pieces.iter().filter(|piece| !piece.is_str()).count();
    let supplied_args = input.args.len();
    match supplied_args.cmp(&required_args) {
        Ordering::Less => {
            return parse::Error::new(
                literal.span(),
                &format!(
                    "format string requires {} arguments but {} {} supplied",
                    required_args,
                    supplied_args,
                    if supplied_args == 1 { "was" } else { "were" }
                ),
            )
            .to_compile_error()
            .into();
        }
        Ordering::Greater => {
            return parse::Error::new(
                input.args[required_args].span(),
                "argument never used".to_string(),
            )
            .to_compile_error()
            .into();
        }
        Ordering::Equal => {}
    }

    let mut args = vec![];
    let mut pats = vec![];
    let mut exprs = vec![];
    let mut i = 0;
    for piece in pieces {
        if let Piece::Str(s) = piece {
            exprs.push(quote!(f.write_str(#s)?;))
        } else {
            let pat = mk_ident(i);
            let arg = &input.args[i];
            i += 1;

            args.push(quote!(&(#arg)));
            pats.push(quote!(#pat));

            match piece {
                Piece::Debug(pretty) => {
                    exprs.push(if pretty {
                        quote!(f.pretty(|f| tfmt::uDebug::fmt(#pat, f))?;)
                    } else {
                        quote!(tfmt::uDebug::fmt(#pat, f)?;)
                    });
                }
                Piece::Display => {
                    exprs.push(quote!(tfmt::uDisplay::fmt(#pat, f)?;));
                }
                Piece::Str(_) => unreachable!(),
                Piece::Float {
                    pad_length,
                    pad_char,
                    alignment,
                    behind,
                } => {
                    let alignment = get_alignment(alignment, pad_length);
                    exprs.push(quote!(tfmt::uDisplayFloat::fmt_float(
                        #pat, 
                        f,
                        #alignment,
                        #pad_char,
                        #behind,
                    )?;));
                }
                Piece::Formatted {
                    prefix,
                    cmd,
                    pad_length,
                    pad_char,
                    alignment,
                    behind,
                } => {
                    let alignment = get_alignment(alignment, pad_length);
                    exprs.push(quote!(tfmt::uDisplayFormatted::fmt_formatted(
                        #pat, 
                        f,
                        #prefix,
                        #cmd,
                        #alignment,
                        #pad_char,
                        #behind,
                    )?;));
                }
                Piece::Hex {
                    prefix,
                    cmd,
                    pad_length,
                    pad_char,
                    alignment,
                } => {
                    let alignment = get_alignment(alignment, pad_length);
                    exprs.push(quote!(tfmt::uDisplayHex::fmt_hex(
                        #pat, 
                        f,
                        #prefix,
                        #cmd,
                        #alignment,
                        #pad_char,
                    )?;));
                }
                Piece::Padded {
                    pad_length,
                    pad_char,
                    alignment,
                } => {
                    let alignment = get_alignment(alignment, pad_length);
                    exprs.push(quote!(tfmt::uDisplayPadded::fmt_padded(
                        #pat, 
                        f,
                        #alignment,
                        #pad_char,
                    )?;))
                }
            }
        }
    }

    quote!(match (#(#args),*) {
        (#(#pats),*) => {
            use tfmt::UnstableDoAsFormatter as _;

            (#formatter).do_as_formatter(|f| {
                #(#exprs)*
                core::result::Result::Ok(())
            })
        }
    })
    .into()
}


fn get_alignment(alignment: Alignment, pad_length: usize) -> TokenStream2 {
    match alignment {
        Alignment::Left => quote!(tfmt::Padding::LeftAligned(#pad_length)),
        Alignment::Right => quote!(tfmt::Padding::RightAligned(#pad_length)),
        Alignment::Center => quote!(tfmt::Padding::CenterAligned(#pad_length)),
        Alignment::Usual => quote!(tfmt::Padding::Usual(#pad_length)),
    }
}