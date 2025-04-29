#[macro_export]
macro_rules! cmp {
    ($($tt:tt)*) => {
        assert_eq!(
            efmt::uformat!(500, $($tt)*).unwrap().as_str(),
            format!($($tt)*).as_str(),
        )
    }
}

#[macro_export]
//#[cfg(not(feature = "std"))]
macro_rules! cmp_str {
    ($s: expr, $($tt:tt)*) => {
        assert_eq!(
            efmt::uformat!(500, $($tt)*).unwrap().as_str(),
            $s,
        )
    }
}
