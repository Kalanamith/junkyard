use std::io::Write;
use std::collections::HashSet;
mod macros {

    #[macro_export]
    macro_rules! compare {
        ($x:literal => $y:block) => {
            $x == $y
        };
    }
    #[macro_export]
    macro_rules! strange_patterns {
        (The pattern must match precisely) => {
            "Text"
        };
        (42) => {
            "Numeric"
        };
        (;<=,<=;) => {
            "Alpha"
        }
    }

    #[macro_export]
    macro_rules! one_plus_one {
        () => {
            1 + 1
        };
    }

    #[macro_export]
    macro_rules! one_and_one {
        (plus) => {
            1 + 1
        };
        (minus) => {
            1 - 1
        };
        (mult) => {
            1 * 1
        };
    }
    #[macro_export]
    macro_rules! web {
        (GET $path:literal => $b:block) => {
            register_handler("GET", $path, &|| $b)
        };
        (POST $path:literal => $b:block) => {
            register_handler("POST", $path, &|| $b)
        };
    }

    // Repeats the statement that was passed in n times
    #[macro_export]
    // `()` indicates that the macro takes no argument
    // help: valid fragment specifiers are `ident`, `block`, `stmt`, `expr`,
    //  `pat`, `ty`, `lifetime`, `literal`, `path`, `meta`, `tt`, `item` and `vis`rustc
    macro_rules! n_times {
        ($n: expr, $f: block) => {
            for _ in 0..$n {
                $f()
            }
        };
    }

    // Declare a function in a macro
    #[macro_export]
    macro_rules! make_fn {
        ($i: ident, $body: block) => {
            fn $i () $body
        };
    }

    #[macro_export]
    macro_rules! default_enum {
        ($name: ident, $(variant: ident => $val:expr), +) => {
            #[derive(Eq, PartialEq, Clone, Debug)]
            pub enum $name {
                Invalid,
                $($variant = $val), +
            }

            impl Default for $name {
                fn default() -> Self { $name::Invalid  }
            }
        };
    }

    #[macro_export]
    macro_rules! print_debug {
        (stdout, $($o:expr), *) => {
            $(print!("{:?}", $o));*;
            println!();
        };
        (error, $($o:expr), *) => {
            $(eprint!("{:?}", $o));*;
            eprintln!();
        };
        ($stream:expr, $($o:expr),*) => {
            $(let _ = write!($stream, "{:?}", $o));*;
            let _ = writeln!($stream);
        }
    }

    #[macro_export]
    macro_rules! mock {
        ($type: ty, $name: ident, $ret_val: ty, $val: block) => {
            pub trait $name {
                fn $name(&self) -> $ret_val;
                }
            impl $name for $type {
                fn $name(&self) -> $ret_val $val
            }
        };
        ($name: ident, $($variant: ident => $type:ty), +) => {
            #[derive(PartialEq, Clone, Debug)]
            struct $name {
                $(pub $variant: $type), +
            }
        };
    }

    #[macro_export]
    macro_rules! set {
        ($($item:expr), *) => {
            {
                let mut s = HashSet::new();
                $(
                    s.insert($item);
                )*
                s
            }
            
        };
    }

    // Creates a DTO
    #[macro_export]
    macro_rules! dto {
        ($name:ident, $($variant:ident => $type:ty), +) => {
            
            #[derive(Clone, Debug, PartialEq)]
            pub struct $name {
                $(pub $variant: $type),+
            }

            impl $name {
                pub fn new($($variant:$type),+) -> Self {
                $name {
                    $($variant: $variant),+
                }
            }
        }
        };
    }

}

#[derive(Debug)]
pub struct Response(usize);
pub fn register_handler(method: &str, path: &str, handler:
&dyn Fn() -> Response ) {}

#[derive(Debug)]
struct MyStruct(usize);

fn main() {
    println!("Hello, world!");
    println!("1 + 1 = {}", one_plus_one!());
    println!("1 + 1 = {}", one_and_one!(plus));
    println!("1 - 1 = {}", one_and_one!(minus));
    println!("1 * 1 = {}", one_and_one!(mult));

    println!("a vec: {:?} {:?}", vec![1, 2, 3], vec!{1,2,3,4});
    println!("concat: {}", concat!(0, 'x', "5ff"));
    println!("MyStruct stringified: {}", stringify!(MyStruct(100)));
    println!("Some random word stringgified: {}", stringify!(helloworld));

    println!("Running on Windows? {}", cfg!(windows));
    println!("Error from file: {}", include_str!("a.txt"));
    println!("$PATH: {:?}", option_env!("PATH"));
    eprintln!("Oh no!");
    debug_assert!(true);

    let mut i = 0;
    n_times!(5, { i = i + 2 });
    println!("The value of i :- {:?}", i);
}

// Creates a Trait for type String named `tell`
// Returns the type `&'static str`
// Return value of Hi!
mock!(String, tell, &'static str, { "Hi!" });

mock!(u32, speak,  &'static str, { "Number" });

mock!(HelloWorld, greeting => String, when => u64);

#[test]
fn test_dto(){
    dto!(Sensordata, value => f32, timestamp => u64);
    let s = Sensordata::new(1.23f32, 123456);
    assert_eq!(s.value, 1.23f32);
    assert_eq!(s.timestamp, 123456);
}

#[test]
fn test_set(){
    let actual = set!("a", "b", "c", "a");
    let mut desired = HashSet::new();
    desired.insert("a");
    desired.insert("b");
    desired.insert("c");
    assert_eq!(actual, desired);
}
#[test]
fn test_mock() {
    let mystr = "".to_owned();
    assert_eq!(mystr.tell(), "Hi!");

    let mynum = 67.to_owned();
    assert_eq!(mynum.speak(), "Number");

    let g = HelloWorld { greeting: "Hello World".to_owned(),
    when: 1560887098 };
    assert_eq!(g.greeting, "Hello World");
    assert_eq!(g.when, 1560887098);

}

#[test]
fn test_strange_patterns() {
    assert_eq!(strange_patterns!(The pattern must match precisely), "Text");
    assert_eq!(strange_patterns!(42), "Numeric");
    assert_eq!(strange_patterns!(;<=,<=;), "Alpha");
}

#[test]
fn test_compare() {
    assert!(compare!(1 => {1}));
}
#[test]
fn test_web() {
    web!(GET "/" => { Response(2020) });
    web!(POST "/" => { Response(4203) });
}

#[test]
#[should_panic]
fn test_failing_make_fn() {
    make_fn!(fail, {assert!(false)});
    fail();
}

#[test]
fn test_make_fn() {
    make_fn!(fail, {assert!(false)});
}


#[test]
fn test_printer() {
    print_debug!(error, "hello std err");
    print_debug!(stdout, "hello std out");
    let mut v = vec![];
    print_debug!(&mut v, "a");
    assert_eq!(v, vec![34, 97, 34, 10]);
}
