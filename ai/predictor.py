# Copyright (c) 2025 Libra2 Contributors
# SPDX-License-Identifier: Apache-2.0
import torch
import torch.nn as nn

class TinyMLP(nn.Module):
    def __init__(self, in_dim=5, hidden=16):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(in_dim, hidden), nn.ReLU(),
            nn.Linear(hidden, 1), nn.Sigmoid()
        )
    def forward(self, x):
        return self.net(x)

def export_onnx(state_dict_path: str, onnx_out: str):
    m = TinyMLP()
    m.load_state_dict(torch.load(state_dict_path, map_location="cpu"))
    m.eval()
    dummy = torch.zeros(1, 5)
    torch.onnx.export(
        m, dummy, onnx_out,
        input_names=["feat"], output_names=["p_choke"],
        dynamic_axes={"feat": {0: "batch"}, "p_choke": {0: "batch"}},
        opset_version=17,
    )

if __name__ == "__main__":
    model = TinyMLP()
    torch.save(model.state_dict(), "ai/predictor_init.pt")
    export_onnx("ai/predictor_init.pt", "ai/predictor.onnx")
    print("Wrote ai/predictor_init.pt and ai/predictor.onnx")