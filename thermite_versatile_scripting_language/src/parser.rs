#![allow(missing_docs, dead_code, unused_imports)]
#![allow(unused_variables)]

use std::rc::Rc;

use crate::lexer::{Token, TokenVariety};
use codemap::{Span, FileMap};
use parse::ast::{Literal, LiteralVariety, Ident, DottedIdent};
use anyhow::bail;
use errors::*;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    filemap: Rc<FileMap>,
    current_index: usize,
}

impl Parser {
    /// Create a new parser.
    pub fn new(tokens: Vec<Token>, filemap: Rc<FileMap>) -> Parser {
        let current_index = 0;
        Parser { tokens, filemap, current_index }
    }

    /// Peek at the current token.
    fn peek(&self) -> Option<&TokenVariety> {
        self.tokens.get(self.current_index).map(|t| &t.variety)
    }

    /// Get the current token, moving the index along one.
    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current_index);

        if token.is_some() {
            self.current_index += 1;
        }

        token
    }
}

impl Parser {
    fn parse_literal(&mut self) -> std::io::Result<Literal> {
        match self.peek() {
            Some(&TokenVariety::Int(_)) |
            Some(&TokenVariety::Double(_)) |
            Some(&TokenVariety::Quotedstring(_)) => {}
            Some(_) => bail!("Expected a literal"),
            None => bail!(ErrorKind::UnexpectedEOF),
        };

        let next = self.next().expect("unreachable");
        let literal_variety = match next.variety {
            TokenVariety::Int(i) => LiteralVariety::Int(i as usize),
            TokenVariety::Double(d) => LiteralVariety::Double(d),
            TokenVariety::Quotedstring(ref s) => LiteralVariety::String(s.clone()),
            ref other => panic!("Unreachable token.variety: {:?}", other),
        };

        Ok(Literal {
            span: next.span,
            kind: literal_variety,
        })
    }
}

impl Parser {
    fn parse_ident(&mut self) -> std::io::Result<Ident> {
        match self.peek() {
            Some(&TokenVariety::Identitystring(_)) => {}
            _ => bail!("Expected an identifier"),
        }

        let next = self.next().unwrap();

        if let TokenVariety::Identitystring(ref ident) = next.variety {
            Ok(Ident {
                span: next.span,
                name: ident.clone(),
            })
        } else {
            unreachable!()
        }
    }

    fn parse_dotted_ident(&mut self) -> std::io::Result<DottedIdent> {
        let first = self.parse_ident()?;
        let mut parts = vec![first];

        while self.peek() == Some(&TokenVariety::Dot) {
            let _ = self.next();
            let next = self.parse_ident()?;
            parts.push(next);
        }

        // the span for a dotted ident should be the union of the spans for
        // each of its components.
        let span = parts.iter()
            .skip(1)
            .fold(parts[0].span, |l, r| self.filemap.merge(l, r.span));

        Ok(DottedIdent { span, parts })
    }
}
