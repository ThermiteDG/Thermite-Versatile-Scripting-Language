// Lexer for the TVSL
use core::primitive::*;
use std::array;
use std::primitive::char;
use std::primitive::i64;
use std::primitive::i128;
use std::primitive::f32;
use std::primitive::f64;
use std::primitive::usize;

#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]

pub enum TokenVariety {
    LInt(i64), // Little Integer
    BInt(i128), // Big Integer
    LIntu(u64), // Little Unsigned Integer
    BIntu(u128), // Big Unsigned Integer
    Single(f32), // Single Precision
    Double(f64), // Double Precision
    Unit(()), // Unit
    Char(char), // Char
    Identitystring(String),
    Quotedstring(String),
    Usize(usize), // Usize
    Isize(isize),
    Arr(),   // Yet to be implemented array
    Asterisk,
    Carat,
    Closeparenthesis,
    Openparenthesis,
    Colon,
    Semicolon,
    Dot,
    Comma,
    Plus,
    Minus,
    Equal,
    Lessthan,
    Greaterthan,
    Closedsquare,
    Opensquare,
    Closedcurly,
    Opencurly,
    Backslash,
    Forwardslash,
    End,
    Underscore,
    Pipe,
    Tilde,
    Ampersand,
    Percent,
    Pound, 
    At,
    Exclamation,
    Question,
    Sinlequote,
    Doublequote,
    Grave
}

impl From<String> for TokenVariety {
    fn from(dif: String) -> TokenVariety {
        TokenVariety::Identitystring(dif)
    }
}

impl<'a> From<&'a str> for TokenVariety {
    fn from(dif: &'a str) -> TokenVariety {
        TokenVariety::Identitystring(dif.to_string())
    }
}

impl From<i64> for TokenVariety {
    fn from(dif: i64) -> TokenVariety {
        TokenVariety::LInt(dif)
    }
}

impl From<i128> for TokenVariety {
    fn from(dif: i128) -> TokenVariety {
        TokenVariety::BInt(dif)
    }
}

impl From<f32> for TokenVariety {
    fn from(dif: f32) -> TokenVariety {
        TokenVariety::Single(dif)
    }
}

impl From<f64> for TokenVariety {
    fn from(dif: f64) -> TokenVariety {
        TokenVariety::Double(dif)
    }
}

impl From<u64> for TokenVariety {
    fn from(dif: u64) -> TokenVariety {
        TokenVariety::LIntu(dif)
    }
}

impl From<u128> for TokenVariety {
    fn from(dif: u128) -> TokenVariety {
        TokenVariety::BIntu(dif)
    }
}

impl From<usize> for TokenVariety {
    fn from(dif: usize) -> TokenVariety {
        TokenVariety::Usize(dif)
    }
}

impl From<()> for TokenVariety {
    fn from(dif: ()) -> TokenVariety {
        TokenVariety::Unit(dif)
    }
}

impl From<char> for TokenVariety {
    fn from(dif: char) -> TokenVariety {
        TokenVariety::Char(dif)
    }
}

impl From<isize> for TokenVariety {
    fn from(dif: isize) -> TokenVariety {
        TokenVariety::Isize(dif)
    }
}

pub struct Lexer {
    
}