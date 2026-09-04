use std::str::FromStr;

use serde::Serialize;
use tree_sitter::{Language, LanguageError, Node, Parser, Tree};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::tree_sitter::languages::helpers::kind::{CustomRecurse, Kind, LevelOne, Recurse, StopRecurse};
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
    r#type: RecurseType
}

#[derive(Debug, Serialize)]
pub enum RecurseType {
    Recurse(Recurse),
    LevelOne(LevelOne),
    StopRecurse(StopRecurse),
    CustomRecurse(CustomRecurse),
}


fn walk(node: Node, extracts: &mut Vec<Extracted>, walked: &[usize], code: &str, context: &ContextV7) {
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        if let Ok(r#type) = LevelOne::from_str(child.kind()) {
            let kind = Kind::from_level_one(&r#type);
            let start = child.start_position();
            let end = child.end_position();

            extracts.push(Extracted {
                id: Uuid::new_v7(Timestamp::now(context)),
                kind,
                code: code[child.byte_range()].to_owned(),
                start_line: start.row,
                start_column: start.column,
                end_line: end.row,
                end_column: end.column,
                parent: None,
                r#type: RecurseType::LevelOne(r#type)
            });

            continue;
        }else if let Ok(r#type) = StopRecurse::from_str(child.kind()) {
            let kind = Kind::from_stop_recurse(&r#type);
            let start = child.start_position();
            let end = child.end_position();

            extracts.push(Extracted {
                id: Uuid::new_v7(Timestamp::now(context)),
                kind,
                code: code[child.byte_range()].to_owned(),
                start_line: start.row,
                start_column: start.column,
                end_line: end.row,
                end_column: end.column,
                parent: None,
                r#type: RecurseType::StopRecurse(r#type)
            });
            continue;
        }else if let Ok(r#type) = Recurse::from_str(child.kind()) {
            let kind = Kind::from_recurse(&r#type);
            let start = child.start_position();
            let end = child.end_position();

            extracts.push(Extracted {
                id: Uuid::new_v7(Timestamp::now(context)),
                kind,
                code: code[child.byte_range()].to_owned(),
                start_line: start.row,
                start_column: start.column,
                end_line: end.row,
                end_column: end.column,
                parent: None,
                r#type: RecurseType::Recurse(r#type)
            });
            walk(child, extracts, walked, code, context);
        }else if let Ok(r#type) = CustomRecurse::from_str(child.kind()) {
            match r#type {
                CustomRecurse::ClassSpecifier => todo!(),
                CustomRecurse::StructSpecifier => todo!(),
                CustomRecurse::TypeDeclaration => todo!(),
                CustomRecurse::MethodDeclaration => todo!(),
                CustomRecurse::StructItem => todo!(),
                CustomRecurse::ImplItem => todo!(),
                CustomRecurse::TraitItem => todo!(),
                CustomRecurse::ClassDeclaration => todo!(),
                CustomRecurse::PublicFieldDefinition => todo!(),
                CustomRecurse::MethodeDefinition => todo!(),
                CustomRecurse::HtmlElement => todo!(),
            }
        }
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
            if let Ok(name) = LevelOne::from_str(child.kind()) {
                let kind = Kind::from_level_one(name);
                let id = Uuid::new_v7(Timestamp::now(&context));
                let start_point = child.start_position();
                let end_point = child.end_position();
                let code = code[child.byte_range()].to_owned();
                extracts.push(Extracted {
                    kind,
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

        walked.sort();
        for child in root.named_children(&mut cursor) {
            if walked.binary_search(&child.id()).is_ok() {
                continue;
            }
        
            walk(child, &mut extracts, &walked, code, &context);
        }
        extracts
    }
}


