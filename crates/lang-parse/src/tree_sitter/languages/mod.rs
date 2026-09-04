use std::str::FromStr;

use tree_sitter::{Language, LanguageError, Node, Parser, Tree};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::tree_sitter::languages::helpers::kind::{
    CustomRecurse, Kind, LevelOne, Recurse, StopRecurse,
};
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

#[derive(Debug)]
struct StructInfo {
    path: String,
    name: String,
}

#[derive(Debug)]
pub struct Extracted {
    id: Uuid,
    kind: Kind,
    code: String,
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    parent: Option<Uuid>,
    r#type: RecurseType,
    struct_name: Option<String>,
}

#[derive(Debug)]
pub enum RecurseType {
    Recurse(Recurse),
    LevelOne(LevelOne),
    StopRecurse(StopRecurse),
    CustomRecurse(CustomRecurse),
}

fn walk(node: Node, extracts: &mut Vec<Extracted>, code: &str, context: &ContextV7) {
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
                r#type: RecurseType::LevelOne(r#type),
                struct_name: None,
            });

            continue;
        } else if let Ok(r#type) = StopRecurse::from_str(child.kind()) {
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
                r#type: RecurseType::StopRecurse(r#type),
                struct_name: None,
            });
            continue;
        } else if let Ok(r#type) = Recurse::from_str(child.kind()) {
            let kind = Kind::from_recurse(&r#type);
            let start = child.start_position();
            let end = child.end_position();

            extracts.push(Extracted {
                id: Uuid::new_v7(Timestamp::now(context)),
                code: code[child.byte_range()].to_owned(),
                start_line: start.row,
                start_column: start.column,
                end_line: end.row,
                end_column: end.column,
                parent: None,
                kind,
                r#type: RecurseType::Recurse(r#type),
                struct_name: None,
            });
            walk(child, extracts, code, context);
        } else if let Ok(r#type) = CustomRecurse::from_str(child.kind()) {
            match r#type {
                CustomRecurse::StructItem => {
                    let mut cursor = node.walk();
                    let kind = Kind::from_custom_recurse(&r#type);
                    let mut struct_info: Option<String> = None;
                    let start = child.start_position();
                    let end = child.end_position();
                    for kid in child.children(&mut cursor) {
                        if kid.kind() == "type_identifier" {
                            struct_info = Some(code[kid.byte_range()].to_owned())
                        }
                    }
                    extracts.push(Extracted {
                        id: Uuid::new_v7(Timestamp::now(context)),
                        kind,
                        code: code[child.byte_range()].to_owned(),
                        start_line: start.row,
                        start_column: start.column,
                        end_line: end.row,
                        end_column: end.column,
                        parent: None,
                        r#type: RecurseType::CustomRecurse(r#type),
                        struct_name: struct_info,
                    });
                }
                CustomRecurse::ClassSpecifier => {}
                CustomRecurse::StructSpecifier => todo!(),
                CustomRecurse::TypeDeclaration => todo!(),
                CustomRecurse::MethodDeclaration => todo!(),
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
        let context = ContextV7::new();
        let root = tree.root_node();
        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            walk(child, &mut extracts, code, &context);
        }

        extracts
    }
}
