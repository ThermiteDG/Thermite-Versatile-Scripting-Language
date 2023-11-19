// Lexer for the TVSL
use core::primitive::*;
use std::primitive::char;
use std::primitive::i64;
use std::primitive::i128;
use std::primitive::f32;
use std::primitive::f64;
use std::primitive::usize;
use std::io::ErrorKind;
use std::str;
use std::io::Result;
use anyhow::bail;
use codemap::Span;
use errors::*;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(tag = "type")]

pub enum TokenVariety {
    Int(i128), // Integer
    Intu(u128), // Unsigned Integer
    Double(f64), // Double Precision
    Unit(()), // Unit
    Char(char), // Char
    Identitystring(String),
    Quotedstring(String),
    Usize(usize), // Usize
    Isize(isize),  // Yet to be implemented array
    Asterisk,
    Caret,
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
    Closedbracket,
    Openbracket,
    Closedcurly,
    Opencurly,
    Backslash,
    Forwardslash,
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
    Grave,
    Dollarsign
}

pub enum FunctionTokens {
    Funct { // how you enter a method/function
        name: String,
        parameters: Vec<char>,
        outputtype: (i64, i128, f32, f64, String, char, usize, isize, (), u64, u128)
    },
    Enumerator {
        name: String

    },
    Structure {
        name: String
    },
    Ifstatement {

    },
    Efstatment {

    },
    Elstatement {

    },
    Whilestatment {

    },
    Forstatment {

    }
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

impl From<i128> for TokenVariety {
    fn from(dif: i128) -> TokenVariety {
        TokenVariety::Int(dif)
    }
}

impl From<f64> for TokenVariety {
    fn from(dif: f64) -> TokenVariety {
        TokenVariety::Double(dif)
    }
}

impl From<u128> for TokenVariety {
    fn from(dif: u128) -> TokenVariety {
        TokenVariety::Intu(dif)
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

fn get_tokenized_identity(data: &str) -> Result<(TokenVariety, usize)> {
    // identities never start with a num
    match data.chars().next() {
        Some(ch) if ch.is_digit(10) => bail!("Error: Identity cannot start with a number!"),
        None => bail!(ErrorKind::UnexpectedEOF),
        _ => {}
    }
    let (got, bytes_read) = take_while(data, |ch| ch == '_' || ch.is_alphanumeric())?;

    // TODO: identify keywords using match

    let token = TokenVariety::Identitystring(got.to_string());
    Ok((token, bytes_read))
}

// Tokenize number
fn get_tokenized_number(data: &str) -> Result<(TokenVariety, usize)> {
    let mut seen_dot = false;

    let (decimal, bytes_read) = take_while(data, |c| {
        if c.is_digit(10) {
            true
        } else if c == '.' {
            if !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    })?;

    if seen_dot {
        let n: f64 = decimal.parse()?;
        Ok((TokenVariety::Double(n), bytes_read))
    } else {
        let n: i128 = decimal.parse()?;
        Ok((TokenVariety::Int(n), bytes_read))
    }
}

fn skip_whitespace(data: &str) -> usize {
    match take_while(data, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0
    }
}

fn ignore_comments(src: &str) -> usize {
    let comment = [("/#", "#/"), ("/#", "\n")];

    for &(pattern, matcher) in &comment {
        if src.starts_with(pattern) {
            let leftover = skip_until(src, matcher);
            return src.len() - leftover.len();
        }
    }
    0
}

fn skip_until<'a>(mut src: &'a str, pattern: &str) -> &'a str {
    while !src.is_empty() && !src.starts_with(pattern) {
        let next_char_size = src.chars().next().expect("The string is not empty").len_utf8();
        src = &src[next_char_size..];
    }

    &src[pattern.len()..]
}

// Skip by all whitespace characters and all comments
fn skip(src: &str) -> usize {
    let mut remaining = src;

    loop {
        let whitespace = skip_whitespace(remaining);
        remaining = &remaining[whitespace..];
        let comments = ignore_comments(remaining);
        remaining = &remaining[comments..];

        if whitespace + comments == 0 {
            return src.len() - remaining.len();
        }
    }
}

/// Consumes bytes while a predicate evaluates to true.
fn take_while<F>(data: &str, mut pred: F) -> Result<(&str, usize)>  
where F: FnMut(char) -> bool
{
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        bail!("No Matches");
    } else {
        Ok((&data[..current_index], current_index))
    }
}

// Get single token from the input
pub fn single_token_tokenizer(data: &str) -> Result<(TokenVariety, usize)> {
    let next = match data.chars().next() {
        Some(c) => c,
        None => bail!(ErrorKind::UnexpectedEOF),
    };

    let (token, length) = match next {
        '.' => (TokenVariety::Dot, 1),
        '=' => (TokenVariety::Equal, 1),
        '+' => (TokenVariety::Plus, 1),
        '-' => (TokenVariety::Minus, 1),
        '*' => (TokenVariety::Asterisk, 1),
        '/' => (TokenVariety::Forwardslash, 1),
        '@' => (TokenVariety::At, 1),
        '^' => (TokenVariety::Caret, 1),
        '(' => (TokenVariety::Openparenthesis, 1),
        ')' => (TokenVariety::Closeparenthesis, 1),
        '[' => (TokenVariety::Openbracket, 1),
        ']' => (TokenVariety::Closedbracket, 1),
        '{' => (TokenVariety::Opencurly, 1),
        '}' => (TokenVariety::Closedcurly, 1),
        '_' => (TokenVariety::Underscore, 1),
        '%' => (TokenVariety::Percent, 1),
        '&' => (TokenVariety::Ampersand, 1),
        '#' => (TokenVariety::Pound, 1),
        '$' => (TokenVariety::Dollarsign, 1),
        '`' => (TokenVariety::Grave, 1),
        '~' => (TokenVariety::Tilde, 1),
        ',' => (TokenVariety::Comma, 1),
        '<' => (TokenVariety::Lessthan, 1),
        '>' => (TokenVariety::Greaterthan, 1),
        '?' => (TokenVariety::Question, 1),
        '!' => (TokenVariety::Exclamation, 1),
        '"' => (TokenVariety::Doublequote, 1),
        '0'..='9' => get_tokenized_number(data).chain_error(|| "Cannot tokenize number data")?,
        c @ '_' | c if c.is_alphabetic() => get_tokenized_identity(data).chain_err(|| "Cannot tokenize identifier data")?,
        other => bail!(ErrorKind::UnknownCharacter(other)),
    };

    Ok((token, length))
}

struct Tokenizer<'a> {
    current_index: usize,
    remaining_text: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            remaining_text: src,
        }
    }

    fn next_token_up(&mut self) -> Result<Option<(TokenVariety, usize, usize)>> {
        self.skip_whitespace();

        if self.remaining_text.is_empty() {
            Ok(None)
        } else {
            let start = self.current_index;
            let token = self._next_token().chain_err(|| ErrorKind::MessageWithLocation(self.current_index, "Cannot read next token"))?;
            let end = self.current_index;
            Ok(Some((token, start, end)))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = skip(self.remaining_text);
        self.chomp(skipped);
    }

    fn _next_token(&mut self) -> Result<TokenVariety> {
        let (token, bytes_read) = single_token_tokenizer(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(token)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes];
        self.current_index += num_bytes;
    }
}

// Turn line of valid TVSL into its constituent tokens including its start and end point
pub fn tokenize(src: &str) -> Result<Vec<(TokenVariety, usize, usize)>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next_token_up()? {
        tokens.push(token);
    }

    Ok(tokens)
}
