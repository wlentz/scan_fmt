// Copyright 2015 Will Lentz.
// Licensed under the MIT license.

//! This crate provides a simple sscanf()-like interface to extract
//! data from strings and stdin.
//!
//! To use this crate, do:
//!
//! ```ignore
//! #[macro_use] extern crate scan_fmt;
//! ```
//!
//! Example to read from a string:
//!
//! ```rust
//! # #[macro_use] extern crate scan_fmt;
//! # fn main() {
//!   let (a,b,c) = scan_fmt!( "hello 12 345 bye", // input string
//!                            "hello {} {d} {}",  // format
//!                            u8, i32, String);   // type of a-c Options
//!   assert_eq!( a.unwrap(), 12 ) ;
//!   assert_eq!( b.unwrap(), 345 ) ;
//!   assert_eq!( c.unwrap(), "bye" ) ;
//! # }
//! ```
//!
//! Special format_string tokens:
//! <pre class="rust">
//!   {{ = escape for '{'
//!   }} = escape for '}'
//!   {} = return any value (until next whitespace)
//!   {d} = return base-10 decimal
//!   {x} = return hex (0xab or ab)
//!   {f} = return float
//!   {*d} = "*" as the first character means "match but don't return"
//!   {[...]} = return pattern.
//!     ^ inverts if it is the first character
//!     - is for ranges.  For a literal - put it at the start or end.
//!     To add a literal ] do "[]abc]"
//!   Examples:
//!     {[0-9ab]} = match 0-9 or a or b
//!     {[^,.]} = match anything but , or .
//! </pre>
//!
//! Example to read from stdin:
//!
//! ```rust
//! # #[macro_use] extern crate scan_fmt;
//! # fn main() {
//!   let (a,b) = scanln_fmt!( "{}-{}",   // format
//!                            u16, u8);    // type of a&b Options
//!   match (a,b) {
//!     (Some(aa),Some(bb)) => println!("Got {} and {}",aa,bb),
//!     _ => println!("input error")
//!   }
//! # }
//! ```
//!
//! ## LIMITATIONS:
//! There are no compile-time checks to make sure the format
//! strings matches the number of return arguments.  Extra
//! return values will be None.
//!
//! Like sscanf(), whitespace (including \n) is largely ignored.
//!
//! Conversion to output values is done using parse::<T>().

pub mod parse ;

/// (a,+) = scan_fmt!( input_string, format_string, types,+ )
#[macro_export]
macro_rules! scan_fmt {
    ( $instr:expr, $fmt:expr, $arg1:ty, $($arg2:ty),* ) => {
        {
            let mut res = $crate::parse::scan( $instr, $fmt ) ;
            ( match res.next() {
                Some(item) => item.parse::<$arg1>().ok(),
                _ => None
            }
              $( , match res.next() {
                  Some(item) => item.parse::<$arg2>().ok(),
                  _ => None
              }
                   )*
              )
        }
    };
    ( $instr:expr, $fmt:expr, $arg1:ty ) => {
        {
            let mut res = $crate::parse::scan( $instr, $fmt ) ;
            ( match res.next() {
                Some(item) => item.parse::<$arg1>().ok(),
                _ => None
            } )
        }
    };
}

pub fn get_input_unwrap() -> String {
    let mut input = String::new() ;
    std::io::stdin().read_line(&mut input).unwrap() ;
    input
}

/// (a,+) = scanln_fmt!( format_string, types,+ )
/// <p>Same as scan_fmt!(), but reads input string from stdin.</p>
#[macro_export]
macro_rules! scanln_fmt {
    ($($arg:tt)*) => {{ scan_fmt!(&$crate::get_input_unwrap(), $($arg)*) }}
}

macro_rules! assert_flt_eq {
    ($t:ident, $v1:expr, $v2:expr) =>
    {{ assert!( ($v1 - $v2).abs() <= 2.0*std::$t::EPSILON ); }};
}

#[test]
fn test_limited_data_range() {
    let (a,b,c) = scan_fmt!("test{\t 1e9 \n bye 257} hi  22.7e-1",
                            "test{{ {} bye {d}}} hi {f}",
                            f64,u8,f32) ;
    assert_flt_eq!( f64, a.unwrap(), 1e9 );
    assert_eq!( b, None ); // 257 doesn't fit into a u8
    assert_flt_eq!( f32, c.unwrap(), 2.27 );
}

#[test]
fn test_too_many_outputs() {
    let (a,b,c,d) = scan_fmt!("a_aa bb_b c",
                              "{} {s} {}",
                              String, String, String, String) ;
    assert_eq!( a.unwrap(), "a_aa" );
    assert_eq!( b.unwrap(), "bb_b" );
    assert_eq!( c.unwrap(), "c" );
    assert_eq!( d, None );
}

#[test]
fn test_skip_assign() {
    let (a,b) = scan_fmt!("1 2 3, 4 5, 6 7",
                          "{[^,]},{*[^,]},{[^,]}",
                          String, String) ;
    assert_eq!( a.unwrap(), "1 2 3" );
    assert_eq!( b.unwrap(), "6 7" );
}
