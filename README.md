### Devconnect folding workshop

This is a demo repo for the devconnect nova/folding workshop. We will go through a simple example, namely explaining the `toy` example (fibonacci circuit) that you can find in the [`nova-scotia`](https://github.com/nalinbhardwaj/Nova-Scotia/tree/main) repo.

Program:

1. A quick applied review of how Nova works [here](https://hackmd.io/@PierreDM/BkBXmhk4T).
2. Going over the circuit that we will fold
3. Implementing folding using `nova-scotia`

### Usage

Two main useful commands:

- Compiling our main circuit: `circom --r1cs --wasm main.circom`
- Running nova with `novas-scotia`: `cargo run -r`