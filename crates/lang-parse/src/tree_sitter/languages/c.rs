use tree_sitter::{Tree};

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_c::LANGUAGE.into())
}



// function -> function_definition
// if_statement -. Level 1
// #if  -> Level 1
// struct_specifier
// declaration -> function declaration
// preproc_ifdef -> macro