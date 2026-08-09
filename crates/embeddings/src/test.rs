
use std::time::Instant;

use ndarray::Array2;
use ort::{
    ep::{MIGraphX},
    inputs,
    session::Session,
    value::Tensor,
};
use tokenizers::Tokenizer;

const WARMUP_RUNS: usize = 5;
const BENCHMARK_RUNS: usize = 20;

const PARA: &str = "Artificial intelligence has rapidly evolved from a niche area of computer science into one of the most influential technologies shaping modern society. What once seemed like science fiction is now integrated into everyday applications, from voice assistants and recommendation systems to medical research, autonomous vehicles, cybersecurity, and software development. At its core, artificial intelligence involves creating systems capable of performing tasks that traditionally require human intelligence, such as understanding language, recognizing patterns, making predictions, solving problems, and learning from data. One particularly important area of AI is natural language processing, which allows computers to work with human language in increasingly sophisticated ways. Modern language models can analyze large amounts of text, summarize documents, answer questions, translate languages, generate content, and even assist programmers with writing and debugging code. Behind many of these applications are neural networks trained on enormous datasets. These networks learn statistical patterns within the data and use those patterns to produce useful outputs when presented with new information. Another important technology closely related to language processing is vector embeddings. An embedding model converts text into numerical vectors that represent the semantic meaning of the original content. Texts with similar meanings tend to produce vectors that are close together in a high-dimensional mathematical space. This makes embeddings extremely useful for search engines, recommendation systems, retrieval-augmented generation, document classification, clustering, and other applications where understanding meaning is more important than simply matching individual words. For example, a document discussing how to repair a computer could still be retrieved when someone searches for information about fixing a laptop, even if the exact words used in the document and query are different. Retrieval-augmented generation systems take this concept further by combining vector search with generative language models. Instead of relying entirely on information stored inside a model's parameters, a RAG system can retrieve relevant information from an external database and provide that information to the language model as context. This approach can make AI applications more useful for private documents, technical documentation, company knowledge bases, source code repositories, and frequently changing information. However, building efficient AI systems also requires careful consideration of performance and hardware limitations. Large models can require significant amounts of memory and computational power, while smaller or quantized models can often run efficiently on consumer hardware. Different numerical precisions, such as FP32, FP16, BF16, and INT8, provide different trade-offs between accuracy, memory usage, and performance. Hardware acceleration through GPUs can dramatically improve inference speed when the required execution provider and model operations are properly supported. Nevertheless, simply having a GPU does not guarantee that a model will automatically run on it. The software stack, drivers, model format, operators, and execution provider all need to work together correctly. This is why technologies such as ONNX and ONNX Runtime are valuable: they provide standardized ways to represent and execute machine-learning models across different hardware platforms. As AI continues to develop, understanding these underlying technologies becomes increasingly important for developers who want to build practical, efficient, and reliable applications rather than simply using AI as a black box.";


struct BenchmarkResult {
    name: &'static str,
    cold_ns: u128,
    average_ns: u128,
    embedding: Vec<f32>,
}


fn benchmark_model(
    name: &'static str,
    model_path: &str,
    input_ids: &Array2<i64>,
    attention_mask: &Array2<i64>,
    position_ids: &Array2<i64>,
) -> BenchmarkResult {
    println!("\n========================================");
    println!("Running {}", name);
    println!("========================================");

    // --------------------------------------------------------
    // COLD START
    //
    // This includes:
    //   - Session creation
    //   - Model loading/compilation
    //   - First inference
    //
    // It does NOT represent pure inference performance.
    // --------------------------------------------------------

    let cold_start = Instant::now();

    let mut session = Session::builder()
        .unwrap()
        .with_execution_providers([
            MIGraphX::default()
                .build()
                .error_on_failure(),
        ])
        .unwrap()
        .commit_from_file(model_path)
        .unwrap();

    let embedding_vec = {
        let output = session
            .run(inputs![
                "input_ids" => Tensor::from_array(input_ids.clone()).unwrap(),
                "attention_mask" => Tensor::from_array(attention_mask.clone()).unwrap(),
                "position_ids" => Tensor::from_array(position_ids.clone()).unwrap(),
            ])
            .unwrap();
    
        let embeddings = output["embeddings"]
            .try_extract_tensor::<f32>()
            .unwrap();
    
        embeddings.1.to_vec()
    };
    
    let cold_ns = cold_start.elapsed().as_nanos();

    // --------------------------------------------------------
    // WARMUP
    // --------------------------------------------------------

    for _ in 0..WARMUP_RUNS {
        let _ = session
            .run(inputs![
                "input_ids" => Tensor::from_array(input_ids.clone()).unwrap(),
                "attention_mask" => Tensor::from_array(attention_mask.clone()).unwrap(),
                "position_ids" => Tensor::from_array(position_ids.clone()).unwrap(),
            ])
            .unwrap();
    }

    // --------------------------------------------------------
    // BENCHMARK
    // --------------------------------------------------------

    let benchmark_start = Instant::now();

    for _ in 0..BENCHMARK_RUNS {
        let _ = session
            .run(inputs![
                "input_ids" => Tensor::from_array(input_ids.clone()).unwrap(),
                "attention_mask" => Tensor::from_array(attention_mask.clone()).unwrap(),
                "position_ids" => Tensor::from_array(position_ids.clone()).unwrap(),
            ])
            .unwrap();
    }

    let total_ns = benchmark_start.elapsed().as_nanos();

    let average_ns = total_ns / BENCHMARK_RUNS as u128;

    println!("Cold start:       {} ns", cold_ns);
    println!("Cold start:       {:.3} s", cold_ns as f64 / 1_000_000_000.0);

    println!("Benchmark total:  {} ns", total_ns);
    println!("Average inference: {} ns", average_ns);
    println!(
        "Average inference: {:.3} ms",
        average_ns as f64 / 1_000_000.0
    );

    println!("Embedding size:   {}", embedding_vec.len());

    BenchmarkResult {
        name,
        cold_ns,
        average_ns,
        embedding: embedding_vec,
    }
}


// ------------------------------------------------------------
// Cosine similarity
// ------------------------------------------------------------

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(
        a.len(),
        b.len(),
        "Embedding dimensions do not match"
    );

    let mut dot = 0.0f64;
    let mut norm_a = 0.0f64;
    let mut norm_b = 0.0f64;

    for (x, y) in a.iter().zip(b.iter()) {
        let x = *x as f64;
        let y = *y as f64;

        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    let denominator = norm_a.sqrt() * norm_b.sqrt();

    if denominator == 0.0 {
        return 0.0;
    }

    (dot / denominator) as f32
}


// ------------------------------------------------------------
// Percentage difference
// ------------------------------------------------------------

fn percentage_difference(fp32: u128, other: u128) -> f64 {
    ((other as f64 - fp32 as f64) / fp32 as f64) * 100.0
}


// ------------------------------------------------------------
// Main test
// ------------------------------------------------------------

pub fn test_session() {
    println!("Preparing tokenizer...");

    let tokenizer =
        Tokenizer::from_file("../../onnx/tokenizer/tokenizer.json")
            .unwrap();

    let encoding = tokenizer
        .encode(PARA, true)
        .unwrap();

    let input_ids: Vec<i64> = encoding
        .get_ids()
        .iter()
        .map(|&x| x as i64)
        .collect();

    let attention_mask: Vec<i64> = encoding
        .get_attention_mask()
        .iter()
        .map(|&x| x as i64)
        .collect();

    let position_ids: Vec<i64> = (0..input_ids.len())
        .map(|x| x as i64)
        .collect();

    let seq_len = input_ids.len();

    println!("Token count: {}", seq_len);
    println!("Text length: {} words approximately", PARA.split_whitespace().count());

    let input_ids =
        Array2::from_shape_vec((1, seq_len), input_ids)
            .unwrap();

    let attention_mask =
        Array2::from_shape_vec((1, seq_len), attention_mask)
            .unwrap();

    let position_ids =
        Array2::from_shape_vec((1, seq_len), position_ids)
            .unwrap();


    // --------------------------------------------------------
    // Run all three models
    // --------------------------------------------------------

    let fp32 = benchmark_model(
        "FP32",
        "../../onnx/quantized/embedding_fp32.onnx",
        &input_ids,
        &attention_mask,
        &position_ids,
    );

    let fp16 = benchmark_model(
        "FP16",
        "../../onnx/quantized/embedding_fp16.onnx",
        &input_ids,
        &attention_mask,
        &position_ids,
    );

    let int8 = benchmark_model(
        "INT8",
        "../../onnx/quantized/embedding_int8_dynamic.onnx",
        &input_ids,
        &attention_mask,
        &position_ids,
    );


    // --------------------------------------------------------
    // SPEED COMPARISON
    // --------------------------------------------------------

    println!("\n\n");
    println!("============================================================");
    println!("                    PERFORMANCE");
    println!("============================================================");

    println!(
        "{:<10} {:>15} {:>15}",
        "Model",
        "Cold (ms)",
        "Average (ms)"
    );

    println!(
        "{:<10} {:>15.3} {:>15.3}",
        fp32.name,
        fp32.cold_ns as f64 / 1_000_000.0,
        fp32.average_ns as f64 / 1_000_000.0
    );

    println!(
        "{:<10} {:>15.3} {:>15.3}",
        fp16.name,
        fp16.cold_ns as f64 / 1_000_000.0,
        fp16.average_ns as f64 / 1_000_000.0
    );

    println!(
        "{:<10} {:>15.3} {:>15.3}",
        int8.name,
        int8.cold_ns as f64 / 1_000_000.0,
        int8.average_ns as f64 / 1_000_000.0
    );


    // --------------------------------------------------------
    // SPEEDUP
    // --------------------------------------------------------

    let fp16_speedup =
        fp32.average_ns as f64 / fp16.average_ns as f64;

    let int8_speedup =
        fp32.average_ns as f64 / int8.average_ns as f64;

    println!("\n============================================================");
    println!("                    SPEEDUP VS FP32");
    println!("============================================================");

    println!(
        "FP16: {:.2}x",
        fp16_speedup
    );

    println!(
        "INT8: {:.2}x",
        int8_speedup
    );

    println!(
        "FP16 time difference: {:.2}%",
        percentage_difference(fp32.average_ns, fp16.average_ns)
    );

    println!(
        "INT8 time difference: {:.2}%",
        percentage_difference(fp32.average_ns, int8.average_ns)
    );


    // --------------------------------------------------------
    // COSINE SIMILARITY
    //
    // FP32 is treated as the reference.
    // --------------------------------------------------------

    let fp16_similarity =
        cosine_similarity(&fp32.embedding, &fp16.embedding);

    let int8_similarity =
        cosine_similarity(&fp32.embedding, &int8.embedding);

    let fp16_distance =
        1.0 - fp16_similarity;

    let int8_distance =
        1.0 - int8_similarity;

    println!("\n============================================================");
    println!("                 EMBEDDING ACCURACY");
    println!("============================================================");

    println!("FP32 embedding dimensions: {}", fp32.embedding.len());

    println!(
        "FP32 vs FP16 cosine similarity: {:.8}",
        fp16_similarity
    );

    println!(
        "FP32 vs INT8 cosine similarity: {:.8}",
        int8_similarity
    );

    println!(
        "FP32 vs FP16 cosine distance:   {:.8}",
        fp16_distance
    );

    println!(
        "FP32 vs INT8 cosine distance:   {:.8}",
        int8_distance
    );


    // --------------------------------------------------------
    // FINAL SUMMARY
    // --------------------------------------------------------

    println!("\n============================================================");
    println!("                       SUMMARY");
    println!("============================================================");

    println!(
        "FP32 average: {:.3} ms",
        fp32.average_ns as f64 / 1_000_000.0
    );

    println!(
        "FP16 average: {:.3} ms ({:.2}x vs FP32)",
        fp16.average_ns as f64 / 1_000_000.0,
        fp16_speedup
    );

    println!(
        "INT8 average: {:.3} ms ({:.2}x vs FP32)",
        int8.average_ns as f64 / 1_000_000.0,
        int8_speedup
    );

    println!(
        "FP16 similarity to FP32: {:.6}",
        fp16_similarity
    );

    println!(
        "INT8 similarity to FP32: {:.6}",
        int8_similarity
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpu_work() {
        test_session();
        assert_eq!(0, 0);
    }
}

