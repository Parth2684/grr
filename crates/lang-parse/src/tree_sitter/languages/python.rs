use tree_sitter::{Tree};

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_python::LANGUAGE.into())
}


// if_statement level 1 ✅
// try_statement level 1 -> functions ✅
// function_definition
// expression_statement 
// class_definition 
// decorated_definition -> ignore function inside it as function