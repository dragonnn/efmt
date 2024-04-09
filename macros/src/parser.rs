use core::mem;
use std::borrow::Cow;

use proc_macro2::Span;
use syn::Ident;

#[derive(Debug, PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Center,
    Usual,
}

#[derive(Debug, PartialEq)]
pub enum Piece<'a> {
    Display,
    Str(Cow<'a, str>),
    Float {
        pad_length: usize,
        pad_char: char,
        alignment: Alignment,
        behind: usize,
    },
    Formatted {
        prefix: bool,
        cmd: char,
        pad_length: usize,
        pad_char: char,
        alignment: Alignment,
        behind: usize,
    },
    Hex {
        prefix: bool,
        cmd: char,
        pad_length: usize,
        pad_char: char,
        alignment: Alignment,
    },
    Padded {
        pad_length: usize,
        pad_char: char,
        alignment: Alignment,
    },
}

impl Piece<'_> {
    pub fn is_str(&self) -> bool {
        matches!(self, Piece::Str(_))
    }
}

pub fn mk_ident(i: usize) -> Ident {
    Ident::new(&format!("__{}", i), Span::call_site())
}

// `}}` -> `}`
fn unescape(mut literal: &str, span: Span) -> syn::parse::Result<Cow<str>> {
    if literal.contains('}') {
        let mut buf = String::new();

        while literal.contains('}') {
            const ERR: &str = "format string contains an unmatched right brace";
            let mut parts = literal.splitn(2, '}');

            match (parts.next(), parts.next()) {
                (Some(left), Some(right)) => {
                    const ESCAPED_BRACE: &str = "}";

                    if let Some(tail) = right.strip_prefix(ESCAPED_BRACE) {
                        buf.push_str(left);
                        buf.push('}');

                        literal = tail;
                    } else {
                        return Err(syn::parse::Error::new(span, ERR));
                    }
                }

                _ => unreachable!(),
            }
        }

        buf.push_str(literal);

        Ok(buf.into())
    } else {
        Ok(Cow::Borrowed(literal))
    }
}

const INVALID_FORMAT_STR: &str = "invalid format string";

pub fn parse(mut literal: &str, span: Span) -> syn::parse::Result<Vec<Piece>> {
    let mut pieces = vec![];

    let mut buf = String::new();
    loop {
        let mut parts = literal.splitn(2, '{');
        match (parts.next(), parts.next()) {
            // empty string literal
            (None, None) => break,

            // end of the string literal
            (Some(s), None) => {
                if buf.is_empty() {
                    if !s.is_empty() {
                        pieces.push(Piece::Str(unescape(s, span)?));
                    }
                } else {
                    buf.push_str(&unescape(s, span)?);
                    pieces.push(Piece::Str(Cow::Owned(buf)));
                }
                break;
            }

            (head, Some(tail)) => {
                const DISPLAY: &str = "}";
                const ESCAPED_BRACE: &str = "{";

                let head = head.unwrap_or("");
                if tail.starts_with(DISPLAY) || tail.starts_with(':') {
                    if buf.is_empty() {
                        if !head.is_empty() {
                            pieces.push(Piece::Str(unescape(head, span)?));
                        }
                    } else {
                        buf.push_str(&unescape(head, span)?);
                        pieces.push(Piece::Str(Cow::Owned(mem::take(&mut buf))));
                    }

                    if let Some(format) = tail.strip_prefix(':') {
                        let (piece, remainder) = parse_colon(format, span)?;
                        pieces.push(piece);
                        literal = remainder;
                    } else {
                        pieces.push(Piece::Display);
                        literal = &tail[DISPLAY.len()..];
                    }
                } else if let Some(tail) = tail.strip_prefix(ESCAPED_BRACE) {
                    buf.push_str(&unescape(head, span)?);
                    buf.push('{');
                    literal = tail;
                } else {
                    return Err(syn::parse::Error::new(span, INVALID_FORMAT_STR));
                }
            }
        }
    }
    Ok(pieces)
}

/// parses the stuff after a `{:` into a [Piece] and the trailing `&str` (what comes after the `}`)
fn parse_colon(format: &str, span: Span) -> syn::parse::Result<(Piece, &str)> {
    let err_piece = || -> syn::Error { syn::parse::Error::new(span, INVALID_FORMAT_STR) };

    let mut chars = format.chars();
    let ch = chars.next().ok_or(err_piece())?;

    let (ch, pad_char) = if ch == '0' {
        let ch = chars.next().ok_or(err_piece())?;
        (ch, b'0')
    } else {
        (ch, b' ')
    };

    let (ch, alignment) = match ch {
        '<' => (chars.next().ok_or(err_piece())?, Alignment::Left),
        '>' => (chars.next().ok_or(err_piece())?, Alignment::Right),
        '^' => (chars.next().ok_or(err_piece())?, Alignment::Center),
        _ => (ch, Alignment::Usual),
    };

    let (mut ch, prefix) = if ch == '#' {
        let ch = chars.next().ok_or(err_piece())?;
        (ch, true)
    } else {
        (ch, false)
    };

    let mut pad_length = 0_usize;
    while ch.is_ascii_digit() {
        pad_length = pad_length * 10 + ch.to_digit(10).unwrap() as usize;
        ch = chars.next().ok_or(err_piece())?;
    }

    let (mut ch, cmd) = match ch {
        '.' | 'A'..='Z' | 'a'..='z' => (chars.next().ok_or(err_piece())?, ch),
        _ => (ch, '*'),
    };

    let mut behind = 0_usize;
    while ch.is_ascii_digit() {
        behind = behind * 10 + ch.to_digit(10).unwrap() as usize;
        ch = chars.next().ok_or(err_piece())?;
    }

    if ch != '}' {
        return Err(err_piece());
    }
    match cmd {
        '.' => {
            if behind < 7 && prefix == false {
                Ok((
                    Piece::Float {
                        pad_length,
                        pad_char: pad_char as char,
                        alignment,
                        behind,
                    },
                    chars.as_str(),
                ))
            } else {
                Err(err_piece())
            }
        }
        'x' | 'X' => {
            if behind == 0 {
                Ok((
                    Piece::Hex {
                        prefix,
                        cmd,
                        pad_length,
                        pad_char: pad_char as char,
                        alignment,
                    },
                    chars.as_str(),
                ))
            } else {
                Err(err_piece())
            }
        }
        '*' => {
            if behind == 0 && prefix == false {
                Ok((
                    Piece::Padded {
                        pad_length,
                        pad_char: pad_char as char,
                        alignment,
                    },
                    chars.as_str(),
                ))
            } else {
                Err(err_piece())
            }
        }
        _ => Ok((
            Piece::Formatted {
                prefix,
                cmd: cmd,
                pad_length: pad_length,
                pad_char: pad_char as char,
                alignment,
                behind,
            },
            chars.as_str(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::Alignment;
    use crate::Piece;
    use proc_macro2::Span;
    use std::borrow::Cow;

    #[test]
    fn pieces() {
        let span = Span::call_site();

        // string interpolation
        assert_eq!(
            super::parse("The answer is {}", span).ok(),
            Some(vec![
                Piece::Str(Cow::Borrowed("The answer is ")),
                Piece::Display
            ]),
        );

        assert_eq!(
            super::parse("{:.0}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 0,
                pad_char: ' ',
                alignment: Alignment::Usual,
                behind: 0
            }]),
        );

        assert_eq!(
            super::parse("{:.6}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 0,
                pad_char: ' ',
                alignment: Alignment::Usual,
                behind: 6
            }]),
        );

        assert_eq!(
            super::parse("{:17.6}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Usual,
                behind: 6
            }]),
        );

        assert_eq!(
            super::parse("{:<17.6}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Left,
                behind: 6
            }]),
        );

        assert_eq!(
            super::parse("{:>17.6}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Right,
                behind: 6
            }]),
        );

        assert_eq!(
            super::parse("{:^17.6}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Center,
                behind: 6
            }]),
        );

        assert_eq!(
            super::parse("{:0^17.6}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 17,
                pad_char: '0',
                alignment: Alignment::Center,
                behind: 6
            }]),
        );

        assert_eq!(
            super::parse("{:0^20.2}", span).ok(),
            Some(vec![Piece::Float {
                pad_length: 20,
                pad_char: '0',
                alignment: Alignment::Center,
                behind: 2
            }]),
        );

        assert_eq!(
            super::parse("{:<27}", span).ok(),
            Some(vec![Piece::Padded {
                pad_length: 27,
                pad_char: ' ',
                alignment: Alignment::Left,
            }]),
        );

        assert_eq!(
            super::parse("{:>27}", span).ok(),
            Some(vec![Piece::Padded {
                pad_length: 27,
                pad_char: ' ',
                alignment: Alignment::Right,
            }]),
        );

        assert_eq!(
            super::parse("{:^27}", span).ok(),
            Some(vec![Piece::Padded {
                pad_length: 27,
                pad_char: ' ',
                alignment: Alignment::Center,
            }]),
        );

        assert_eq!(
            super::parse("{:27}", span).ok(),
            Some(vec![Piece::Padded {
                pad_length: 27,
                pad_char: ' ',
                alignment: Alignment::Usual,
            }]),
        );

        assert_eq!(
            super::parse("{:0<27}", span).ok(),
            Some(vec![Piece::Padded {
                pad_length: 27,
                pad_char: '0',
                alignment: Alignment::Left,
            }]),
        );

        assert_eq!(
            super::parse("{:x}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: false,
                cmd: 'x',
                pad_length: 0,
                pad_char: ' ',
                alignment: Alignment::Usual,
            }]),
        );

        assert_eq!(
            super::parse("{:17X}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: false,
                cmd: 'X',
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Usual,
            }]),
        );

        assert_eq!(
            super::parse("{:<17X}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: false,
                cmd: 'X',
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Left,
            }]),
        );

        assert_eq!(
            super::parse("{:>17X}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: false,
                cmd: 'X',
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Right,
            }]),
        );

        assert_eq!(
            super::parse("{:^17X}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: false,
                cmd: 'X',
                pad_length: 17,
                pad_char: ' ',
                alignment: Alignment::Center,
            }]),
        );

        assert_eq!(
            super::parse("{:0^17X}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: false,
                cmd: 'X',
                pad_length: 17,
                pad_char: '0',
                alignment: Alignment::Center,
            }]),
        );

        assert_eq!(
            super::parse("{:0^#17X}", span).ok(),
            Some(vec![Piece::Hex {
                prefix: true,
                cmd: 'X',
                pad_length: 17,
                pad_char: '0',
                alignment: Alignment::Center,
            }]),
        );

        // escaped braces
        assert_eq!(
            super::parse("{{}} is not an argument", span).ok(),
            Some(vec![Piece::Str(Cow::Borrowed("{} is not an argument"))]),
        );

        // left brace & junk
        assert!(super::parse("{", span).is_err());
        assert!(super::parse(" {", span).is_err());
        assert!(super::parse("{ ", span).is_err());
        assert!(super::parse("{ {", span).is_err());
    }

    #[test]
    fn unescape() {
        let span = Span::call_site();

        // no right brace
        assert_eq!(super::unescape("", span).ok(), Some(Cow::Borrowed("")));
        assert_eq!(
            super::unescape("Hello", span).ok(),
            Some(Cow::Borrowed("Hello"))
        );

        // unmatched right brace
        assert!(super::unescape(" }", span).is_err());
        assert!(super::unescape("} ", span).is_err());
        assert!(super::unescape("}", span).is_err());

        // escaped right brace
        assert_eq!(super::unescape("}}", span).ok(), Some(Cow::Borrowed("}")));
        assert_eq!(super::unescape("}} ", span).ok(), Some(Cow::Borrowed("} ")));
    }
}
