use crate::{uWrite, Formatter, Padding};

#[cfg(not(feature = "std"))]
use heapless::String;

// Implementation detail of the `uwrite*!` macros
#[doc(hidden)]
pub trait UnstableDoAsFormatter {
    type Writer: uWrite + ?Sized;

    fn do_as_formatter(
        &mut self,
        f: impl FnOnce(&mut Formatter<'_, Self::Writer>) -> Result<(), <Self::Writer as uWrite>::Error>,
    ) -> Result<(), <Self::Writer as uWrite>::Error>;
}

impl<W> UnstableDoAsFormatter for W
where
    W: uWrite + ?Sized,
{
    type Writer = W;

    fn do_as_formatter(
        &mut self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(&mut Formatter::new(self))
    }
}

impl<W> UnstableDoAsFormatter for Formatter<'_, W>
where
    W: uWrite + ?Sized,
{
    type Writer = W;

    fn do_as_formatter(
        &mut self,
        f: impl FnOnce(&mut Formatter<'_, W>) -> Result<(), W::Error>,
    ) -> Result<(), W::Error> {
        f(self)
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> uWrite for String<N> {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s)
    }
}

#[cfg(feature = "std")]
impl uWrite for String {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s);
        Ok(())
    }
}

// This trait is only intended for use within this crate
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait uDisplayFloat {
    /// Formats the value using the given formatter
    fn fmt_float<W>(
        &self,
        _: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
        behind: usize,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized;
}

// This trait is only intended for use within this crate
#[doc(hidden)]
#[allow(non_camel_case_types)]
pub trait uDisplayHex {
    /// Formats the value using the given formatter
    fn fmt_hex<W>(
        &self,
        _: &mut Formatter<'_, W>,
        prefix: bool,
        cmd: char,
        padding: Padding,
        pad_char: char,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized;
}
