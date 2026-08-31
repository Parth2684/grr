use serde::Serialize;
use tree_sitter::{Language, LanguageError, Node, Parser, Tree};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::tree_sitter::languages::helpers::kind::Kind;
mod helpers;

pub mod c;
pub mod cpp;
pub mod css;
pub mod go;
pub mod html;
pub mod javascript;
pub mod python;
pub mod rust;
pub mod sql;
pub mod tsx;
pub mod typescript;
pub mod zig;

#[derive(Debug)]
pub enum CustomError {
    LanguageError(LanguageError),
    String(String),
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

#[derive(Debug, Serialize)]
pub struct Extracted {
    id: Uuid,
    kind: Kind,
    code: String,
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    parent: Option<Uuid>,
}


fn walk(node: Node, extracts: &mut Vec<Extracted>, walked: &Vec<usize>) {
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if walked.binary_search(&child.id()).is_ok() {
            return
        }else {
            
        }
    }

    if node.child_count() == 0 {
        return;
    }
}


pub trait Extract {
    fn extract(tree: Tree, code: &str) -> Vec<Extracted> {
        let mut extracts = Vec::new();
        let mut walked = Vec::new();
        let context = ContextV7::new();
        let root = tree.root_node();
        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            if let Some(name) = Kind::from_level_one(child.kind()) {
                let id = Uuid::new_v7(Timestamp::now(&context));
                let start_point = child.start_position();
                let end_point = child.end_position();
                let code = code[child.byte_range()].to_owned();
                extracts.push(Extracted {
                    kind: name,
                    start_line: start_point.row,
                    start_column: start_point.column,
                    end_line: end_point.row,
                    end_column: end_point.column,
                    code,
                    id,
                    parent: None,
                });
                walked.push(child.id());
            }
        }

        extracts.sort();
        
        extracts
    }
}


