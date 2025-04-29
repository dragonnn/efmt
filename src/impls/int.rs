use crate::{
    uDebug, uDisplay, uDisplayHex, uDisplayPadded, uWrite, udisplay_as_udebug, Convert, Formatter,
    Padding,
};
use core::{slice, str};

macro_rules! uxx {
    ($n:expr, $len:expr) => {{
        let mut buf = [0_u8; $len];
        let ptr = &buf.as_mut_ptr().cast::<u8>();
        let mut n = $n;
        let mut i = $len - 1;
        loop {
            // SAFETY: Since i >= 0 and below CAP, this access is secure. This construct is
            // necessary because rust array accesses generate an undesired panicking branch.
            unsafe { ptr.add(i).write_volatile((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        // SAFETY: We only return characters here that we have previously initialised. This is
        // therefore safe and a new check for utf8 conformity is pointless.
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
                fmt.write_str(uxx!(*self as $utype, $len))
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
                let s = uxx!(*self as $utype, $len);
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
        let mut buf = [88_u8; $len];
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
            // SAFETY: Since i >= 0 and below CAP, this access is secure. This construct is
            // necessary because rust array accesses generate an undesired panicking branch.
            unsafe { ptr.add(i).write_volatile((n % 10) as u8 + b'0') }
            n /= 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        if negative {
            i -= 1;
            // SAFETY: Since i >= 0 and below CAP, this access is secure. This construct is
            // necessary because rust array accesses generate an undesired panicking branch.
            unsafe { ptr.add(i).write_volatile(b'-') }
        }

        // SAFETY: We only return characters here that we have previously initialised. This is
        // therefore safe and a new check for utf8 conformity is pointless.
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

const HEX_BUF_LEN: usize = 35;

macro_rules! hex_oct_bin {
    ($utype: ty, $n:expr, $upper: expr, $prefix: expr, $div: expr, $len: expr) => {{
        let mut buf = [0_u8; HEX_BUF_LEN];
        let ptr = &buf.as_mut_ptr().cast::<u8>();
        let mut n = $n;
        let c = if $upper { b'A' } else { b'a' };
        let mut i = HEX_BUF_LEN - 1;
        let mut len = $len;
        loop {
            let val = (n % $div) as u8;
            let d = if val < 10 { b'0' + val } else { c + val - 10 };

            if i == 2 {
                buf[HEX_BUF_LEN - 5..HEX_BUF_LEN].copy_from_slice(b" ovfl");
                i = HEX_BUF_LEN - 5;
                break;
            } else {
                // SAFETY: Since i >= 0 and below CAP, this access is secure. This construct is
                // necessary because rust array accesses generate an undesired panicking branch.
                unsafe { ptr.add(i).write_volatile(d) }
            }

            n /= $div;
            if n == 0 && len == 0 {
                break;
            } else {
                i -= 1;
                if len > 0 {
                    len -= 1;
                }
            }
        }
        if let Some(c) = $prefix {
            i -= 1;
            // SAFETY: Since i >= 0 and below CAP, this access is secure. This construct is
            // necessary because rust array accesses generate an undesired panicking branch.
            unsafe { ptr.add(i).write_volatile(c) }
            i -= 1;
            // SAFETY: Since i >= 0 and below CAP, this access is secure. This construct is
            // necessary because rust array accesses generate an undesired panicking branch.
            unsafe { ptr.add(i).write_volatile(b'0') }
        }

        // SAFETY: We only return characters here that we have previously initialised. This is
        // therefore safe and a new check for utf8 conformity is pointless.
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(ptr.add(i), HEX_BUF_LEN - i)) }
    }};
}

macro_rules! hex_trait_impl {
    ($type: ty, $u_type: ty) => {
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
                let (c, div) = match cmd {
                    'o' => (b'o', 8),
                    'b' => (b'b', 2),
                    _ => (b'x', 16), // 'x', 'X'
                };
                let pre_char = if prefix { Some(c) } else { None };
                let len = if pad_char == '0' {
                    let len = match padding {
                        Padding::LeftAligned(l) => l,
                        Padding::RightAligned(l) => l,
                        Padding::CenterAligned(l) => l,
                        Padding::Usual(l) => l,
                    };
                    if prefix {
                        len - 3
                    } else {
                        len - 1
                    }
                } else {
                    0
                };
                let s = hex_oct_bin!($u_type, *self as $u_type, cmd == 'X', pre_char, div, len);
                if len == 0 {
                    fmt.write_padded(s, pad_char, padding)
                } else {
                    fmt.write_str(s)
                }
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
hex_trait_impl!(usize, usize);
#[cfg(target_pointer_width = "32")]
hex_trait_impl!(usize, usize);
#[cfg(target_pointer_width = "64")]
hex_trait_impl!(usize, usize);

hex_trait_impl!(i8, u8);
hex_trait_impl!(i16, u16);
hex_trait_impl!(i32, u32);
hex_trait_impl!(i64, u64);
hex_trait_impl!(i128, u128);

#[cfg(target_pointer_width = "16")]
hex_trait_impl!(isize, usize);
#[cfg(target_pointer_width = "32")]
hex_trait_impl!(isize, usize);
#[cfg(target_pointer_width = "64")]
hex_trait_impl!(isize, usize);

impl<T> uDebug for *const T {
    #[cfg(target_pointer_width = "16")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let s = hex_oct_bin!(u16, *self as u16, false, Some(b'x'), 16, 0);
        f.write_str(s)
    }

    #[cfg(target_pointer_width = "32")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let s = hex_oct_bin!(u32, *self as u32, false, Some(b'x'), 16, 0);
        f.write_str(s)
    }

    #[cfg(target_pointer_width = "64")]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let s = hex_oct_bin!(u64, *self as u64, false, Some(b'x'), 16, 0);
        f.write_str(s)
    }
}

impl<T> uDebug for *mut T {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        (*self as *const T).fmt(f)
    }
}

// Extend the Convert struct
impl<const CAP: usize> Convert<CAP> {
    /// Converts a u32 number into a string
    ///
    /// ```
    ///     use efmt::Convert;
    ///
    ///     let conv = Convert::<20>::from_u32(4711).unwrap();
    ///     assert_eq!("4711", conv.as_str());
    /// ```
    pub fn from_u32(u: u32) -> Result<Self, ()> {
        let buf = [b' '; CAP];
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.u32(u)?;
        Ok(fbuf)
    }

    /// Converts a u32 number into a padded string with the specified len
    ///
    /// ```
    ///     use efmt::Convert;
    ///
    ///     let mut conv = Convert::<20>::new(b'0');
    ///     conv.u32_pad(4711, 6).unwrap();
    ///     assert_eq!("004711", conv.as_str());
    /// ```
    pub fn u32_pad(&mut self, u: u32, len: usize) -> Result<(), ()> {
        if len > self.idx {
            return Err(());
        }
        let next_idx = self.idx - len;
        self.u32(u)?;
        self.idx = next_idx;
        Ok(())
    }

    /// Appends a u32 number to buffer
    pub fn u32(&mut self, mut u: u32) -> Result<(), ()> {
        loop {
            self.write_u8((u % 10) as u8 + b'0')?;
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
    ///     use efmt::Convert;
    ///
    ///     let conv = Convert::<20>::from_i32(-4711).unwrap();
    ///     assert_eq!("-4711", conv.as_str());
    /// ```
    pub fn from_i32(i: i32) -> Result<Self, ()> {
        let buf = [b' '; CAP];
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.i32(i)?;
        Ok(fbuf)
    }

    /// Converts a i32 number into a padded string with the specified len
    ///
    /// ```
    ///     use efmt::Convert;
    ///
    ///     let mut conv = Convert::<20>::new(b' ');
    ///     conv.i32_pad(-4711, 6).unwrap();
    ///     assert_eq!(" -4711", conv.as_str());
    /// ```
    pub fn i32_pad(&mut self, i: i32, len: usize) -> Result<(), ()> {
        if len > CAP {
            return Err(());
        }
        let next_idx = self.idx - len;
        self.i32(i)?;
        self.idx = next_idx;
        Ok(())
    }

    /// Appends a i32 number to buffer
    pub fn i32(&mut self, i: i32) -> Result<(), ()> {
        let negative = i.is_negative();
        let u = if negative {
            match i.checked_abs() {
                Some(i) => i as u32,
                None => <u32>::max_value() / 2 + 1,
            }
        } else {
            i as u32
        };
        self.u32(u)?;
        if negative {
            self.write_u8(b'-')?;
        }
        Ok(())
    }
}
