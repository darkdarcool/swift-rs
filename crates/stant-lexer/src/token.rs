#[derive(Debug, Default, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

// Yes, this is 100 lines of tokens. I'm sorry.
// TODO: Refactor this enum to make it more organized and readable
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Please add this part of the enum (Plus, PlusEq should be their own thing)
    Plus,
    PlusEq,
    Identifier,

    // Declarations
    AssociatedType,
    Class,
    Deinit,
    Enum,
    Extension,
    FilePrivate,
    Func,
    Import,
    Init,
    Inout,
    Internal,
    Let,
    Open,
    Operator,
    Private,
    PrecedenceGroup,
    Protocol,
    Public,
    Rethrows,
    Static,
    Subscript,
    TypeAlias,
    Var,

    // Statements
    Break,
    Case,
    Catch,
    Continue,
    Default,
    Defer,
    Do,
    Else,
    FallThrough,
    For,
    Guard,
    If,
    In,
    Repeat,
    Return,
    Throw,
    Throws,
    True,
    Try,

    // Expressions and Types
    Any,
    As,
    Await,
    // Catch,
    False,
    Is,
    Nil,
    // Rethrows,
    /// `self` (lowercase `Self`)
    LSelf,
    /// `Self` (uppercase `Self`)
    USelf,
    Super,
    // Throw,
    // Throws,
    // True,
    // Try,

    // Patterns
    Underscore,

    // Keywords that start with a number sign (#)
    /// `#available`
    HashAvailable,
    /// `#colorLiteral`
    HashColorLiteral,
    /// `#else`
    HashElse,
    /// `#elseif`
    HashElseIf,
    /// `#endif`
    HashEndIf,
    /// `#fileLiteral`
    HashFileLiteral,
    /// `#if`
    HashIf,
    /// `#imageLiteral`
    HashImageLiteral,
    /// `#keyPath`
    HashKeyPath,
    /// `#selector`
    HashSelector,
    /// `#sourceLocation`
    HashSourceLocation,
    /// `#unavailable`
    HashUnavailable,

    // The default token
    #[default]
    Empty,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new() -> Self {
        Token {
            kind: TokenKind::Empty,
            span: Span { start: 0, end: 0 },
        }
    }
}
