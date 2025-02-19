use crate::priv_prelude::*;

#[derive(Debug, Error, Clone, PartialEq, Hash)]
pub enum ParseErrorKind {
    #[error("Expected an import name, group of imports, or `*`.")]
    ExpectedImportNameGroupOrGlob,
    #[error("Expected an item.")]
    ExpectedAnItem,
    #[error("Expected a comma or closing parenthesis in function arguments.")]
    ExpectedCommaOrCloseParenInFnArgs,
    #[error("Unrecognized op code.")]
    UnrecognizedOpCode,
    #[error("Unexpected token in statement.")]
    UnexpectedTokenInStatement,
    #[error("This expression cannot be assigned to.")]
    UnassignableExpression,
    #[error("Unexpected token after array index.")]
    UnexpectedTokenAfterArrayIndex,
    #[error("Invalid literal to use as a field name.")]
    InvalidLiteralFieldName,
    #[error("Integer field names cannot have type suffixes.")]
    IntFieldWithTypeSuffix,
    #[error("Expected a field name.")]
    ExpectedFieldName,
    #[error("Expected a comma or closing parenthesis in this tuple or parenthesized expression.")]
    ExpectedCommaOrCloseParenInTupleOrParenExpression,
    #[error("Expected an expression.")]
    ExpectedExpression,
    #[error("Unexpected token after array length.")]
    UnexpectedTokenAfterArrayLength,
    #[error("Expected a comma, semicolon or closing bracket when parsing this array.")]
    ExpectedCommaSemicolonOrCloseBracketInArray,
    #[error("Unexpected token after asm return type.")]
    UnexpectedTokenAfterAsmReturnType,
    #[error("Malformed asm immediate value.")]
    MalformedAsmImmediate,
    #[error("Expected an identifier.")]
    ExpectedIdent,
    #[error("Unexpected token after str length.")]
    UnexpectedTokenAfterStrLength,
    #[error("Expected a type.")]
    ExpectedType,
    #[error("Unexpected token after array type length.")]
    UnexpectedTokenAfterArrayTypeLength,
    #[error("Expected an opening brace.")]
    ExpectedOpenBrace,
    #[error("Expected an opening parenthesis.")]
    ExpectedOpenParen,
    #[error("Expected an opening square bracket.")]
    ExpectedOpenBracket,
    #[error("Expected a literal.")]
    ExpectedLiteral,
    #[error("Expected a program kind (script, contract, predicate or library).")]
    ExpectedProgramKind,
    #[error("Expected `{}`.", kinds.iter().map(PunctKind::as_char).collect::<String>())]
    ExpectedPunct { kinds: Vec<PunctKind> },
    #[error("Expected `{}`.", word)]
    ExpectedKeyword { word: &'static str },
    #[error("Unexpected token after abi address.")]
    UnexpectedTokenAfterAbiAddress,
    #[error("Expected an attribute.")]
    ExpectedAnAttribute,
    #[error("Unexpected token after an attribute.")]
    UnexpectedTokenAfterAttribute,
    #[error("Identifiers cannot begin with a double underscore, as that naming convention is reserved for compiler intrinsics.")]
    InvalidDoubleUnderscore,
}

#[derive(Debug, Error, Clone, PartialEq, Hash)]
#[error("{}", kind)]
pub struct ParseError {
    pub span: Span,
    pub kind: ParseErrorKind,
}
