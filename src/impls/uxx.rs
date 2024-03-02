use core::{mem::MaybeUninit, slice, str};

use crate::{uDebug, uDisplay, uWrite, Formatter, uDisplayWithPadding, Format};

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

        impl uDebug for $utype {
            fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized,
            {
                <$utype as uDisplay>::fmt(self, fmt)
            }
        }
        
        impl uDisplayWithPadding for $utype {
            fn fmt_padding<W>(
                &self, 
                fmt: &mut Formatter<'_, W>, 
                format: Format
            ) -> Result<(), W::Error>
            where
                W: uWrite + ?Sized 
            {
                let s = uxx!(*self, $len);
                match format {
                    Format::LeftAligned(pad_length) => {
                        fmt.write_str(s)?;
                        for _ in s.len() .. pad_length {
                            fmt.write_char(' ')?;
                        }
                        Ok(())
                    }
                    Format::Padded(pad_length) | Format::RightAligned(pad_length) => {
                        for _ in s.len() .. pad_length {
                            fmt.write_char(' ')?;
                        }
                        fmt.write_str(s)
                    }
                }
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
