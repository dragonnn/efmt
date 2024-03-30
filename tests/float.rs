mod macros;

#[test]
fn f32() {
    cmp!("{:10.3}", 3.14_f32);

    cmp!("{:20.2}", 3.14_f32);
    cmp!("{:<20.2}", 3.14_f32);
    cmp!("{:>20.2}", 3.14_f32);
    cmp!("{:^20.2}", 3.14_f32);
    cmp!("{:0^20.2}", 3.14_f32);

    cmp!("{:.0}", 1.1_f32);
    cmp!("{:.0}", 0.7_f32);
    cmp!("{:.0}", 0.0_f32);
    cmp!("{:.0}", -0.7_f32);
    cmp!("{:.0}", -1.1_f32);
    cmp!("{:.1}", 1.1_f32);
    cmp!("{:.1}", 0.7_f32);
    cmp!("{:.1}", 0.0_f32);
    cmp!("{:.1}", -0.7_f32);
    cmp!("{:.1}", -1.1_f32);
    cmp!("{:.3}", 1.10234_f32);
    cmp!("{:.3}", 0.70555_f32);
    cmp!("{:.3}", 0.0_f32);
    cmp!("{:.3}", -0.70234_f32);
    cmp!("{:.3}", -1.10555_f32);
    cmp!("{:.5}", 1.10234_f32);
    cmp!("{:.5}", 0.70555_f32);
    cmp!("{:.5}", 0.0_f32);
    cmp!("{:.5}", -0.70234_f32);
    cmp!("{:.5}", -1.10555_f32);
    cmp!("{:.6}", -0.702341234_f32);
    cmp!("{:.6}", -1.105554321_f32);

    const F_MAX32: f32 = 8388608.0; // 2**23
    cmp!("{:.0}", F_MAX32 - 1.0_f32);
    cmp!("{:.0}", F_MAX32);
    cmp!("{:.0}", -F_MAX32);
    cmp!("{:.0}", -F_MAX32 + 1.0_f32);
    cmp!("{:.3}", f32::NAN);
    cmp!("{:.3}", f32::EPSILON);
    cmp!("{:.3}", f32::MIN_POSITIVE);
    cmp!("{:.6}", 8388607.1234567_f32);

    assert_eq!(
        uformat!("{:.3}", F_MAX32 + 1.0_f32),
        Ok(String::from("ovfl")) // std::format "8388609.000"
    );
    assert_eq!(
        uformat!("{:.3}", -F_MAX32 - 1.0_f32),
        Ok(String::from("-ovfl")) // std::format "-8388609.000"
    );
    assert_eq!(
        uformat!("{:.3}", f32::INFINITY),
        Ok(String::from("ovfl")) // std::format "inf"
    );
    assert_eq!(
        uformat!("{:.3}", f32::NEG_INFINITY),
        Ok(String::from("-ovfl")) // std::format "-inf"
    );
    assert_eq!(
        uformat!("{}", 321.123456_f32),
        Ok(String::from("321.123")) // std::format "321.12344"
    );
    assert_eq!(
        uformat!("{}", 321.0_f32),
        Ok(String::from("321.000")) // std::format "321"
    );
}

#[test]
fn f64() {
    cmp!("{:10.3}", 3.14_f64);

    cmp!("{:20.2}", 3.14_f64);
    cmp!("{:<20.2}", 3.14_f64);
    cmp!("{:>20.2}", 3.14_f64);
    cmp!("{:^20.2}", 3.14_f64);
    cmp!("{:0^20.2}", 3.14_f64);

    cmp!("{:.0}", 1.1_f64);
    cmp!("{:.0}", 0.7_f64);
    cmp!("{:.0}", 0.0_f64);
    cmp!("{:.0}", -0.7_f64);
    cmp!("{:.0}", -1.1_f64);
    cmp!("{:.1}", 1.1_f64);
    cmp!("{:.1}", 0.7_f64);
    cmp!("{:.1}", 0.0_f64);
    cmp!("{:.1}", -0.7_f64);
    cmp!("{:.1}", -1.1_f64);
    cmp!("{:.3}", 1.10234_f64);
    cmp!("{:.3}", 0.70555_f64);
    cmp!("{:.3}", 0.0_f64);
    cmp!("{:.3}", -0.70234_f64);
    cmp!("{:.3}", -1.10555_f64);
    cmp!("{:.5}", 1.10234_f64);
    cmp!("{:.5}", 0.70555_f64);
    cmp!("{:.5}", 0.0_f64);
    cmp!("{:.5}", -0.70234_f64);
    cmp!("{:.5}", -1.10555_f64);
    cmp!("{:.6}", -0.702341234_f64);
    cmp!("{:.6}", -1.105554321_f64);
    cmp!("{:.6}", 4_294_967_294.123456_f64);

    const F_MAX64: f64 = 4_294_967_295.0; // 2**32 - 1
    cmp!("{:.0}", F_MAX64 - 1.0_f64);
    cmp!("{:.0}", F_MAX64);
    cmp!("{:.0}", -F_MAX64);
    cmp!("{:.0}", -F_MAX64 + 1.0_f64);
    cmp!("{:.3}", f64::NAN);
    cmp!("{:.3}", f64::EPSILON);
    cmp!("{:.3}", f64::MIN_POSITIVE);

    assert_eq!(
        uformat!("{:.3}", F_MAX64 + 1.0_f64),
        Ok(String::from("ovfl")) // std::format "1844674407371.000"
    );
    assert_eq!(
        uformat!("{:.3}", -F_MAX64 - 1.0_f64),
        Ok(String::from("-ovfl")) // std::format "-1844674407371.000"
    );
    assert_eq!(
        uformat!("{:.3}", f64::INFINITY),
        Ok(String::from("ovfl")) // std::format "inf"
    );
    assert_eq!(
        uformat!("{:.3}", f64::NEG_INFINITY),
        Ok(String::from("-ovfl")) // std::format "-inf"
    );
    assert_eq!(
        uformat!("{}", 321.123456_f64),
        Ok(String::from("321.123")) // std::format "321.123456"
    );
    assert_eq!(
        uformat!("{}", 321.0_f64),
        Ok(String::from("321.000")) // std::format "321"
    );
}
