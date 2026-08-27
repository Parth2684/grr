use tree_sitter::{Language, Parser, Tree, LanguageError};

pub mod rust;
pub mod javascript;
pub mod typescript;
pub mod tsx;
pub mod python;
pub mod go;
pub mod sql;
pub mod html;
pub mod css;
pub mod c;
pub mod cpp;
pub mod zig;

#[derive(Debug)]
pub enum CustomError {
    LanguageError(LanguageError),
    String(String)
}



pub fn parse_code(code: &str, language: Language) -> Result<Tree, CustomError> {
    let mut parser = Parser::new();

    parser
        .set_language(&language)
        .map_err(CustomError::LanguageError)?;

    parser
        .parse(code, None)
        .ok_or_else(|| CustomError::String("Could not get tree".into()))
}
