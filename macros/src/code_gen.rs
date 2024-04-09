use proc_macro::TokenStream;
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
                Piece::Display => {
                    exprs.push(quote!(sfmt::uDisplay::fmt(#pat, f)?;));
                }
                Piece::Str(_) => unreachable!(),
                Piece::Float {
                    pad_length,
                    pad_char,
                    alignment: format,
                    behind,
                } => match format {
                    Alignment::Left => exprs.push(quote!(sfmt::uDisplayFloat::fmt_float(
                                #pat, 
                                f,
                                sfmt::Padding::LeftAligned(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                    Alignment::Right => exprs.push(quote!(sfmt::uDisplayFloat::fmt_float(
                                #pat, 
                                f, 
                                sfmt::Padding::RightAligned(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                    Alignment::Center => exprs.push(quote!(sfmt::uDisplayFloat::fmt_float(
                                #pat, 
                                f, 
                                sfmt::Padding::CenterAligned(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                    Alignment::Usual => exprs.push(quote!(sfmt::uDisplayFloat::fmt_float(
                                #pat, 
                                f, 
                                sfmt::Padding::Usual(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                },
                Piece::Formatted {
                    prefix,
                    cmd,
                    pad_length,
                    pad_char,
                    alignment: format,
                    behind,
                } => match format {
                    Alignment::Left => exprs.push(quote!(sfmt::uDisplayFormatted::fmt_formatted(
                                #pat, 
                                f,
                                #prefix,
                                #cmd,
                                sfmt::Padding::LeftAligned(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                    Alignment::Right => exprs.push(quote!(sfmt::uDisplayFormatted::fmt_formatted(
                                #pat, 
                                f, 
                                #prefix,
                                #cmd,
                                sfmt::Padding::RightAligned(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                    Alignment::Center => exprs.push(quote!(sfmt::uDisplayFormatted::fmt_formatted(
                                #pat, 
                                f, 
                                #prefix,
                                #cmd,
                                sfmt::Padding::CenterAligned(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                    Alignment::Usual => exprs.push(quote!(sfmt::uDisplayFormatted::fmt_formatted(
                                #pat, 
                                f, 
                                #prefix,
                                #cmd,
                                sfmt::Padding::Usual(#pad_length),
                                #pad_char,
                                #behind,
                            )?;)),
                },
                Piece::Hex {
                    prefix,
                    cmd,
                    pad_length,
                    pad_char,
                    alignment: format,
                } => match format {
                    Alignment::Left => exprs.push(quote!(sfmt::uDisplayHex::fmt_hex(
                                #pat, 
                                f,
                                #prefix,
                                #cmd,
                                sfmt::Padding::LeftAligned(#pad_length),
                                #pad_char,
                            )?;)),
                    Alignment::Right => exprs.push(quote!(sfmt::uDisplayHex::fmt_hex(
                                #pat, 
                                f, 
                                #prefix,
                                #cmd,
                                sfmt::Padding::RightAligned(#pad_length),
                                #pad_char,
                            )?;)),
                    Alignment::Center => exprs.push(quote!(sfmt::uDisplayHex::fmt_hex(
                                #pat, 
                                f, 
                                #prefix,
                                #cmd,
                                sfmt::Padding::CenterAligned(#pad_length),
                                #pad_char,
                            )?;)),
                    Alignment::Usual => exprs.push(quote!(sfmt::uDisplayHex::fmt_hex(
                                #pat, 
                                f, 
                                #prefix,
                                #cmd,
                                sfmt::Padding::Usual(#pad_length),
                                #pad_char,
                            )?;)),
                },
                Piece::Padded {
                    pad_length,
                    pad_char,
                    alignment: format,
                } => match format {
                    Alignment::Left => exprs.push(quote!(sfmt::uDisplayPadded::fmt_padded(
                                #pat, 
                                f,
                                sfmt::Padding::LeftAligned(#pad_length),
                                #pad_char,
                            )?;)),
                    Alignment::Right => exprs.push(quote!(sfmt::uDisplayPadded::fmt_padded(
                                #pat, 
                                f, 
                                sfmt::Padding::RightAligned(#pad_length),
                                #pad_char,
                            )?;)),
                    Alignment::Center => exprs.push(quote!(sfmt::uDisplayPadded::fmt_padded(
                                #pat, 
                                f, 
                                sfmt::Padding::CenterAligned(#pad_length),
                                #pad_char,
                            )?;)),
                    Alignment::Usual => exprs.push(quote!(sfmt::uDisplayPadded::fmt_padded(
                                #pat, 
                                f, 
                                sfmt::Padding::Usual(#pad_length),
                                #pad_char,
                            )?;)),
                },
            }
        }
    }

    quote!(match (#(#args),*) {
        (#(#pats),*) => {
            use sfmt::UnstableDoAsFormatter as _;

            (#formatter).do_as_formatter(|f| {
                #(#exprs)*
                core::result::Result::Ok(())
            })
        }
    })
    .into()
}
