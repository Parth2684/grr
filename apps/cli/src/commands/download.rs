use embeddings::{Precision, recommend::get_recommendation};
use inquire::MultiSelect;

pub fn download_command_interactive() {
    let recommendation = get_recommendation();
    let default: usize = match recommendation {
        Precision::Fp32 => 0,
        Precision::Fp16 => 1,
        Precision::Int8 => 2,
    };
    let options = [
        (
            "FP32",
            Precision::Fp32,
            "Highest accuracy • Size: 1.98 GB • Highest memory usage • Slightly Slower on GPU than fp16",
        ),
        (
            "FP16",
            Precision::Fp16,
            "Near-FP32 accuracy • Size: 988 MB • Lower memory usage • Slightly Slower on CPU than fp32",
        ),
        (
            "INT8",
            Precision::Int8,
            "Smallest model • Size: 495 MB • May reduce accuracy • Fastest on CPU but least accurate | Only use if hardware is really bad",
        ),
    ]
    .into_iter()
    .map(|(name, precision, description)| {
        if precision == recommendation {
            format!("{name} (recommended) — {description}")
        } else {
            format!("{name} — {description}")
        }
    })
    .collect::<Vec<_>>();

    let some = MultiSelect::new("Select Model Precision for Vector embeddings", options)
        .with_default(&[default])
        .with_help_message("Use Space to select, Enter to confirm")
        .prompt()
        .unwrap();
    println!("{:?}", some);
}
