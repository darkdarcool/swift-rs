use stant_lexer::{token::TokenKind, Lexer};
use oxc_allocator::Allocator;

fn main() {
    let alloc = Allocator::default();

    let source = "Any";

    let mut lexer = Lexer::new(&alloc, source);

    while !lexer.source.is_at_end() {
        let tok = lexer.next_token();
        if tok.kind != TokenKind::Empty {
            let span = tok.span;
            let c = &source[span.start..span.end];
            println!("{:?}: {}", tok.kind, c);
        }
    }
}
