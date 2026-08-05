# Embedding Benchmark

Generated: 2026-08-05 19:31:32.870984

## Environment

- Execution Provider: CPUExecutionProvider
- PyTorch Device: cpu
- Benchmark Runs: 100
- Warmup Runs: 10

## Summary

|Rank|Model|Format|Size (MB)|Load (ms)|Cold (ms)|Warm (ms)|Emb/s|RSS (MB)|Cosine|
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|
|1|embedding_int8_dynamic|Embedding|471.65|1993.91|123.22|123.42|8.10|3689.55|0.8685634136|
|2|embedding_fp16|Embedding|1884.80|1999.43|148.82|151.81|6.59|4171.31|0.9998064041|
|3|embedding_sim|Embedding|0.25|2899.05|186.07|182.42|5.48|3613.19|1.0000000000|
|5|embedding_fp32|Embedding|1884.81|2748.33|182.80|184.56|5.42|3754.42|1.0000000000|


# embedding_int8_dynamic

## Information

- Path: `models/embedding_int8_dynamic.onnx`
- Output Format: Embedding
- Model Size: 471.65 MB

## Performance

|Metric|Value|
|---|---:|
|Load|1993.91 ms|
|Cold|123.22 ms|
|Warm|123.42 ms|
|Embeddings/sec|8.10|
|RSS|3689.55 MB|

## Accuracy

|Metric|Value|
|---|---:|
|Cosine|0.8685634136|
|Max Difference|0.08268744|
|Mean Difference|0.01352405|
|L2 Distance|0.51271147|

### Largest Differences

|Dimension|Difference|
|---:|---:|
|47|0.08268744|
|383|0.06017057|
|438|0.05403728|
|545|0.05155816|
|255|0.04986805|
|221|0.04945890|
|766|0.04871403|
|861|0.04844216|
|243|0.04801890|
|736|0.04695419|

---


# embedding_sim

## Information

- Path: `simplified/embedding_sim.onnx`
- Output Format: Embedding
- Model Size: 0.25 MB

## Performance

|Metric|Value|
|---|---:|
|Load|2899.05 ms|
|Cold|186.07 ms|
|Warm|182.42 ms|
|Embeddings/sec|5.48|
|RSS|3613.19 MB|

## Accuracy

|Metric|Value|
|---|---:|
|Cosine|1.0000000000|
|Max Difference|0.00000022|
|Mean Difference|0.00000004|
|L2 Distance|0.00000156|

### Largest Differences

|Dimension|Difference|
|---:|---:|
|690|0.00000022|
|798|0.00000020|
|54|0.00000018|
|741|0.00000016|
|621|0.00000016|
|61|0.00000015|
|427|0.00000015|
|549|0.00000015|
|360|0.00000014|
|39|0.00000014|

---

# embedding_fp32

## Information

- Path: `models/embedding_fp32.onnx`
- Output Format: Embedding
- Model Size: 1884.81 MB

## Performance

|Metric|Value|
|---|---:|
|Load|2748.33 ms|
|Cold|182.80 ms|
|Warm|184.56 ms|
|Embeddings/sec|5.42|
|RSS|3754.42 MB|

## Accuracy

|Metric|Value|
|---|---:|
|Cosine|1.0000000000|
|Max Difference|0.00000022|
|Mean Difference|0.00000004|
|L2 Distance|0.00000156|

### Largest Differences

|Dimension|Difference|
|---:|---:|
|690|0.00000022|
|798|0.00000020|
|54|0.00000018|
|741|0.00000016|
|621|0.00000016|
|61|0.00000015|
|427|0.00000015|
|549|0.00000015|
|360|0.00000014|
|39|0.00000014|

---

# embedding

## Information

- Path: `original/embedding.onnx`
- Output Format: Embedding
- Model Size: 1887.71 MB

## Performance

|Metric|Value|
|---|---:|
|Load|3768.67 ms|
|Cold|184.88 ms|
|Warm|186.74 ms|
|Embeddings/sec|5.36|
|RSS|3626.37 MB|

## Accuracy

|Metric|Value|
|---|---:|
|Cosine|1.0000000000|
|Max Difference|0.00000022|
|Mean Difference|0.00000004|
|L2 Distance|0.00000156|

### Largest Differences

|Dimension|Difference|
|---:|---:|
|690|0.00000022|
|798|0.00000020|
|54|0.00000018|
|741|0.00000016|
|621|0.00000016|
|61|0.00000015|
|427|0.00000015|
|549|0.00000015|
|360|0.00000014|
|39|0.00000014|

---

# embedding_fp16

## Information

- Path: `models/embedding_fp16_repaired.onnx`
- Output Format: Embedding
- Model Size: 942.52 MB

## Performance

|Metric|Value|
|---|---:|
|Load|3195.94 ms|
|Cold|173.52 ms|
|Warm|187.30 ms|
|Embeddings/sec|5.34|
|RSS|4204.14 MB|

## Accuracy

|Metric|Value|
|---|---:|
|Cosine|0.9998064041|
|Max Difference|0.00009135|
|Mean Difference|0.00002241|
|L2 Distance|0.00083608|

### Largest Differences

|Dimension|Difference|
|---:|---:|
|601|0.00009135|
|513|0.00008408|
|115|0.00008136|
|427|0.00008062|
|403|0.00007974|
|649|0.00007833|
|741|0.00007714|
|101|0.00007680|
|493|0.00007320|
|423|0.00007247|

---

# Conclusions

**Fastest:** embedding_int8_dynamic

**Smallest:** embedding_sim

**Most Accurate:** embedding_sim

