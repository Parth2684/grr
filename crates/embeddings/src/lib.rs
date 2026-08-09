
mod test;
pub mod recommend;


#[derive(PartialEq, Eq)]
pub enum Precision {
    Fp32,
    Fp16,
    Int8
}