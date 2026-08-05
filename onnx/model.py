# 1
from transformers import AutoModel, AutoTokenizer

MODEL = "jinaai/jina-code-embeddings-0.5b"

model = AutoModel.from_pretrained(
    MODEL,
    trust_remote_code=True,
)

# Disable KV cache before export
model.config.use_cache = False
model.eval()

tokenizer = AutoTokenizer.from_pretrained(
    MODEL,
    trust_remote_code=True,
)

tokenizer.save_pretrained("tokenizer")