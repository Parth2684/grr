use serde::{Deserialize, Serialize};
use strum::{EnumString, VariantNames};

#[derive(Serialize, Deserialize, VariantNames, EnumString)]
#[serde(rename_all="snake_case")]
pub enum Kind {
    Struct,
    Impl,
    Type,
    Function,
    IfElse,
    Enum,
    Declaration(String),
    TryStatement,
    Macro,
    Trait,
    Test,
    Custom(String),
}
