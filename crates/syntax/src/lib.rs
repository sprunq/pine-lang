pub mod ast;
mod lexer;
pub mod parser;
mod token;

lalrpop_util::lalrpop_mod!(
    #[allow(clippy::all)]
    grammar,
    "/grammar.rs"
);
