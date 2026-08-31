use tree_sitter::{Tree};

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_cpp::LANGUAGE.into())
}


// function_definition
// preproc_if  -> Level 1
// preproc_ifdef -> Level 1
// namespace_definition
// class_specifier ✅
// field_declaration group with according to class_specifier ✅
// struct_specifier
// enum_specifier -> level 1