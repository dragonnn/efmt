mod macros;

#[test]
fn uxx_min_max() {
    cmp!("{}", u8::max_value());
    cmp!("{:?}", u16::max_value());
    cmp!("{}", u32::max_value());
    cmp!("{}", u64::max_value());
    cmp!("{}", u128::max_value());
    cmp!("{}", usize::max_value());

    cmp!("{}", 0_u8);
    cmp!("{}", 0_u16);
    cmp!("{}", 0_u32);
    cmp!("{}", 0_u64);
    cmp!("{}", 0_u128);
    cmp!("{}", 0_usize);
}

#[test]
fn uxx_aligned() {
    // extreme values
    cmp!("{:10}", 235_u8);
    cmp!("{:10}", 235_u16);
    cmp!("{:10}", 235_u32);
    cmp!("{:10}", 235_u64);
    cmp!("{:10}", 235_u128);
    cmp!("{:10}", 235_usize);

    cmp!("{:<10}", 235_u8);
    cmp!("{:<10}", 235_u16);
    cmp!("{:<10}", 235_u32);
    cmp!("{:<10}", 235_u64);
    cmp!("{:<10}", 235_u128);
    cmp!("{:<10}", 235_usize);

    cmp!("{:>10}", 235_u8);
    cmp!("{:>10}", 235_u16);
    cmp!("{:>10}", 235_u32);
    cmp!("{:>10}", 235_u64);
    cmp!("{:>10}", 235_u128);
    cmp!("{:>10}", 235_usize);

    cmp!("{:0>10}", 235_u8);
    cmp!("{:0<10}", 235_u16);
    cmp!("{:0^10}", 235_u32);
    cmp!("{:0>10}", 235_u64);
    cmp!("{:0<10}", 235_u128);
    cmp!("{:0^10}", 235_usize);
}

#[test]
fn uxx_hex() {
    cmp!("{:10x}", 235_u8);
    cmp!("{:10X}", 235_u16);
    cmp!("{:10x}", 235_u32);
    cmp!("{:10X}", 235_u64);
    cmp!("{:10x}", 235_u128);
    cmp!("{:10X}", 235_usize);

    cmp!("{:#10x}", 235_u8);
    cmp!("{:#10X}", 235_u16);
    cmp!("{:#10x}", 235_u32);
    cmp!("{:#10X}", 235_u64);
    cmp!("{:#10x}", 235_u128);
    cmp!("{:#10X}", 235_usize);

    cmp!("{:x}", u8::max_value());
    cmp!("{:x}", u16::max_value());
    cmp!("{:#x}", u32::max_value());
    cmp!("{:#x}", u64::max_value());
    cmp!("{:x}", u128::max_value());
    cmp!("{:x}", usize::max_value());

    cmp!("{:#x}", 0_u8);
    cmp!("{:#x}", 0_u16);
    cmp!("{:x}", 0_u32);
    cmp!("{:x}", 0_u64);
    cmp!("{:#x}", 0_u128);
    cmp!("{:#x}", 0_usize);
}

#[test]
fn ixx_min_max() {
    cmp!("{}", i8::min_value());
    cmp!("{}", i8::max_value());
    cmp!("{}", i16::min_value());
    cmp!("{}", i16::max_value());
    cmp!("{}", i32::min_value());
    cmp!("{}", i32::max_value());
    cmp!("{}", i64::min_value());
    cmp!("{}", i64::max_value());
    cmp!("{}", i128::min_value());
    cmp!("{}", i128::max_value());
    cmp!("{}", isize::min_value());
    cmp!("{}", isize::max_value());
}

#[test]
fn ixx_aligned() {
    cmp!("{:10}", -115_i8);
    cmp!("{:10}", -115_i16);
    cmp!("{:10}", -115_i32);
    cmp!("{:10}", -115_i64);
    cmp!("{:10}", -115_i128);
    cmp!("{:10}", -115_isize);

    cmp!("{:<10}", -115_i8);
    cmp!("{:<10}", -115_i16);
    cmp!("{:<10}", -115_i32);
    cmp!("{:<10}", -115_i64);
    cmp!("{:<10}", -115_i128);
    cmp!("{:<10}", -115_isize);

    cmp!("{:>10}", -115_i8);
    cmp!("{:>10}", -115_i16);
    cmp!("{:>10}", -115_i32);
    cmp!("{:>10}", -115_i64);
    cmp!("{:>10}", -115_i128);
    cmp!("{:>10}", -115_isize);

    cmp!("{:0>10}", -115_i8);
    cmp!("{:0<10}", -115_i16);
    cmp!("{:0^10}", -115_i32);
    cmp!("{:0>10}", -115_i64);
    cmp!("{:0<10}", -115_i128);
    cmp!("{:0^10}", -115_isize);
}

#[test]
fn ixx_hex() {
    cmp!("{:10x}", 111_i8);
    cmp!("{:10X}", -235_i16);
    cmp!("{:10x}", -235_i32);
    cmp!("{:10X}", 235_i64);
    cmp!("{:10x}", -235_i128);
    cmp!("{:10X}", 235_isize);

    cmp!("{:#10x}", 111_i8);
    cmp!("{:#10X}", -235_i16);
    cmp!("{:#10x}", -235_i32);
    cmp!("{:#10X}", 235_i64);
    cmp!("{:#10x}", -235_i128);
    cmp!("{:#10X}", 235_isize);

    cmp!("{:x}", i8::min_value());
    cmp!("{:x}", i8::max_value());
    cmp!("{:#x}", i16::min_value());
    cmp!("{:#x}", i16::max_value());
    cmp!("{:x}", i32::min_value());
    cmp!("{:x}", i32::max_value());
    cmp!("{:#x}", i64::min_value());
    cmp!("{:#x}", i64::max_value());
    cmp!("{:x}", i128::min_value());
    cmp!("{:x}", i128::max_value());
    cmp!("{:#x}", isize::min_value());
    cmp!("{:#x}", isize::max_value());
}

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
