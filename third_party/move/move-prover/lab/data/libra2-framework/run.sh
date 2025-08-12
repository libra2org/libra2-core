#!/bin/bash

FRAMEWORK="../../../../../../libra2-move/framework/libra2-framework/sources"

# Benchmark per function (with `-f``). `-a` is for including the libra2-natives.
cargo run --release -p prover-lab -- bench -a -f -c prover.toml $FRAMEWORK/*.move $FRAMEWORK/configs/*.move $FRAMEWORK/aggregator/*.move

# Benchmark per module (without `-f`). `-a` is for including the libra2-natives.
cargo run --release -p prover-lab -- bench -a -c prover.toml $FRAMEWORK/*.move $FRAMEWORK/configs/*.move $FRAMEWORK/aggregator/*.move
