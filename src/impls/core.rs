use crate::{uDebug, udisplay_as_udebug, uDisplay, uDisplayPadded, uWrite, Formatter, Padding};

impl uDisplay for bool {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        if *self {
            f.write_str("true")
        } else {
            f.write_str("false")
        }
    }
}

udisplay_as_udebug!(bool);

impl uDisplayPadded for bool {
    fn fmt_padded<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let s = if *self { "true" } else { "false" };
        s.fmt_padded(fmt, padding, pad_char)
    }
}

impl uDisplay for char {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.write_char(*self)
    }
}

udisplay_as_udebug!(char);

impl uDisplayPadded for char {
    fn fmt_padded<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let mut buf = [0_u8; 4];
        let pad_c: &str = (*self).encode_utf8(&mut buf);
        pad_c.fmt_padded(fmt, padding, pad_char)
    }
}

impl uDisplay for str {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        f.write_str(self)
    }
}

udisplay_as_udebug!(str);

impl uDisplayPadded for &str {
    fn fmt_padded<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let padding = match padding {
            Padding::Usual(pad_length) => Padding::LeftAligned(pad_length),
            _ => padding,
        };
        fmt.write_padded(*self, pad_char, padding)
    }
}

impl<T> uDebug for &'_ T
where
    T: uDebug + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for &'_ T
where
    T: uDisplay + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDisplay>::fmt(self, f)
    }
}

impl<T> uDebug for &'_ mut T
where
    T: uDebug + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDebug>::fmt(self, f)
    }
}

impl<T> uDisplay for &'_ mut T
where
    T: uDisplay + ?Sized,
{
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <T as uDisplay>::fmt(self, f)
    }
}

impl<T> uDebug for Option<T>
where
    T: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            None => f.write_str("None"),
            Some(x) => f.debug_tuple("Some")?.field(x)?.finish(),
        }
    }
}

impl<T, E> uDebug for Result<T, E>
where
    T: uDebug,
    E: uDebug,
{
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self {
            Err(e) => f.debug_tuple("Err")?.field(e)?.finish(),
            Ok(x) => f.debug_tuple("Ok")?.field(x)?.finish(),
        }
    }
}
