#![doc = include_str!("../README.md")]

#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod utils;
use core::{slice::from_raw_parts, str::from_utf8_unchecked};
#[doc(hidden)]
pub use utils::{uDisplayFloat, uDisplayHex, UnstableDoAsFormatter};

/// A collection of methods that are required / used to format a message into a stream.
#[allow(non_camel_case_types)]
pub trait uWrite {
    /// The error associated to this writer
    type Error;

    /// Writes a string slice into this writer, returning whether the write succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully written, and this
    /// method will not return until all data has been written or an error occurs.
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;
}

/// Documentation
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! uformat {
    ($cap:expr, $($tt:tt)*) => {{
        let mut s = heapless::String::<$cap>::new();
        #[allow(unreachable_code)]
        match sfmt::uwrite!(&mut s, $($tt)*) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }};
}

/// Documentation
#[macro_export]
#[cfg(feature = "std")]
macro_rules! uformat {
    ($($tt:tt)*) => {{
        let mut s = String::new();
        #[allow(unreachable_code)]
        match sfmt::uwrite!(&mut s, $($tt)*) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }};
}

/// Write formatted data into a buffer
///
/// This macro accepts a format string, a list of arguments, and a 'writer'. Arguments will be
/// formatted according to the specified format string and the result will be passed to the writer.
/// The writer must have type `[&mut] impl uWrite` or `[&mut] ufmt::Formatter<'_, impl uWrite>`. The
/// macro returns the associated `Error` type of the `uWrite`-r.
///
/// The syntax is similar to [`core::write!`] but only a handful of argument types are accepted:
///
/// [`core::write!`]: https://doc.rust-lang.org/core/macro.write.html
///
/// - `{}` - `uDisplay`
/// - `{:...}` - `uDisplay` formatted and padded
/// - `{:?}` - `uDebug`
/// - `{:#?}` - "pretty" `uDebug`
///
/// Named parameters and "specified" positional parameters (`{0}`) are not supported.
///
/// `{{` and `}}` can be used to escape braces.
pub use nfmt_macros::uwrite;

/// Write formatted data into a buffer, with a newline appended
///
/// See [`uwrite!`](macro.uwrite.html) for more details
pub use nfmt_macros::uwriteln;

/// Configuration for formatting
#[allow(non_camel_case_types)]
pub struct Formatter<'w, W>
where
    W: uWrite + ?Sized,
{
    writer: &'w mut W,
}

impl<'w, W> Formatter<'w, W>
where
    W: uWrite + ?Sized,
{
    /// Creates a formatter from the given writer
    pub fn new(writer: &'w mut W) -> Self {
        Self { writer }
    }

    /// Writes a character to the underlying buffer contained within this formatter.
    pub fn write_char(&mut self, c: char) -> Result<(), W::Error> {
        let mut buf = [0_u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.writer.write_str(s)
    }

    /// Writes a string slice to the underlying buffer contained within this formatter.
    pub fn write_str(&mut self, s: &str) -> Result<(), W::Error> {
        self.writer.write_str(s)
    }

    /// Writes a string slice to the underlying buffer and fills it with the pad_char according to
    /// the padding specifications. Here, `Padding::Usual` is treated in the same way as
    /// `Padding::RightAligned`.
    pub fn write_padded(
        &mut self,
        s: &str,
        pad_char: char,
        padding: Padding,
    ) -> Result<(), W::Error> {
        // Converting a char to &str is expensive, so we only do it once
        let mut buf = [0_u8; 4];
        let pad_c = pad_char.encode_utf8(&mut buf);
        match padding {
            Padding::LeftAligned(pad_length) => {
                self.writer.write_str(s)?;
                for _ in s.len()..pad_length {
                    self.writer.write_str(pad_c)?;
                }
                Ok(())
            }
            Padding::Usual(pad_length) | Padding::RightAligned(pad_length) => {
                for _ in s.len()..pad_length {
                    self.writer.write_str(pad_c)?;
                }
                self.writer.write_str(s)
            }
            Padding::CenterAligned(pad_length) => {
                let padding = pad_length - s.len();
                let half = padding / 2;
                for _ in 0..half {
                    self.writer.write_str(pad_c)?;
                }
                self.writer.write_str(s)?;
                for _ in half..padding {
                    self.writer.write_str(pad_c)?;
                }
                Ok(())
            }
        }
    }
}

/// Implement this trait if `{}` is to be used with the write macro.
#[allow(non_camel_case_types)]
pub trait uDisplay {
    /// Formats the value using the given formatter
    fn fmt<W>(&self, _: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized;
}

/// This enum determines how the display is to be filled (trait [uDisplayFormatted])
#[derive(PartialEq, Clone, Copy)]
pub enum Padding {
    /// Usual padding left or right depending on type
    Usual(usize),
    /// Padding left aligned
    LeftAligned(usize),
    /// Padding right aligned
    RightAligned(usize),
    /// Padding on left and right side
    CenterAligned(usize),
}

/// Creating formatted output string
///
/// This property should be used to obtain a formatted output with the write macro. The source
/// text is parsed by ufmt and the parameters for the call are determined. The interface is
/// defined in such a way that all parameters of the format string are transferred and can be
/// used.
///
/// | Example    | prefix | cmd | padding             | pad_char | behind |
/// |------------|--------|-----|---------------------|----------|--------|
/// | "{:17}"    | false  | '*' | Padding::Usual(17)  | ' '      | 0      |
/// | "{:<17}"   | false  | '*' | Padding::Left(17)   | ' '      | 0      |
/// | "{:>17}"   | false  | '*' | Padding::Right(17)  | ' '      | 0      |
/// | "{:^17}"   | false  | '*' | Padding::Center(17) | ' '      | 0      |
/// | "{:017}"   | false  | '*' | Padding::Usual(17)  | '0'      | 0      |
/// | "{:17.3}"  | false  | '.' | Padding::Usual(17)  | ' '      | 3      |
/// | "{:#10x}"  | true   | 'x' | Padding::Usual(10)  | ' '      | 0      |
///
/// Notes:
/// * This design leads to the fact that not all parameters are usually valid or used. It is up
///   to the implementation to decide how to deal with this: ignore, use default implementations
///   or trigger panic.
/// * Another consequence of this design is that only limited warnings are issued by the parser when
///   writing the source code.
/// * Parameter `cmd` can be ., a..z, A..Z This property can be used to implement advanced formatting
///   functions. If no cmd char is specified in the text, a `*` is set instead.
#[allow(non_camel_case_types)]
pub trait uDisplayFormatted {
    /// Formats the value using the given formatter
    fn fmt_formatted<W>(
        &self,
        _: &mut Formatter<'_, W>,
        prefix: bool,
        cmd: char,
        padding: Padding,
        pad_char: char,
        behind: usize,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized;
}

/// Converts numerical data types to &str
///
/// Convert contains the actual conversion engine of this crate for different data types. It is
/// provided as a public toolbox so that you can integrate your own data types easily and
/// efficiently.
///
/// You will only need this component if you implement the [uDisplay] trait or other traits.
pub struct Convert<const CAP: usize> {
    buf: [u8; CAP],
    idx: usize,
}

impl<const CAP: usize> Convert<CAP> {
    /// Returns a reference to the string contained in Convert
    pub fn as_str(&self) -> &str {
        unsafe {
            let p_buf = self.buf.as_ptr().cast::<u8>();
            let length = CAP - self.idx;
            let slice = from_raw_parts(p_buf.add(self.idx), length);
            from_utf8_unchecked(slice)
        }
    }

    /// Writes a u8 to the buffer and post decrements the idx
    fn write_char(&mut self, c: u8) -> Result<(), ()> {
        if self.idx > 0 {
            let p_buf = self.buf.as_mut_ptr().cast::<u8>();
            self.idx -= 1;
            unsafe { p_buf.add(self.idx).write(c) };
            Ok(())
        } else {
            Err(())
        }
    }

    /// Writes a string to the buffer
    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        for c in s.bytes().rev() {
            self.write_char(c)?;
        }
        Ok(())
    }

    /// Provides non-initialised Convert instance
    unsafe fn uninit() -> Self {
        let buf = core::mem::MaybeUninit::<[u8; CAP]>::uninit();
        let buf = unsafe { buf.assume_init() };
        Convert { buf, idx: CAP }
    }
}

#[allow(non_camel_case_types)]
pub trait uDisplayPadded {
    /// Formats the value using the given formatter
    fn fmt_padded<W>(
        &self,
        _: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized;
}
