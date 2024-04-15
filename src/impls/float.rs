use crate::{uDebug, udisplay_as_udebug, uDisplay, uDisplayFloat, uWrite, Convert, Formatter, Padding};

// max 2**32 4_294_967_296 (10 digits) + 6 digits right dp + '.' + '-' => 18 digits max
const BUF_LEN: usize = 18;

impl uDisplayFloat for f32 {
    fn fmt_float<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
        behind: usize,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let convert = Convert::<BUF_LEN>::f32(*self, behind);
        if let Ok(convert) = convert {
            fmt.write_padded(convert.as_str(), pad_char, padding)?;
        }
        Ok(()) // Silently ignore errors during formatting
    }
}

impl uDisplay for f32 {
    fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let convert = Convert::<BUF_LEN>::f32(*self, 3);
        if let Ok(convert) = convert {
            fmt.write_str(convert.as_str())?;
        }
        Ok(()) // Silently ignore errors during formatting
    }
}

udisplay_as_udebug!(f32);

impl uDisplayFloat for f64 {
    fn fmt_float<W>(
        &self,
        fmt: &mut Formatter<'_, W>,
        padding: Padding,
        pad_char: char,
        behind: usize,
    ) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let convert = Convert::<BUF_LEN>::f64(*self, behind);
        if let Ok(convert) = convert {
            fmt.write_padded(convert.as_str(), pad_char, padding)?;
        }
        Ok(()) // Silently ignore errors during formatting
    }
}

impl uDisplay for f64 {
    fn fmt<W>(&self, fmt: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        let convert = Convert::<BUF_LEN>::f64(*self, 3);
        if let Ok(convert) = convert {
            fmt.write_str(convert.as_str())?;
        }
        Ok(()) // Silently ignore errors during formatting
    }
}

udisplay_as_udebug!(f64);

impl<const CAP: usize> Convert<CAP> {
    /// Converts a f32 number into a string with the specified precision
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::f32(3.14159265359, 4).unwrap();
    ///     assert_eq!("3.1416", conv.as_str());
    /// ```
    pub fn f32(f: f32, decimal_places: usize) -> Result<Self, ()> {
        let buf = core::mem::MaybeUninit::<[u8; CAP]>::uninit();
        // SAFETY: This routine returns only the part of string, which is initiliased, so this is save
        //         Not initialising the buffer saves approx. 90 cycles
        let buf = unsafe { buf.assume_init() };
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.format_f32(f, decimal_places)?;
        Ok(fbuf)
    }

    /// Converts a f64 number into a string with the specified precision
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::f64(3.14159265359, 4).unwrap();
    ///     assert_eq!("3.1416", conv.as_str());
    /// ```
    pub fn f64(f: f64, decimal_places: usize) -> Result<Self, ()> {
        let buf = core::mem::MaybeUninit::<[u8; CAP]>::uninit();
        // SAFETY: This routine returns only the part of string, which is initiliased, so this is save
        //         Not initialising the buffer saves approx. 90 cycles
        let buf = unsafe { buf.assume_init() };
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.format_f64(f, decimal_places)?;
        Ok(fbuf)
    }

    /// Converts a f32 number into a string of length len with the specified precision
    ///
    /// The digits not occupied by the number are pre-assigned with `pad_char`. The representation
    /// is aligned to the right.
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::f32_pad(3.14159265359, 7, 3, '0').unwrap();
    ///     assert_eq!("003.142", conv.as_str());
    /// ```
    pub fn f32_pad(f: f32, len: usize, decimal_places: usize, pad_char: char) -> Result<Self, ()> {
        if pad_char as u32 >= 0x80 || len > CAP {
            return Err(());
        }
        let buf = [pad_char as u8; CAP];
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.format_f32(f, decimal_places)?;
        fbuf.idx = CAP - len;
        Ok(fbuf)
    }

    /// Converts a f64 number into a string of length len with the specified precision
    ///
    /// The digits not occupied by the number are pre-assigned with `pad_char`. The representation
    /// is aligned to the right.
    ///
    /// ```
    ///     use tfmt::Convert;
    ///
    ///     let conv = Convert::<20>::f64_pad(3.14159265359, 7, 3, '0').unwrap();
    ///     assert_eq!("003.142", conv.as_str());
    /// ```
    pub fn f64_pad(f: f64, len: usize, decimal_places: usize, pad_char: char) -> Result<Self, ()> {
        if pad_char as u32 >= 0x80 || len > CAP {
            return Err(());
        }
        let buf = [pad_char as u8; CAP];
        let mut fbuf = Convert { buf, idx: CAP };
        fbuf.format_f64(f, decimal_places)?;
        fbuf.idx = CAP - len;
        Ok(fbuf)
    }

    fn format_f32(&mut self, f: f32, decimal_places: usize) -> Result<(), ()> {
        // General checks for validity and overflow
        if f.is_nan() {
            self.write_str("NaN")?;
            return Ok(());
        }

        if f > 8388608.0 {
            // 2**23
            self.write_str("ovfl")?;
            return Ok(());
        }

        if f < -8388608.0 {
            // 2**23
            self.write_str("-ovfl")?;
            return Ok(());
        }

        if decimal_places > 6 {
            return Err(());
        }

        const MUL_TAB: [f32; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];
        const ADD_TAB: [f32; 7] = [0.5, 0.05, 0.005, 0.000_5, 0.000_05, 0.000_005, 0.000_000_5];

        let (f, is_neg) = if f.is_sign_negative() {
            ((-f) + ADD_TAB[decimal_places as usize], true)
        } else {
            (f + ADD_TAB[decimal_places as usize], false)
        };

        let left = f as u32;
        let right = ((f - (left as f32)) * MUL_TAB[decimal_places as usize]) as u32;

        self.float_as_str(left, right, decimal_places, is_neg)
    }

    fn format_f64(&mut self, f: f64, decimal_places: usize) -> Result<(), ()> {
        // General checks for validity and overflow
        if f.is_nan() {
            self.write_str("NaN")?;
            return Ok(());
        }

        if f > 4_294_967_295.0 {
            // u32::MAX
            self.write_str("ovfl")?;
            return Ok(());
        }

        if f < -4_294_967_295.0 {
            // u32::MAX
            self.write_str("-ovfl")?;
            return Ok(());
        }

        if decimal_places > 6 {
            return Err(());
        }

        const MUL_TAB: [f64; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];
        const ADD_TAB: [f64; 7] = [0.5, 0.05, 0.005, 0.000_5, 0.000_05, 0.000_005, 0.000_000_5];

        let (f, is_neg) = if f.is_sign_negative() {
            ((-f) + ADD_TAB[decimal_places as usize], true)
        } else {
            (f + ADD_TAB[decimal_places as usize], false)
        };

        let left = f as u32;
        let right = ((f - (left as f64)) * MUL_TAB[decimal_places as usize]) as u32;

        self.float_as_str(left, right, decimal_places, is_neg)
    }

    // Internally, the floating point number is displayed as two integers, whereby the location of
    // the decimal point is shown separately.
    //
    // This routine then writes the floating point number correctly in the formatter
    fn float_as_str(
        &mut self,
        mut left: u32,
        mut right: u32,
        decimal_places: usize,
        is_neg: bool,
    ) -> Result<(), ()> {
        let dp_idx = if decimal_places == 0 {
            None
        } else {
            Some(CAP - decimal_places as usize)
        };

        // Safety: This is necessary to avoid getting a panic branch
        // The algorithm ensures that the buf array range limits are not exceeded

        // write digits to the right of the dp
        if let Some(dp_idx) = dp_idx {
            while self.idx > dp_idx {
                let m = (right % 10) as u8;
                right = right / 10;
                self.write_char(m + b'0')?;
            }
            self.write_char(b'.')?;
        }

        // write digits to the left of the dp
        if left == 0 {
            self.write_char(b'0')?;
        } else {
            while left > 0 {
                let m = (left % 10) as u8;
                left = left / 10;
                self.write_char(m + b'0')?;
            }
        }

        // Add negativ sign if necessary
        if is_neg {
            self.write_char(b'-')?;
        }
        Ok(())
    }
}
