use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString, VariantNames};


#[derive(Debug, VariantNames, EnumString)]
pub enum LevelOne {
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
    #[strum(serialize = "variable_declaration")] VariableDeclaration, 
}

#[derive(Debug, VariantNames, EnumString)]
pub enum CustomRecurse {
    #[strum(serialize = "class_specifier")] ClassSpecifier,
    #[strum(serialize = "struct_specifier")] StructSpecifier, // C++
    #[strum(serialize = "type_declaration")] TypeDeclaration, 
    #[strum(serialize = "method_declaration")] MethodDeclaration, // go
    #[strum(serialize = "struct_item")] StructItem,
    #[strum(serialize = "impl_item")] ImplItem,
    #[strum(serialize = "trait_item")] TraitItem, //rust
    #[strum(serialize = "class_declaration")] ClassDeclaration, 
    #[strum(serialize = "public_field_definition")] PublicFieldDefinition, //group
    #[strum(serialize = "method_definition")] MethodeDefinition, //ts
    #[strum(serialize = "element")] HtmlElement, //html
}

#[derive(Debug, VariantNames, EnumString)]
pub enum Recurse {
    #[strum(serialize = "function_definition")] FunctionDefinition,
    #[strum(serialize = "function_declaration")] FunctionDeclaration,
    #[strum(serialize = "function_item")] FunctionItem,
}

#[derive(Debug, VariantNames, EnumString)]
pub enum StopRecurse {
    #[strum(serialize = "decorated_definition")] DecoratedDefinition,
    #[strum(serialize = "macro_definition")] MacroDefinition,
    #[strum(serialize = "enum_item")] EnumItem,
    #[strum(serialize = "type_item")] TypeItem,
    #[strum(serialize = "const_item")] ConstItem, //group
    #[strum(serialize = "expression_statement")] ExpressionStatement, //group
    #[strum(serialize = "arrow_function")] ArrowFunction,
    #[strum(serialize = "interface_declaration")] InterfaceDeclaration,
    #[strum(serialize = "type_alias_declaration")] TypeAliasDeclaration,
    #[strum(serialize = "enum_declaration")] EnumDeclaration,
    #[strum(serialize = "comptime_declaration")] ComptimeDeclaration,
    #[strum(serialize = "test_declaration")] TestDeclaration,
    #[strum(serialize = "jsx_element")] JsxElement,
    #[strum(serialize = "attribute_item")] AttributeItem, //skip group with next node
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
    Sql,
    Skip,
    Method,
    JsxTsx
}

impl Kind {
    pub fn from_level_one(r#type: &LevelOne) -> Self{
        match r#type {
            LevelOne::AmbientDeclaration => Self::Declaration,
            LevelOne::ConstDeclaration => Self::ConstVar,
            LevelOne::ConstItem => Self::ConstVar,
            LevelOne::ElseStatement => Self::IfElse,
            LevelOne::EnumSpecifier => Self::Enum,
            LevelOne::If => Self::IfElse,
            LevelOne::IfStatement => Self::IfElse,
            LevelOne::MediaStatement => Self::HtmlCss,
            LevelOne::NamespaceDefinition => Self::Declaration,
            LevelOne::PreprocIf => Self::IfElse,
            LevelOne::PreprocIfdef => Self::IfElse,
            LevelOne::RuleSet => Self::HtmlCss,
            LevelOne::Statement => Self::Sql,
            LevelOne::StaticItem => Self::ConstVar,
            LevelOne::TryStatement => Self::Function,
            LevelOne::VarDeclaration => Self::ConstVar,
            LevelOne::VariableDeclaration => Self::ConstVar,
        }
    }

    pub fn from_recurse(r#type: &Recurse) -> Self {
        match r#type {
            Recurse::FunctionDeclaration => Self::Function,
            Recurse::FunctionDefinition => Self::Function,
            Recurse::FunctionItem => Self::Function,
        }
    }

    pub fn from_stop_recurse(r#type: &StopRecurse) -> Self {
        match r#type {
            StopRecurse::DecoratedDefinition => Self::Function,
            StopRecurse::MacroDefinition => Self::Macro,
            StopRecurse::EnumItem => Self::Enum,
            StopRecurse::TypeItem => Self::Type,
            StopRecurse::ConstItem => Self::ConstVar,
            StopRecurse::ExpressionStatement => Self::Function,
            StopRecurse::ArrowFunction => Self::Function,
            StopRecurse::InterfaceDeclaration => Self::Type,
            StopRecurse::TypeAliasDeclaration => Self::Type,
            StopRecurse::EnumDeclaration => Self::Enum,
            StopRecurse::ComptimeDeclaration => Self::Declaration,
            StopRecurse::TestDeclaration => Self::Function,
            StopRecurse::JsxElement => Self::JsxTsx,
            StopRecurse::AttributeItem => Self::Skip
        }
    }

    pub fn from_custom_recurse(r#type: &CustomRecurse) -> Self {
        match r#type {
            CustomRecurse::ClassSpecifier => Self::Skip,
            CustomRecurse::StructSpecifier => Self::Struct,
            CustomRecurse::TypeDeclaration => Self::Type,
            CustomRecurse::MethodDeclaration => Self::Method,
            CustomRecurse::StructItem => Self::Struct,
            CustomRecurse::ImplItem => Self::Skip,
            CustomRecurse::TraitItem => Self::Skip,
            CustomRecurse::ClassDeclaration => Self::Skip,
            CustomRecurse::PublicFieldDefinition => Self::Struct,
            CustomRecurse::MethodeDefinition => Self::Method,
            CustomRecurse::HtmlElement => Self::HtmlCss,
        }
    }
}