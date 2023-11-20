#![allow(missing_docs)]
use codemap::Span;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LiteralVariety {
    Int(usize),
    Double(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub span: Span,
    pub kind: LiteralVariety,
}

impl From<usize> for LiteralVariety {
    fn from(other: usize) -> LiteralVariety {
        LiteralVariety::Int(other)
    }
}

impl From<f64> for LiteralVariety {
    fn from(other: f64) -> LiteralVariety {
        LiteralVariety::Double(other)
    }
}

impl PartialEq<LiteralVariety> for Literal {
    fn eq(&self, other: &LiteralVariety) -> bool {
        &self.kind == other
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident {
    pub span: Span,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DottedIdent {
    pub span: Span,
    pub parts: Vec<Ident>,
}

impl<'a> PartialEq<&'a str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        &self.name == other
    }
}

impl<'a, T: AsRef<[&'a str]>> PartialEq<T> for DottedIdent {
    fn eq(&self, other: &T) -> bool {
        self.parts.iter()
            .zip(other.as_ref().iter())
            .all(|(l, r)| l == r)
    }
}