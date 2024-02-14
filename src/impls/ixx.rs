use core::{mem::MaybeUninit, slice, str};

use crate::{uDebug, uDisplay, uWrite, Formatter, uDisplayWithPadding};

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

        impl uDebug for $itype {
            fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                <$itype as uDisplay>::fmt(self, fmt)
            }
        }
        
        impl uDisplayWithPadding for $itype {
            fn fmt_padding<W>(
                &self, 
                fmt: &mut Formatter<'_, W>, 
                pad_length: usize, 
                left_aligned: bool
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized 
            {
                let s = ixx!($utype, *self, $len);
                if left_aligned {
                    fmt.write_str(s)?;
                    for _ in s.len() .. pad_length {
                        fmt.write_char(' ')?;
                    }
                    Ok(())
                } else {
                    for _ in s.len() .. pad_length {
                        fmt.write_char(' ')?;
                    }
                    fmt.write_str(s)
                }
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

