
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString, VariantNames};


#[derive(VariantNames, EnumString, AsRefStr)]
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


#[derive(Debug, Serialize, Deserialize, VariantNames, EnumString)]
#[serde(rename_all="snake_case")]
pub enum Kind {
    Struct,
    ConstVar,
    Impl,
    Type,
    Function,
    IfElse,
    Enum,
    Declaration,
    Macro,
    Trait,
    Test,
    HtmlCss,
    Sql
}

impl Kind {
    pub fn from_level_one(r#type: &str) -> Option<Self> {
        match r#type.parse::<LevelOne>() {
            Ok(LevelOne::AmbientDeclaration) => Some(Self::Declaration),
            Ok(LevelOne::ConstDeclaration) => Some(Self::ConstVar),
            Ok(LevelOne::ConstItem) => Some(Self::ConstVar),
            Ok(LevelOne::ElseStatement) => Some(Self::IfElse),
            Ok(LevelOne::EnumSpecifier) => Some(Self::Enum),
            Ok(LevelOne::If) => Some(Self::IfElse),
            Ok(LevelOne::IfStatement) => Some(Self::IfElse),
            Ok(LevelOne::MediaStatement) => Some(Self::HtmlCss),
            Ok(LevelOne::NamespaceDefinition) => Some(Self::Declaration),
            Ok(LevelOne::PreprocIf) => Some(Self::IfElse),
            Ok(LevelOne::PreprocIfdef) => Some(Self::IfElse),
            Ok(LevelOne::RuleSet) => Some(Self::HtmlCss),
            Ok(LevelOne::Statement) => Some(Self::Sql),
            Ok(LevelOne::StaticItem) => Some(Self::ConstVar),
            Ok(LevelOne::TryStatement) => Some(Self::Function),
            Ok(LevelOne::VarDeclaration) => Some(Self::ConstVar),
            Err(_) => None
        }
    }
}