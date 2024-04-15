A tiny, fast and panic-free alternative to core::fmt

The basis for the development of tfmt is [japaric's ufmt](https://github.com/japaric/ufmt). No 
changes have been made to the original repository for some time. The author also clearly states 
that the display of float numbers and padding is not the focus of the implementation. However, 
these two points in particular can be important when using this crate. 

# Design goals
- Optimised for size and speed for small embedded systems
- Usable during development `Debug`and runtime `Display`
- No panicking branches in generated code when optimised
- It should be easy to integrate additional data types

## Features
- String conversation and formatting options for the following data types included
  - u8, u16, u32, u64, u128, usize
  - i8, i16, i32, i64, i128, isize
  - bool, str, char
  - f32, f64
- [`#[derive(uDebug)]`][macro@derive]
- `uDebug` and `uDisplay` traits like [core::fmt::Debug] and [core::fmt::Display]
- [uDisplayPadded] trait for your own formatted outputs
- [uDisplayFormatted] trait for your own complex formatted outputs
- [uformat] macro to simply generating strings

## Restrictions
`tfmt` offers significantly less functionality than `core::fmt`. For example:
- No named arguments
- No exponential representation of float numbers
- Restricted number range of float numbers (see `tests/float.rs`)
- No binary and octal formatting

# Examples

## Format Standard Rust Types

```rust
    use tfmt::uformat;

    assert_eq!(
        uformat!(100, "The answer to {} is {}", "everything", 42).unwrap().as_str(),
        "The answer to everything is 42" 
    );

    assert_eq!("4711",     uformat!(100, "{}", 4711).unwrap().as_str());
    assert_eq!("00004711", uformat!(100, "{:08}", 4711).unwrap().as_str());
    assert_eq!("   -4711", uformat!(100, "{:8}", -4711).unwrap().as_str());
    assert_eq!("-4711   ", uformat!(100, "{:<8}", -4711).unwrap().as_str());
    assert_eq!("    4711", uformat!(100, "{:>8}", 4711).unwrap().as_str());
    assert_eq!("  4711  ", uformat!(100, "{:^8}", 4711).unwrap().as_str());
    assert_eq!("  4711  ", uformat!(100, "{:^8}", 4711).unwrap().as_str());

    assert_eq!("1ab4",     uformat!(100, "{:x}", 0x1ab4).unwrap().as_str());
    assert_eq!("    1AB4", uformat!(100, "{:8X}", 0x1ab4).unwrap().as_str());
    assert_eq!("0x1ab4",   uformat!(100, "{:#x}", 0x1ab4).unwrap().as_str());

    assert_eq!("3.14",     uformat!(100, "{:.2}", 3.14).unwrap().as_str());
    assert_eq!("    3.14", uformat!(100, "{:8.2}", 3.14).unwrap().as_str());
    assert_eq!("3.14    ", uformat!(100, "{:<8.2}", 3.14).unwrap().as_str());
    assert_eq!("  3.14  ", uformat!(100, "{:^8.2}", 3.14).unwrap().as_str());
    assert_eq!("00003.14", uformat!(100, "{:08.2}", 3.14).unwrap().as_str());

    assert_eq!("hello",    uformat!(100, "{}", "hello").unwrap().as_str());
    assert_eq!("  true  ", uformat!(100, "{:^8}", true).unwrap().as_str());
    assert_eq!("c       ", uformat!(100, "{:<8}", 'c').unwrap().as_str());
```

## Performance

The following table shows a comparison of `tfmt` with `core::fmt` using a few examples. tfmt is 
significantly smaller and also much faster than `core::fmt`. Another difference is that
`tfmt` does not contain a panicking branch. This can be an important difference for embedded systems.

| Name                 | Crate |         Size |   Cycles_min |   Cycles_max |
|----------------------|-------|--------------|--------------|--------------|
| u32                  |  tfmt |          408 |           34 |          277 |
| u32                  |   fmt |          584 |          166 |          428 |
| u32 padded           |  tfmt |          496 |          284 |          406 |
| u32 padded           |   fmt |          940 |          770 |         1019 |
| u32-hex              |  tfmt |          128 |          125 |          237 |
| u32-hex              |   fmt |          948 |          422 |          563 |
| u8 u16 u32           |  tfmt |          708 |          118 |          512 |
| u8 u16 u32           |   fmt |          940 |          770 |         1019 |
| f32                  |  tfmt |          720 |          189 |          196 |
| f32                  |   fmt |        23420 |         1049 |         4799 |

The contents of the table are shown graphically below. The sources for generating the data and 
the visualisation can be found in `tests/size`

![Size comparisation](https://github.com/Simsys/tfmt/blob/main/tests/size/performance.png?raw=true)

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.