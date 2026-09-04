use tree_sitter::{Tree};

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
}

// arrow_function✅
// function_declaration -> (do recursive search for recursive functions inside it) ✅
// interface_declaration ✅
// level 1 expression_statement (group them if less than 5 liners in all from a file first if consequent) -> Hooks in react no recursion✅ 
// level 1 if_statement ✅
// level 1 return_statement and jsx_element  inside it
// level 1 else_statement ✅
// type_alias_declaration ✅
// class_declaration ✅ 
// public_field_definition group by class ✅ 
// method_definition✅ 
// enum_declaration✅
// ambient_declaration level 1✅
