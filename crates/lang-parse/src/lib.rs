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
