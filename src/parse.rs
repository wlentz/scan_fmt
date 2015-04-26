// Copyright 2015 Will Lentz.
// Licensed under the MIT license.
use std ;

// Make it slightly easier to scan through a Vec<>
struct VecScanner {
    data : Vec<char>,
    pos : usize
}

impl VecScanner {
    fn new( d : Vec<char>) -> VecScanner {
        VecScanner { data : d, pos : 0 }
    }

    fn cur(&self) -> char {
        self.data[self.pos]
    }

    fn is_end(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn inc(&mut self) -> bool {
        self.pos += 1;
        !self.is_end()
    }

    fn is_whitespace(&self, c : char) -> bool {
        match c {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false
        }
    }

    fn skip_whitespace(&mut self) -> bool {
        while self.pos < self.data.len() {
            if self.is_whitespace(self.data[self.pos]) {
                self.pos += 1 ;
            }
            else {
                break ;
            }
        }
        !self.is_end()
    }

    fn get_token(&mut self, end_char : char) -> String {
        let pos_start = self.pos ;
        let mut pos_end = self.pos ;
        while pos_end < self.data.len()
            && self.data[pos_end] != end_char
            && !self.is_whitespace(self.data[pos_end])
        {
            pos_end += 1;
        }
        self.pos = pos_end ;
        self.data[pos_start..pos_end].iter().cloned().collect()
    }
}

// Extract String tokens from the input string based on
// the format string.  See lib.rs for more info.
// Returns an iterator of the String results.
pub fn scan( input_string : &str, format : &str )
             -> std::vec::IntoIter<String>
{
    let mut res : Vec<String> = vec![] ;
    let mut fmt = VecScanner::new(format.chars().collect()) ;
    let mut instr = VecScanner::new(input_string.chars().collect()) ;
    loop {
        let mut do_compare = true ;
        if ! fmt.skip_whitespace() { break; }
        if ! instr.skip_whitespace() { break; }

        if fmt.cur() == '{' {
            if ! fmt.inc() { break; }
            if fmt.cur() == '}' {
                // got a capture pair {}
                fmt.inc();
                let c_end = if ! fmt.is_end() { fmt.cur() } else { '\0' } ;
                res.push( instr.get_token( c_end ) ) ;
                do_compare = false ;
            }
            else if fmt.cur() != '{' {
                // If we get a '{' without a matching { or }, something
                // is wrong.
                break ;
            }
        }
        else {
            if fmt.cur() == '}' {
                // handle escaped }} by skipping first '}'
                if ! fmt.inc() { break; }
            }
        }
        if do_compare {
            if fmt.cur() != instr.cur() { break; }
            if ! fmt.inc() { break; }
            if ! instr.inc() { break; }
        }
    }
    res.into_iter()
}


#[test]
fn test_simple() {
    let mut res = scan(" data 42-12=30",
                       "data {}-{}={}");
    assert_eq!( res.next().unwrap(), "42" ) ;
    assert_eq!( res.next().unwrap(), "12" ) ;
    assert_eq!( res.next().unwrap(), "30" ) ;
    assert_eq!( res.next(), None ) ;
}

#[test]
fn test_complex() {
    let mut res
        = scan("test{123  bye -456} hi  22.7e-1",
               "test{{{} bye {}}} hi {}") ;
    assert_eq!( res.next().unwrap(), "123" ) ;
    assert_eq!( res.next().unwrap(), "-456" ) ;
    assert_eq!( res.next().unwrap(), "22.7e-1" ) ;
    assert_eq!( res.next(), None ) ;
}

#[test]
fn test_endline() {
    let mut res = scan("hi 15\r\n", "{} {}" ) ;
    assert_eq!( res.next().unwrap(), "hi" );
    assert_eq!( res.next().unwrap(), "15" );
}
