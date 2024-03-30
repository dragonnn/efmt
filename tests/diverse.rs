mod macros;
use core::convert::Infallible;

use ufmt::{derive::uDebug, uDebug, uWrite, uwrite, uwriteln, Formatter};


#[test]
fn aligned() {
    cmp!("{:10}", true);
    cmp!("{:<20}", true);
    cmp!("{:>20}", true);
    cmp!("{:>20}", false);
    cmp!("{:0>20}", false);

    cmp!("{:10}", 'c');
    cmp!("{:<20}", 'c');
    cmp!("{:>20}", 'c');
    cmp!("{:0>20}", 'c');

    cmp!("{:10}", "hello");
    cmp!("{:<20}", "hello");
    cmp!("{:>20}", "hello");
    cmp!("{:0>20}", "hello");
}

#[test]
fn fmt() {
    cmp!("Hello, world!");
    cmp!("The answer is {}", 42);
}


#[test]
fn uwriteln() {
    let mut s = String::new();
    uwriteln!(&mut s, "Hello").unwrap();
    uwriteln!(&mut s, "World",).unwrap();
    assert_eq!(s, "Hello\nWorld\n");
}

#[test]
fn formatter_uwrite() {
    #[derive(uDebug)]
    struct X;

    struct Y;

    impl uDebug for Y {
        fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
        where
            W: uWrite + ?Sized,
        {
            uwrite!(f, "{:?}", X)
        }
    }

    assert_eq!(uformat!("{:?}", Y).unwrap(), "X")
}

#[test]
fn generic() {
    #[derive(uDebug, Debug)]
    struct X<T>(T);

    cmp!("{:?}", X(0));

    #[derive(uDebug, Debug)]
    enum Y<T> {
        Z(T),
    }

    cmp!("{:?}", Y::Z(0));
}

// compile-pass test
#[allow(dead_code)]
fn static_lifetime(x: &'static mut u32) {
    fn foo(x: &'static mut u32) -> *mut u32 {
        x as *mut u32
    }

    uwrite!(&mut String::new(), "{:?}", foo(x)).ok();
}

// test dynamically sized writer
#[test]
fn dst() {
    struct Cursor<B>
    where
        B: ?Sized,
    {
        pos: usize,
        buffer: B,
    }

    impl<B> Cursor<B> {
        fn new(buffer: B) -> Self {
            Cursor { pos: 0, buffer }
        }
    }

    impl uWrite for Cursor<[u8]> {
        type Error = Infallible;

        fn write_str(&mut self, s: &str) -> Result<(), Infallible> {
            let bytes = s.as_bytes();
            let len = bytes.len();
            let start = self.pos;
            if let Some(buffer) = self.buffer.get_mut(start..start + len) {
                buffer.copy_from_slice(bytes);
                self.pos += len;
            }

            Ok(())
        }
    }

    let mut cursor = Cursor::new([0; 256]);
    let cursor: &mut Cursor<[u8]> = &mut cursor;

    uwrite!(cursor, "The answer is {}", 42).ok();

    let msg = b"The answer is 42";
    assert_eq!(&cursor.buffer[..msg.len()], msg);
}
