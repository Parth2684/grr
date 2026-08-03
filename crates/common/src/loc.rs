use std::path::Path;

use cli_table::Table;
use tokei::{Config, LanguageType, Languages};


#[derive(Debug, Table)]
pub struct Loc {
    pub language: LanguageType,
    pub files: usize,
    pub code: usize,
    pub comments: usize,
    pub blanks: usize,
    pub lines: usize
}

pub fn count_loc<P: AsRef<Path>>(src_path: P, exclude_pattern: Vec<&str>) -> Vec<Loc> {
    let config = Config::default();
    let mut languages = Languages::new();
    languages.get_statistics(&[src_path], &exclude_pattern, &config);
    let mut loc = Vec::new();
    for (language, stats) in &languages {
        loc.push( Loc { 
            language: language.to_owned(),
            files: stats.reports.len(),
            code: stats.code,
            comments: stats.comments,
            blanks: stats.blanks,
            lines: stats.lines()
        });
    }
    loc
}


// run test with -- --nocapture
#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::loc::count_loc;
    
    #[test]
    fn test_count_loc() {
        let loc = count_loc(PathBuf::from_str("./").unwrap(), vec![]);
        dbg!(loc);
    }
}
