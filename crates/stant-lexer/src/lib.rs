mod source;
mod handler;
mod identifier;
pub mod token;

use handler::{ByteHandler, BYTE_HANDLERS};
use oxc_allocator::Allocator;
use crate::source::Source;
use crate::token::{Token, TokenKind};

// `Lexer` is a struct that holds a reference to an `Allocator` and a `Source` instance.
///
/// The `Lexer` is responsible for tokenizing the source code. It uses the `Allocator` to allocate memory for the tokens,
/// and the `Source` to keep track of the current position in the source code.
///
/// # Fields
///
/// * `alloc`: A reference to an `Allocator` instance.
/// * `source`: A `Source` instance that represents the source code.
pub struct Lexer<'alloc> {
    /// Reference to the given `Allocator` instance.
    #[allow(dead_code)]
    pub(crate) alloc: &'alloc Allocator,
    /// The source code to be tokenized.
    pub source: Source,
    /// The current index in the source code.
    // index: usize,

    pub token: Token,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` instance.
    ///
    /// # Parameters
    ///
    /// * `alloc`: A reference to an `Allocator` instance.
    /// * `source`: A string slice that represents the source code.
    ///
    /// # Returns
    ///
    /// A `Lexer` instance.
    pub fn new(alloc: &'a Allocator, source: &'a str) -> Self {
        Lexer {
            alloc,
            source: Source::new(source),
            // index: 0,
            token: Token::default(),
        }
    }

    /// Returns the [`ByteHandler`] for the given byte
    /// Returns `None` if the byte does not have a handler
    fn handler_from_byte(&self, byte: u8) -> ByteHandler {
        unsafe { *(&BYTE_HANDLERS as *const ByteHandler).offset(byte as isize) }
    }

    fn __handler_index_from_byte(&self, byte: u8) -> usize {
        byte as usize
    }

    #[inline]
    fn bump(&mut self) {
        self.source.advance_ptr();
        //self.index += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let next_byte = self.read_byte();
        if let Some(handler) = self.handler_from_byte(next_byte) {
            self.token.span.start = self.source.current_pos(); // self.index;
            handler(self);
        } else {
            println!("Unexpected byte: {} [{}]", next_byte as char, next_byte);
            self.bump();
        }

        self.token.span.end = self.source.current_pos(); // self.index

        let tok = self.token;
        self.token = Token::default();

        tok
    }

    pub fn token_as_str(&self) -> &'a str {
        let start = self.token.span.start;
        let end = self.token.span.end;

        self.source.get_slice(start, end)
    }

    pub fn read_byte(&mut self) -> u8 {
        // unsafe { *self.source.ptr.add(self.index) }
        // self.source.advance(self.index)
        self.source.current()
    }

    pub fn next_eq(&mut self, c: char) -> bool {
        if self.source.peek() == Some(c as u8) {
            // In the event this breaks, change this
            // self.source.advance(1);
            self.bump();
            true
        } else {
            false
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.source.is_at_end()
    }

    pub fn peek(&self) -> u8 {
        self.source.peek().unwrap()
    }

    fn consume_char(&mut self) -> u8 {
        self.source.next_char().unwrap() as u8
    }
}

/*
assert_lexer!("hello +", [Token::Identifier, Token::Plus])
*/
macro_rules! assert_lexer {
    ($src:expr, [$($tok:expr),*]) => {
        let alloc = Allocator::default();
        let mut lexer = Lexer::new(&alloc, $src);
        let mut tokens = Vec::new();
        while !lexer.is_at_end() {
            let tok = lexer.next_token();
            tokens.push(tok);
        }

        let expected = vec![$($tok),*];
        // filter out empty Token in tokens
        let tokens: Vec<_> = tokens.into_iter().filter(|t| t.kind != TokenKind::Empty).collect();

        for (i, tok) in tokens.iter().enumerate() {
            // print the tok
            assert_eq!(tok.kind, expected[i]);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_empty() {
        assert_lexer!("", []);
    }

    #[test]
    fn test_lexer_plain_identifier() {
        assert_lexer!("hello swift people", [TokenKind::Identifier, TokenKind::Identifier, TokenKind::Identifier]);
    }

    #[test]
    fn test_lexer_plain_identifier_with_quoted_identifier() {
        assert_lexer!("hello `swift` people", [TokenKind::Identifier, TokenKind::Identifier, TokenKind::Identifier]);
    }

    #[test]
    fn test_lexer_multiple_quoted_identifier() {
        assert_lexer!("`hello` `swift` `people`", [TokenKind::Identifier, TokenKind::Identifier, TokenKind::Identifier]);
    }

    #[test]
    fn test_lexer_keyword() {
        assert_lexer!("await As", [TokenKind::Await, TokenKind::As]);
    }

    #[test]
    fn test_lexer_keyword_with_identifier() {
        assert_lexer!("await As hello", [TokenKind::Await, TokenKind::As, TokenKind::Identifier]);
    }
}



