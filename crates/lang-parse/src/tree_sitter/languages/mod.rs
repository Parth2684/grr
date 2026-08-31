use strum::{EnumString, VariantNames};
use tree_sitter::{Language, LanguageError, Node, Parser, Tree};
use uuid::Uuid;

use crate::tree_sitter::languages::helpers::kind::Kind;
mod helpers;

pub mod rust;
pub mod javascript;
pub mod typescript;
pub mod tsx;
pub mod python;
pub mod go;
pub mod sql;
pub mod html;
pub mod css;
pub mod c;
pub mod cpp;
pub mod zig;

#[derive(Debug)]
pub enum CustomError {
    LanguageError(LanguageError),
    String(String)
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


#[derive(VariantNames, EnumString)]
enum LevelOne {
    #[strum(serialize = "if_statement")] IfStatement,  
    #[strum(serialize = "#if")] If,
    #[strum(serialize = "preproc_if")] PreprocIf,
    #[strum(serialize = "preproc_ifdef")] PreprocIfdef,
    #[strum(serialize = "namespace_definition")] NamespaceDefinition,
    #[strum(serialize = "enum_specifier")] EnumSpecifier,
    #[strum(serialize = "rule_set")] RuleSet,
    #[strum(serialize = "media_statement")] MediaStatement,
    #[strum(serialize = "const_declaration")] ConstDeclaration,
    #[strum(serialize = "var_declaration")] VarDeclaration,
    #[strum(serialize = "try_statement")] TryStatement,
    #[strum(serialize = "static_item")] StaticItem,
    #[strum(serialize = "const_item")] ConstItem,
    #[strum(serialize = "statement")] Statement,
    #[strum(serialize = "else_statement")] ElseStatement,
    #[strum(serialize = "ambient_declaration")] AmbientDeclaration,
}



enum Recurse {

}



struct Extracted {
    id: Uuid,
    kind: Kind,
    code: String,
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    parent: Option<Uuid>
}


fn help_extract (child: &Node) -> Extracted {
    // let kind = Kind::from_str(child.kind()).unwrap();
    todo!();
    Extracted { id: (), kind, code: (), start_line: (), start_column: (), end_line: (), end_column: (), parent: () }
}


pub trait Extract {
    fn extract(tree: &Tree) {
        let mut extracts = Vec::new();
        let root = tree.root_node();
        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            if Skip::VARIANTS.contains(&child.kind()) {
                let kind = Kind::from(value)
            }
        }
    }
}
