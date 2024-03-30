#[macro_export]
macro_rules! uformat {
    ($($tt:tt)*) => {{
        let mut s = String::new();
        #[allow(unreachable_code)]
        match ufmt::uwrite!(&mut s, $($tt)*) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }};
}

#[macro_export]
macro_rules! cmp {
    ($($tt:tt)*) => {
        assert_eq!(
            uformat!($($tt)*),
            Ok(format!($($tt)*)),
        )
    }
}
