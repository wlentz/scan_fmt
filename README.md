# scan_fmt ![BuildStatus](https://travis-ci.org/wlentz/scan_fmt.svg?branch=master)
scan_fmt provides a simple scanf()-like input for Rust.  The goal is to make it easier to read data from a string or stdin.

Currently the format string supports the following special sequences:
<pre>
   {{ = escape for '{'
   }} = escape for '}'
   {} = return any value (until next whitespace)
   {d} = return base-10 decimal
   {x} = return hex (0xab or ab)
   {f} = return float
   {*d} = "*" as the first character means "match but don't return"
   {[...]} = return pattern.
     ^ inverts if it is the first character
     - is for ranges.  For a literal - put it at the start or end.
     To add a literal ] do "[]abc]"
   Examples:
     {[0-9ab]} = match 0-9 or a or b
     {[^,.]} = match anything but , or .</pre>

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
   let (c,d) = scanln_fmt!( "{d}-{d}", // format
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
