use std::path::PathBuf;

use cli_table::{Cell, CellStruct, Style, Table, format::{Border, Justify}, print_stdout};
use common::tokei::count_loc;




pub fn count_lines (path: String, exclude: Vec<String>) {
    let path: PathBuf = PathBuf::from(path);
    let exclude_strs: Vec<&str> = exclude
        .iter()
        .map(|s| s.as_str()) // or .map(AsRef::as_ref)
        .collect();
    let locs = count_loc(path, exclude_strs);
    let table: Vec<Vec<CellStruct>> = locs
        .into_iter()
        .map(|loc| {
            vec![
                loc.language.cell().justify(Justify::Center),
                loc.files.cell().justify(Justify::Center),
                loc.code.cell().justify(Justify::Center),
                loc.comments.cell().justify(Justify::Center),
                loc.blanks.cell().justify(Justify::Center),
                loc.lines.cell().justify(Justify::Center),
            ]
        })
        .collect();
    let table = table
        .table()
        .title(vec![
            "Language".cell().bold(true),
            "Files".cell().bold(true),
            "Code".cell().bold(true),
            "Comments".cell().bold(true),
            "Blanks".cell().bold(true),
            "Total".cell().bold(true),
        ])
        .border(Border::builder().build());
    print_stdout(table).ok();
}