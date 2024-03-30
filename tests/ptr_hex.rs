mod macros;

#[test]
fn ptr() {
    cmp!("{:?}", 1 as *const u8);
    cmp!("{:?}", 0xf as *const u8);
    cmp!("{:?}", 0xff as *const u8);
    cmp!("{:?}", 0xfff as *const u8);
    cmp!("{:?}", 0xffff as *const u8);
    cmp!("{:?}", 0xfffff as *const u8);
    cmp!("{:?}", 0xffffff as *const u8);
    cmp!("{:?}", 0xfffffff as *const u8);
    cmp!("{:?}", 0xffffffff as *const u8);

    #[cfg(target_pointer_width = "64")]
    cmp!("{:?}", 0xfffffffff as *const u8);
}

#[test]
fn hex() {
    cmp!("{:x}", 771u32);
    cmp!("{:x}", -10000);
    cmp!("{:4x}", 33);
    cmp!("{:4x}", 89001);
    cmp!("{:04x}", 33);
    cmp!("{:#03x}", 33);
    cmp!("{:#09x}", 33);
    cmp!("{:#x}", 71);

    // extreme values
    cmp!("{:x}", i8::min_value());
    cmp!("{:x}", i8::max_value());
    cmp!("{:x}", i16::min_value());
    cmp!("{:x}", i16::max_value());
    cmp!("{:x}", i32::min_value());
    cmp!("{:x}", i32::max_value());
    cmp!("{:x}", i64::min_value());
    cmp!("{:x}", i64::max_value());
    cmp!("{:x}", i128::min_value());
    cmp!("{:x}", i128::max_value());
    cmp!("{:x}", isize::min_value());
    cmp!("{:x}", isize::max_value());

    // <i8 as std::fmt::Display>::fmt(-128)
}

