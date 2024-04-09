# `nopanic`

One of the design goals of `μfmt` is "No panicking branches when optimized" so
here we test that!

Should NOT compile
```
$ cargo build --examples --target thumbv7em-none-eabihf
```

Should compile
```
$ cargo build --examples --target thumbv7em-none-eabihf --release
```
