// This example shows the use of the uDisplayFormatted trait. As a result, the data point can be
// used flexibly with the choice of leading zeros and the number of decimal places.

use std::f64::consts::PI;
use tfmt::{uDisplayFormatted, uformat, Convert};

struct Coord(f64);

impl uDisplayFormatted for Coord {
    fn fmt_formatted<W>(
        &self,
        fmt: &mut tfmt::Formatter<'_, W>,
        _prefix: bool,
        cmd: char,
        padding: tfmt::Padding,
        pad_char: char,
        decimal_places: usize,
    ) -> Result<(), W::Error>
    where
        W: tfmt::uWrite + ?Sized,
    {
        let (sign, rad) = match cmd {
            'E' => {
                if (*self).0.is_sign_positive() {
                    ("E", (*self).0)
                } else {
                    ("W", -(*self).0)
                }
            }
            _ => {
                if (*self).0.is_sign_positive() {
                    ("N", (*self).0)
                } else {
                    ("S", -(*self).0)
                }
            }
        };

        let degs = rad * 180.0 / PI;
        let mins = degs.fract() * 60.0;

        let len = if decimal_places > 0 {
            decimal_places + 3
        } else {
            decimal_places + 2
        };

        let conv = Convert::<9>::f64_pad(mins, len, decimal_places, '0').unwrap();
        let s = uformat!(15, "{}{},{}", degs as u32, conv.as_str(), sign).unwrap();
        fmt.write_padded(s.as_str(), pad_char, padding)
    }
}

fn main() {
    let lat_berlin = Coord(0.9180516165333352);
    let lon_berlin = Coord(0.23304198843966833);

    // format for coord is dddmm
    let s = uformat!(100, "{:N0},{:E0}", lat_berlin, lon_berlin).unwrap();
    assert_eq!("5236,N,1321,E", s.as_str());

    // format for coord is dddmm.mmm
    let s = uformat!(100, "{:N3},{:E3}", lat_berlin, lon_berlin).unwrap();
    assert_eq!("5236.029,N,1321.139,E", s.as_str());

    // format for coord is dddmm.mmmmmm
    let s = uformat!(100, "{:013N6},{:014E6}", lat_berlin, lon_berlin).unwrap();
    assert_eq!("5236.028980,N,01321.139343,E", s.as_str());
}
