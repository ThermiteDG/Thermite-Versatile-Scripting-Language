#![deny(missing_docs)]

mod parser;
mod lexer;
mod lower;
mod analyze;
mod errorhandle;
mod codemap;
mod driver;
pub use driver::Driver;


#[macro_use]
mod macros;
pub use self::parser::Parser;
mod ast;
pub use self::ast::{Literal, LiteralKind, Ident, DottedIdent};


#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub(crate) fn main() {
    println!("Hello, world!");
}
