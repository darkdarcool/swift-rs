#![allow(dead_code)]

use crate::Lexer;
use crate::token::TokenKind;

/// A byte handler is a function that handles a specific byte value
pub type ByteHandler = Option<for<'arena> fn(&mut Lexer<'arena>)>;

/// List of byte handlers for each byte value.
/// Ref: <https://www.freecodecamp.org/news/ascii-table-hex-to-ascii-value-character-code-chart-2/>
pub static BYTE_HANDLERS: [ByteHandler; 256] = [
//   0    1    2    3    4    5    6    7    8    9    A    B    C    D    E    F   //
    EOF, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 0 16
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 1 32
    SPS, ___, ___, ___, IDN, ___, ___, ___, ___, ___, ___, PLS, ___, ___, ___, ___, // 2 48
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 3 64
    ___, LUA, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 4 80
    ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, ___, // 5 96
    QUI, LLA, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, // 6 112
    IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, IDN, ___, ___, ___, ___, ___, // 7
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 8
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // 9
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // A
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // B
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // C
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // D
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // E
    UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, UNI, // F
];

pub const ___: ByteHandler = None;

/// Literal uppercase `A`
pub const LUA: ByteHandler = Some(|lex| {
    // Only `As` is a keyword that starts with uppercase `A`
    lex.token.kind = match lex.identifier_handler() {
        "As" => TokenKind::As,
        _ => TokenKind::Identifier,
    }
});

pub const UNI: ByteHandler = Some(|lex| {
    lex.source.advance(1);
    // TODO: Whatever this is
});

/// `` ` ``
///
/// Stands for quoted identifier (used for identifiers that would conflict with keywords)
pub const QUI: ByteHandler = Some(|lex| {
    lex.quoted_identifier_handler();

    lex.token.kind = TokenKind::Identifier;
});

/// `+`
pub const PLS: ByteHandler = Some(|lex| {
    if lex.next_eq('=') {
        // lex.index += 2;
        // lex.source.advance(2);
        lex.source.advance(1);
        lex.token.kind = TokenKind::PlusEq;
    } else {
        lex.bump();
        lex.token.kind = TokenKind::Plus;
    }
});


/// Identifiers, and special characters like `$` and `_`
pub const IDN: ByteHandler = Some(|lex| {
    lex.identifier_handler();

    lex.token.kind = TokenKind::Identifier;
});

/// Literal lowercase A
pub const LLA: ByteHandler = Some(|lex| {
    // lex.identifier_handler();
    lex.token.kind = match lex.identifier_handler() {
        "await" => TokenKind::Await,
        // "As" => TokenKind::As,
        // I don't think capitalization counts here
        _ => TokenKind::Identifier,
    }
    // lex.token.kind = TokenKind::Identifier;
});

/// Space
pub const SPS: ByteHandler = Some(|lex| {
    lex.token.kind = TokenKind::Empty;
    lex.bump();
});

pub const EOF: ByteHandler = Some(|_lex| {
    println!("End of file reached");
});

