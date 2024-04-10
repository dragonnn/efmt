use crate::{
    uDisplay, uDebug, udisplay_as_udebug, uDisplayHex, uDisplayPadded, uWrite, Convert, 
    Formatter, Padding
};
use core::{mem::MaybeUninit, slice, str};

macro_rules! hex {
    ($utype: ty, $n:expr, $upper: expr, $prefix: expr) => {{
        let mut buf = [MaybeUninit::<u8>::uninit(); core::mem::size_of::<$utype>() * 2 + 2];
        let ptr = &buf.as_mut_ptr().cast::<u8>();
        let len = core::mem::size_of::<$utype>() * 2 + 2;
        let mut n = $n;
        let c = if $upper { b'A' } else { b'a' };
        let mut i = len - 1;
        loop {
            let val = (n % 16) as u8;
            let d = if val < 10 { b'0' + val } else { c + val - 10 };
            unsafe { ptr.add(i).write(d) }

            n /= 16;
            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        if $prefix {
            i -= 1;
            unsafe { ptr.add(i).write(b'x') }
            i -= 1;
            unsafe { ptr.add(i).write(b'0') }
        }

        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), len - i)) }
    }};
}

macro_rules! hex_trait_impl {
    ($type: ty, $utype: ty) => {
        impl uDisplayHex for $type {
            fn fmt_hex<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                prefix: bool,
                cmd: char,
                padding: Padding,
                pad_char: char,
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                let s = hex!($utype, *self as $utype, cmd == 'X', prefix);
                fmt.write_padded(s, pad_char, padding)
            }
        }
    };
}

hex_trait_impl!(u8, u8);
hex_trait_impl!(u16, u16);
hex_trait_impl!(u32, u32);
hex_trait_impl!(u64, u64);
hex_trait_impl!(u128, u128);

#[cfg(target_pointer_width = "16")]
hex_trait_impl!(usize, u16);
#[cfg(target_pointer_width = "32")]
hex_trait_impl!(usize, u32);
#[cfg(target_pointer_width = "64")]
hex_trait_impl!(usize, u64);

hex_trait_impl!(i8, u8);
hex_trait_impl!(i16, u16);
hex_trait_impl!(i32, u32);
hex_trait_impl!(i64, u64);
hex_trait_impl!(i128, u128);

#[cfg(target_pointer_width = "16")]
hex_trait_impl!(isize, u16);
#[cfg(target_pointer_width = "32")]
hex_trait_impl!(isize, u32);
#[cfg(target_pointer_width = "64")]
hex_trait_impl!(isize, u64);

macro_rules! uxx {
    ($n:expr, $len:expr) => {{
        let mut buf = [MaybeUninit::<u8>::uninit(); $len];
        let ptr = &buf.as_mut_ptr().cast::<u8>();
        let mut n = $n;
        let mut i = $len - 1;
        loop {
            unsafe { ptr.add(i).write((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), $len - i)) }
    }};
}

macro_rules! uxx_trait_impl {
    ($utype: ty, $len:expr) => {
        impl uDisplay for $utype {
            fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                fmt.write_str(uxx!(*self, $len))
            }
        }

        impl uDisplayPadded for $utype {
            fn fmt_padded<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                padding: Padding,
                pad_char: char,
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                let s = uxx!(*self, $len);
                fmt.write_padded(s, pad_char, padding)
            }
        }
    };
}

uxx_trait_impl!(u8, 3);
uxx_trait_impl!(u16, 5);
uxx_trait_impl!(u32, 10);
uxx_trait_impl!(u64, 20);
uxx_trait_impl!(u128, 39);

#[cfg(target_pointer_width = "16")]
uxx_trait_impl!(usize, 5);
#[cfg(target_pointer_width = "32")]
uxx_trait_impl!(usize, 10);
#[cfg(target_pointer_width = "64")]
uxx_trait_impl!(usize, 20);

udisplay_as_udebug!(u8);
udisplay_as_udebug!(u16);
udisplay_as_udebug!(u32);
udisplay_as_udebug!(u64);
udisplay_as_udebug!(u128);
udisplay_as_udebug!(usize);

macro_rules! ixx {
    ($uxx:ty, $n:expr, $len:expr) => {{
        let mut buf = [MaybeUninit::<u8>::uninit(); $len];
        let ptr = &buf.as_mut_ptr().cast::<u8>();
        let n = $n;
        let negative = n.is_negative();
        let mut n = if negative {
            match n.checked_abs() {
                Some(n) => n as $uxx,
                None => <$uxx>::max_value() / 2 + 1,
            }
        } else {
            n as $uxx
        };
        let mut i = $len - 1;
        loop {
            unsafe { ptr.add(i).write((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        if negative {
            i -= 1;
            unsafe { ptr.add(i).write(b'-') }
        }

        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), $len - i)) }
    }};
}

macro_rules! ixx_trait_impl {
    ($itype: ty, $utype: ty, $len:expr) => {
        impl uDisplay for $itype {
            fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                fmt.write_str(ixx!($utype, *self, $len))
            }
        }

        impl uDisplayPadded for $itype {
            fn fmt_padded<W>(
                &self,
                fmt: &mut Formatter<'_, W>,
                padding: Padding,
                pad_char: char,
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                let s = ixx!($utype, *self, $len);
                fmt.write_padded(s, pad_char, padding)
            }
        }
    };
}

ixx_trait_impl!(i8, u8, 4);
ixx_trait_impl!(i16, u16, 6);
ixx_trait_impl!(i32, u32, 11);
ixx_trait_impl!(i64, u64, 21);
ixx_trait_impl!(i128, u128, 40);

#[cfg(target_pointer_width = "16")]
ixx_trait_impl!(isize, usize, 6);
#[cfg(target_pointer_width = "32")]
ixx_trait_impl!(isize, usize, 11);
#[cfg(target_pointer_width = "64")]
ixx_trait_impl!(isize, usize, 21);

udisplay_as_udebug!(i8);
udisplay_as_udebug!(i16);
udisplay_as_udebug!(i32);
udisplay_as_udebug!(i64);
udisplay_as_udebug!(i128);
udisplay_as_udebug!(isize);


// Extend the Convert struct
impl<const CAP: usize> Convert<CAP> {
    /// Converts a u32 number into a string
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::u32(4711).unwrap();
    ///     assert_eq!("4711", conv.as_str());
    /// ```
    pub fn u32(u: u32) -> Result<Self, ()> {
        // SAFETY: The data provided by this function is at the end definitly initialized
        let mut fbuf = unsafe { Self::uninit() };
        fbuf.format_u32(u)?;
        Ok(fbuf)
    }

    /// Converts a u32 number into a padded string with the specified len
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::u32_pad(4711, 6, '0').unwrap();
    ///     assert_eq!("004711", conv.as_str());
    /// ```
    pub fn u32_pad(u: u32, len: usize, pad_char: char) -> Result<Self, ()> {
        if pad_char as u32 >= 0x80 || len > CAP {
            return Err(());
        }
        let buf = [pad_char as u8; CAP];
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.format_u32(u)?;
        fbuf.idx = CAP - len;
        Ok(fbuf)
    }

    fn format_u32(&mut self, mut u: u32) -> Result<(), ()> {
        loop {
            self.write_char((u % 10) as u8 + b'0')?;
            u /= 10;

            if u == 0 {
                break;
            }
        }
        Ok(())
    }

    /// Converts a i32 number into a string
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::i32(-4711).unwrap();
    ///     assert_eq!("-4711", conv.as_str());
    /// ```
    pub fn i32(i: i32) -> Result<Self, ()> {
        // SAFETY: The data provided by this function is at the end definitly initialized
        let mut fbuf = unsafe { Self::uninit() };
        fbuf.format_i32(i)?;
        Ok(fbuf)
    }

    /// Converts a i32 number into a padded string with the specified len
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::i32_pad(-4711, 6, ' ').unwrap();
    ///     assert_eq!(" -4711", conv.as_str());
    /// ```
    pub fn i32_pad(i: i32, len: usize, pad_char: char) -> Result<Self, ()> {
        if pad_char as u32 >= 0x80 || len > CAP {
            return Err(());
        }
        let buf = [pad_char as u8; CAP];
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.format_i32(i)?;
        fbuf.idx = CAP - len;
        Ok(fbuf)
    }

    fn format_i32(&mut self, i: i32) -> Result<(), ()> {
        let negative = i.is_negative();
        let u = if negative {
            match i.checked_abs() {
                Some(i) => i as u32,
                None => <u32>::max_value() / 2 + 1,
            }
        } else {
            i as u32
        };
        self.format_u32(u)?;
        if negative {
            self.write_char(b'-')?;
        }
        Ok(())
    }
}

