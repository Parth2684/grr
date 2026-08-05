# 2
from pathlib import Path

import torch
import torch.nn as nn
import torch.nn.functional as F
from transformers import AutoModel, AutoTokenizer

MODEL = "jinaai/jina-code-embeddings-0.5b"
OUTPUT = "original/embedding.onnx"


class EmbeddingWrapper(nn.Module):
    def __init__(self, model):
        super().__init__()
        self.model = model
        self.model.config.use_cache = False

    def forward(
        self,
        input_ids,
        attention_mask,
        position_ids,
    ):
        outputs = self.model(
            input_ids=input_ids,
            attention_mask=attention_mask,
            position_ids=position_ids,
            use_cache=False,
            return_dict=True,
        )

        hidden = outputs.last_hidden_state

        last = attention_mask.sum(dim=1) - 1

        batch = torch.arange(
            hidden.shape[0],
            device=hidden.device,
        )

        embedding = hidden[
            batch,
            last,
        ]

        embedding = F.normalize(
            embedding,
            p=2,
            dim=1,
        )

        return embedding


print("Loading model...")


# Create a single folder directly in the CWD
Path("original").mkdir(exist_ok=True)

model = AutoModel.from_pretrained(
    MODEL,
    trust_remote_code=True,
).eval()

tokenizer = AutoTokenizer.from_pretrained(
    MODEL,
    trust_remote_code=True,
)

wrapper = EmbeddingWrapper(model).eval()

text = ['fn main() { println!("hello"); }']

inputs = tokenizer(
    text,
    return_tensors="pt",
)

seq = inputs["input_ids"].shape[1]

position_ids = torch.arange(seq).unsqueeze(0).expand(inputs["input_ids"].shape[0], -1)

print("Exporting...")

torch.onnx.export(
    wrapper,
    (
        inputs["input_ids"],
        inputs["attention_mask"],
        position_ids,
    ),
    OUTPUT,
    input_names=[
        "input_ids",
        "attention_mask",
        "position_ids",
    ],
    output_names=[
        "embeddings",
    ],
    dynamic_axes={
        "input_ids": {
            0: "batch",
            1: "sequence",
        },
        "attention_mask": {
            0: "batch",
            1: "sequence",
        },
        "position_ids": {
            0: "batch",
            1: "sequence",
        },
        "embeddings": {
            0: "batch",
        },
    },
    export_params=True,
    opset_version=18,
    do_constant_folding=True,
)

print("Done.")
