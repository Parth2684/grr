pub mod download;
pub mod tree_sitter;

pub struct CodeRange {
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

pub enum FlexibleArray {
    SizeThree([u32; 3]),
    SizeFour([u32; 4]),
}

impl CodeRange {
    pub fn from_array(range: FlexibleArray) -> Self {
        match range {
            FlexibleArray::SizeThree([start_line, start_column, end_column]) => Self {
                start_line,
                start_column,
                end_line: start_line,
                end_column,
            },
            FlexibleArray::SizeFour([start_line, start_column, end_line, end_column]) => Self {
                start_line,
                start_column,
                end_line,
                end_column,
            },
        }
    }
}



#[cfg(test)]
mod tests {

use tokei::LanguageType;
use ::tree_sitter::Node;


fn test_tree_sitter(lang: &LanguageType, path: &str, file_name: &str, lang_parse: Language) {
    let langs = common::tokei::get_files(path);
    
        let files = langs
            .get(lang)
            .into_iter()
            .flatten()
            .chain(
                match lang {
                    LanguageType::C => langs.get(&LanguageType::CHeader),
                    LanguageType::Cpp => {
                        langs.get(&LanguageType::CppHeader);
                        langs.get(&LanguageType::CHeader)
                    },
                    _ => None,
                }
                .into_iter()
                .flatten(),
            );

    
    let mut output = String::new();

    for file in files {
        let code = std::fs::read_to_string(file).unwrap();
        let tree = lang_parse.parse(&code).unwrap();

        output.push_str("════════════════════════════════════════════════════════════\n");
        output.push_str(&format!("FILE: {}\n", file.display()));
        output.push_str("════════════════════════════════════════════════════════════\n\n");

        write_tree(
            tree.root_node(),
            &mut output,
            "",
            true,
        );

        output.push_str("\n\n");
    }

    std::fs::write(file_name, output).unwrap();
}
use std::fmt::Write;

use crate::tree_sitter::prelude::Language;

fn write_tree(node: Node, output: &mut String, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };

    let start = node.start_position();
    let end = node.end_position();

    writeln!(
        output,
        "{}{} kind: {} [{}:{} → {}:{}]",
        prefix,
        connector,
        node.kind(),
        start.row + 1,
        start.column,
        end.row + 1,
        end.column,
    ).unwrap();

    let child_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();

    for (i, child) in children.iter().enumerate() {
        write_tree(
            *child,
            output,
            &child_prefix,
            i == children.len() - 1,
        );
    }
}
    
    #[test]
    fn test_rust() {
        test_tree_sitter(&LanguageType::Rust, "../../", "rust-trees.txt", Language::Rust);
    }

    #[test]
    fn test_typescript() {
        test_tree_sitter(&LanguageType::TypeScript, "../../../unicloud", "typescript-trees.txt", Language::TypeScript);
    }

    #[test]
    fn test_tsx () {
        test_tree_sitter(&LanguageType::Tsx, "../../../unicloud", "tsx-trees.txt", Language::Tsx);
    }

    #[test]
    fn test_sql () {
        test_tree_sitter(&LanguageType::Sql, "../../", "sql-trees.txt", Language::Sql);
    }

    #[test]
    fn test_python () {
        test_tree_sitter(&LanguageType::Python, "../../../test/request", "python-trees.txt", Language::Python);
    }
    
    #[test]
    fn test_c () {
        test_tree_sitter(&LanguageType::C, "../../../test/sds", "c-trees.txt", Language::Clang);
    }

    #[test]
    fn test_go() {
        test_tree_sitter(&LanguageType::Go, "../../../test/cobra", "go-trees.txt", Language::Go);
    }

    #[test]
    fn test_cpp() {
        test_tree_sitter(&LanguageType::Cpp, "../../../test/spdlog", "cpp-trees.txt", Language::Cpp);
    }

    #[test]
    fn test_html() {
        test_tree_sitter(&LanguageType::Html, "../../../test/github-markdown-css", "html-trees.txt", Language::Html);
    }

    #[test]
    fn test_css() {
        test_tree_sitter(&LanguageType::Css, "../../../test/github-markdown-css", "css-trees.txt", Language::Css);
    }

    #[test]
    fn test_zig() {
        test_tree_sitter(&LanguageType::Zig, "../../../test/libxev", "zig-trees.txt", Language::Zig);
    }
}