use tree_sitter::Tree;

use crate::tree_sitter::languages::{CustomError, parse_code};

pub fn parse(code: &str) -> Result<Tree, CustomError> {
    parse_code(code, tree_sitter_rust::LANGUAGE.into())
}


// function -> function_item include attribute_item above it if exists
// enum -> enum_item attribute_item above it if exists
// struct -> struct_item include attribute_item above it if exists
// impl -> impl_item
// methods -> fns inside impl_item
// type -> type_item
// const -> const_item
// trait -> trait_item
// macro -> macro_definition
// static -> static_item level 1 ✅
