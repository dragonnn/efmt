mod macros;

use sfmt::uwriteln;

#[test]
fn aligned() {
    cmp!("{:10}", true);
    cmp!("{:<20}", true);
    cmp!("{:>20}", true);
    cmp!("{:>20}", false);
    cmp!("{:>20}", false);

    cmp!("{:10}", 'c');
    cmp!("{:<20}", 'c');
    cmp!("{:>20}", 'c');
    cmp!("{:>20}", 'c');

    cmp!("{:10}", "hello");
    cmp!("{:<20}", "hello");
    cmp!("{:>20}", "hello");
    cmp!("{:>20}", "hello");
}

#[test]
fn fmt() {
    cmp!("Hello, world!");
    cmp!("The answer is {}", 42);
}

#[test]
#[cfg(not(feature = "std"))]
fn uwriteln() {
    let mut s = heapless::String::<100>::new();
    uwriteln!(&mut s, "Hello").unwrap();
    uwriteln!(&mut s, "World",).unwrap();
    assert_eq!(s, "Hello\nWorld\n");
}

#[test]
#[cfg(feature = "std")]
fn uwriteln() {
    let mut s = String::new();
    uwriteln!(&mut s, "Hello").unwrap();
    uwriteln!(&mut s, "World",).unwrap();
    assert_eq!(s, "Hello\nWorld\n");
}
