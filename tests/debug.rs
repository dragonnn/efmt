mod macros;

use tfmt::{derive::uDebug, uDebug, uWrite, uformat, uwrite, Formatter};

#[test]
fn struct_() {
    #[derive(Debug, uDebug)]
    struct Braces {}

    #[derive(Debug, uDebug)]
    struct Parens();

    #[derive(Debug, Default, uDebug)]
    struct I32(i32);

    #[derive(Debug, Default, uDebug)]
    struct Tuple(i32, i32);

    #[derive(Debug, Default, uDebug)]
    struct Pair {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Default, uDebug)]
    struct Nested {
        first: Pair,
        second: Pair,
    }

    cmp!("{:?}", Braces {});
    cmp!("{:?}", Parens());
    cmp!("{:?}", I32::default());
    cmp!("{:?}", Tuple::default());
    cmp!("{:?}", Pair::default());
    cmp!("{:?}", Nested::default());

    cmp!("{:#?}", Braces {});
    cmp!("{:#?}", Parens());
    cmp!("{:#?}", I32::default());
    cmp!("{:#?}", Tuple::default());
    cmp!("{:#?}", Pair::default());
    cmp!("{:#?}", Nested::default());
}

#[test]
fn enum_() {
    #[derive(Debug, uDebug)]
    enum X {
        A,
        B(u8, u16),
        C { x: u8, y: u16 },
    }

    cmp!("{:?}", X::A);
    cmp!("{:?}", X::B(0, 1));
    cmp!("{:?}", X::C { x: 0, y: 1 });

    cmp!("{:#?}", X::A);
    cmp!("{:#?}", X::B(0, 1));
    cmp!("{:#?}", X::C { x: 0, y: 1 });
}

#[test]
fn tuples() {
    cmp!("{:?}", ());
    cmp!("{:?}", (1,));
    cmp!("{:?}", (1, 2));
    cmp!("{:?}", (1, 2, 3));
    cmp!("{:?}", (1, 2, 3, 4));
    cmp!("{:?}", (1, 2, 3, 4, 5));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
    cmp!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    cmp!("{:#?}", ());
    cmp!("{:#?}", (1,));
    cmp!("{:#?}", (1, 2));
}

#[test]
fn slice() {
    cmp!("{:?}", [0; 0]);
    cmp!("{:?}", [0]);
    cmp!("{:?}", [0, 1]);

    cmp!("{:#?}", [0; 0]);
    cmp!("{:#?}", [0]);
    cmp!("{:#?}", [0, 1]);
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

    assert_eq!(uformat!(100, "{:?}", Y).unwrap().as_str(), "X")
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
