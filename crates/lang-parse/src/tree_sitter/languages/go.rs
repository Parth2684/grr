use tree_sitter::{Tree};

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_go::LANGUAGE.into())
}


// const_declaration -> level 1✅
// function_declaration✅
// type_declaration ✅
// var_declaration -> Level 1✅
// method_declaration ✅
 