# scan_fmt
scan_fmt provides a simple scanf()-like input for Rust.  The goal is to make it easier to read data from a string or stdin.

Currently the format string only supports the following special sequences:
<pre>
 {} = read input until next character or whitespace
 {{ = escaped {
 }} = escaped }</pre>
The plan is to add more format conversions (e.g., {[0-9]} to read numbers).

### Examples
```
 #[macro_use] extern crate scan_fmt;
 fn main() {
   let (a,b,c) = scan_fmt!( "hello 12 345 bye", // input string
                            "hello {} {} {}",   // format
                            u8, i32, String);   // type of a-c Options
   assert_eq!( a.unwrap(), 12 ) ;
   assert_eq!( b.unwrap(), 345 ) ;
   assert_eq!( c.unwrap(), "bye" ) ;

   println!("Enter something like: 123-22");
   let (c,d) = scanln_fmt!( "{}-{}",   // format
                            u16, u8);  // type of a&b Options
   match (c,d) {
     (Some(cc),Some(dd)) => println!("Got {} and {}",cc,dd),
     _ => println!("input error")
   }
   // Note - currently scanln_fmt! just calls unwrap() on read_line()
  }
```

### Limitations
There is no compile-time warning if the number of {}'s in the format string doesn't match the number of return values.  You'll just get None for extra return values.  See src/lib.rs for more details.
