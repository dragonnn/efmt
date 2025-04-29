use crate::{uWrite, Formatter, Padding};

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

#[cfg(feature = "heapless07")]
impl<const N: usize> uWrite for heapless07::String<N> {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s)
    }
}

#[cfg(feature = "heapless08")]
impl<const N: usize> uWrite for heapless08::String<N> {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s)
    }
}

#[cfg(feature = "heapless09")]
impl<const N: usize, L: heapless09::LenType> uWrite for heapless09::String<N, L> {
    type Error = heapless09::CapacityError;

    fn write_str(&mut self, s: &str) -> Result<(), heapless09::CapacityError> {
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

#[cfg(feature = "alloc")]
impl uWrite for alloc::string::String {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s);
        Ok(())
    }
}

impl uWrite for dyn core::fmt::Write {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_str(s).map_err(|_| ())?;
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
