#!/usr/bin/env bash

set -euo pipefail

python -m venv .venv

source .venv/bin/activate

pip install -r requirements.txt

echo
echo "Environment ready."

echo "============================================================"
echo "grr Embedding Model Build Pipeline"
echo "============================================================"

echo
echo "[1/4] Downloading HuggingFace model..."
python model.py

echo
echo "[2/4] Exporting embedding ONNX..."
python export_embeddings.py

echo
echo "[3/4] Simplifying ONNX graph..."
python optimise.py

echo
echo "[4/4] Generating quantized models..."
python quantize.py

echo
echo "============================================================"
echo "Build completed successfully!"
echo "============================================================"

echo
echo "Generated files:"
echo "  original/"
echo "    embedding.onnx"
echo "    embedding.onnx.data"
echo
echo "  simplified/"
echo "    embedding_sim.onnx"
echo "    embedding_sim.data.onnx"
echo
echo "  models/"
echo "    embedding_fp32.onnx"
echo "    embedding_fp32.onnx.data"
echo "    embedding_fp16.onnx"
echo "    embedding_fp16.onnx.data"
echo "    embedding_int8_dynamic.onnx"

