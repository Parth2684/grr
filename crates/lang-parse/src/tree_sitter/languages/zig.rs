use tree_sitter::{Tree};

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_zig::LANGUAGE.into())
}


// variable_declaration -> top level only group one liners✅
// comptime_declaration ✅
// test_declaration✅ 
// function_declaration✅