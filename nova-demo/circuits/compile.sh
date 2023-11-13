#!/bin/bash
# we compile to r1cs for nova and need wasm for witness generation
circom main.circom --r1cs --wasm
