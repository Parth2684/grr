# 4
import shutil
import warnings
from pathlib import Path

from onnxconverter_common import float16
from onnxruntime.quantization import (
    QuantType,
    quantize_dynamic,
)

import onnx

warnings.filterwarnings("ignore")

SOURCE = Path("simplified/embedding_sim.onnx")
OUT = Path("quantized")
OUT.mkdir(exist_ok=True)

# ---------------------------------------------------
# Copy FP32
# ---------------------------------------------------

# print("Generating FP32...")
# # Copy ONNX graph
# shutil.copy2(
#     SOURCE,
#     OUT / "embedding_fp32.onnx",
# )

# # Read model and copy every referenced external data file
# model = onnx.load(
#     SOURCE,
#     load_external_data=True,
# )

# onnx.save_model(
#     model,
#     OUT / "embedding_fp32.onnx",
#     save_as_external_data=True,
#     all_tensors_to_one_file=True,
#     location="embedding_fp32.onnx.data",
# )

# print("Done.")


# ---------------------------------------------------
# FP16
# ---------------------------------------------------

print("Generating FP16...")

model = onnx.load(
    SOURCE,
    load_external_data=True,
)

model = float16.convert_float_to_float16(
    model,
    keep_io_types=True,
)

from onnx import TensorProto

for node in model.graph.node:
    if node.op_type != "Cast":
        continue

    out = node.output[0]

    expected = None
    for vi in list(model.graph.value_info) + list(model.graph.output):
        if vi.name == out:
            expected = vi.type.tensor_type.elem_type
            break

    if expected is None:
        continue

    for attr in node.attribute:
        if attr.name == "to" and attr.i != expected:
            print(f"Fixing {node.name}: {attr.i} -> {expected}")
            attr.i = expected

onnx.save_model(
    model,
    OUT / "embedding_fp16.onnx",
    save_as_external_data=True,
    all_tensors_to_one_file=True,
    location="embedding_fp16.onnx.data",
)

print("Done.")

# ---------------------------------------------------
# Dynamic INT8
# ---------------------------------------------------

# print("Generating Dynamic INT8...")

# quantize_dynamic(
#     model_input=str(SOURCE),
#     model_output=str(OUT / "embedding_int8_dynamic.onnx"),
#     weight_type=QuantType.QInt8,
# )

# print("Done.")

# # ---------------------------------------------------
# # Sizes
# # ---------------------------------------------------

# print("\nModel sizes\n")

# for model in sorted(OUT.glob("embedding_*")):
#     print(f"{model.name:<35}{model.stat().st_size / (1024**2):8.2f} MB")
