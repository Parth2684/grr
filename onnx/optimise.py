# 3
from pathlib import Path

import onnxruntime as ort
from onnxsim import simplify

import onnx

INPUT = "original/embedding.onnx"

SIMPLIFIED = "simplified/embedding_sim.onnx"
Path("simplified").mkdir(exist_ok=True)

print("=" * 60)
print("Loading model...")
print("=" * 60)

model = onnx.load(
    INPUT,
    load_external_data=True,
)

print("Simplifying...")

model_sim, check = simplify(model)

if not check:
    raise RuntimeError("Simplification failed.")

onnx.save_model(
    model_sim,
    SIMPLIFIED,
    save_as_external_data=True,
    all_tensors_to_one_file=True,
    location="embedding_sim.data.onnx",
)

print("Saved simplified model.")


print()
print("Done.")
