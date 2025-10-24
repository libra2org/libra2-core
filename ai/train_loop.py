# Copyright (c) 2025 Libra2 Contributors
# SPDX-License-Identifier: Apache-2.0
import os, json
import torch
import torch.nn as nn
import torch.optim as optim
from predictor import TinyMLP, export_onnx

BATCH = 512
EPOCHS = 2
LR = 1e-3
DATA_PATH = "ai/data/rounds.jsonl"
STATE_OUT = "ai/weights.pt"
ONNX_OUT = "ai/predictor.onnx"

def load_dataset(path=DATA_PATH):
    X, y = [], []
    if not os.path.exists(path):
        return None, None
    with open(path) as f:
        for line in f:
            r = json.loads(line)
            X.append([
                float(r.get("last_vote_delay_sec", 0.0)),
                float(r.get("avg_ping_3min", 100.0)),
                float(r.get("cpu_load", 0.5)),
                float(r.get("stake_amount", 0.01)),
                float(r.get("past_lag_count", 0.0)),
            ])
            y.append(float(r.get("choke_next_block", 0.0)))
    if not X:
        return None, None
    X = torch.tensor(X, dtype=torch.float32)
    y = torch.tensor(y, dtype=torch.float32).unsqueeze(1)
    return X, y

def train_once(state=STATE_OUT, data=DATA_PATH):
    X, y = load_dataset(data)
    if X is None:
        print("No data at", data); return
    if len(X) < 200:
        print("Not enough samples to train:", len(X)); return
    model = TinyMLP()
    if os.path.exists(state):
        model.load_state_dict(torch.load(state, map_location="cpu"))
    model.train()
    opt = optim.Adam(model.parameters(), lr=LR)
    bce = nn.BCELoss()
    for _ in range(EPOCHS):
        idx = torch.randperm(len(X))
        for i in range(0, len(X), BATCH):
            j = idx[i:i+BATCH]
            loss = bce(model(X[j]), y[j])
            opt.zero_grad(); loss.backward(); opt.step()
    torch.save(model.state_dict(), state)
    export_onnx(state, ONNX_OUT)
    print(f"Updated {state} and {ONNX_OUT}")

if __name__ == "__main__":
    train_once()