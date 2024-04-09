A small, fast and panic-free alternative to core::fmt

The basis for the development of sfmt is [japaric's ufmt](https://github.com/japaric/ufmt).

# Design goals
- Optimised for size and speed for small embedded systems
- Usable during development `Debug`and runtime `Display`
- No panicking branches in generated code when optimised

# Features
- String conversation and formatting options for the following data types included
  - u8, u16, u32, u64, u128, usize
  - i8, i16, i32, i64, i128, isize
  - bool, str, char
  - f32, f64
- [`#[derive(uDebug)]`][derive]
- `uDebug` and `uDisplay` traits like [core::fmt::Debug] and [core::fmt::Display]
- [uDisplayPadded] trait for your own formatted outputs
- [uDisplayFormatted] trait for your own complex formatted outputs
- [uformat] macro to simply generating strings

# Formatting Examples

```rust
    use sfmt::uformat;

    assert_eq!(
        "The answer to everything is 42", 
        uformat!(100, "The answer to {} is {}", "everything", 42).unwrap().as_str()
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

## License

All source code (including code snippets) is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [https://www.apache.org/licenses/LICENSE-2.0][L1])

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [https://opensource.org/licenses/MIT][L2])

[L1]: https://www.apache.org/licenses/LICENSE-2.0
[L2]: https://opensource.org/licenses/MIT

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.