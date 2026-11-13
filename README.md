# Instantiating a WebAssembly interpreter

This repository illustrates how to create a (Rust) library instantiating an interpreter for WebAssembly bytecode. This guide uses the interpreter in [DLR-FT/wasm-interpreter](https://github.com/DLR-FT/wasm-interpreter). 

The compilation toolchain targets the Digilent Cora Z7 (dual-core) board which hosts a Zynq7000 chip. 
A separate guide showing how to use the library from within a Ravenscar-compliant Ada application, running on the selected board (*without any operating system*). 

We are using the Wasm interpreter version 0.1.0.
The submodule is already pinned to the commit associated with the release tag. 

---

