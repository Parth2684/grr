use tree_sitter::Tree;

use crate::tree_sitter::languages::CustomError;
use crate::tree_sitter::languages::Extract;

use super::languages::rust as Rust;
use super::languages::javascript as JavaScript;
use super::languages::typescript as TypeScript;
use super::languages::tsx as Tsx;
use super::languages::python as Python;
use super::languages::go as Go;
use super::languages::sql as Sql;
use super::languages::html as Html;
use super::languages::css as Css;
use super::languages::c as Clang;
use super::languages::cpp as Cpp;
use super::languages::zig as Zig;

pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Tsx,
    Python,
    Go,
    Sql,
    Html,
    Css,
    Clang,
    Cpp,
    Zig,
}

impl Language {
    pub fn parse(&self, code: &str) -> Result<Tree, CustomError>{
        match self {
            Self::Rust => Rust::parse(code),
            Self::JavaScript => JavaScript::parse(code),
            Self::TypeScript => TypeScript::parse(code),
            Self::Tsx => Tsx::parse(code),
            Self::Python => Python::parse(code),
            Self::Go => Go::parse(code),
            Self::Sql => Sql::parse(code),
            Self::Html => Html::parse(code),
            Self::Css => Css::parse(code),
            Self::Clang => Clang::parse(code),
            Self::Cpp => Cpp::parse(code),
            Self::Zig => Zig::parse(code),
        }
    }
}


impl Extract for Language {
    
}

