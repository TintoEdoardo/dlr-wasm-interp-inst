# Instantiating a WebAssembly interpreter

This repository illustrates how to create a (Rust) library instantiating an interpreter for WebAssembly bytecode. This guide uses the interpreter in [DLR-FT/wasm-interpreter](https://github.com/DLR-FT/wasm-interpreter). 

The compilation toolchain targets the Digilent Cora Z7 (dual-core) board which hosts a Zynq7000 chip. 
A separate guide showing how to use the library from within a Ravenscar-compliant Ada application, running on the selected board (*without any operating system*). 

Before building the library, you need the first version of the Wasm interpreter. 
Hence, remember to run the following command from the git submodule folder: 
```
git checkout dd31ded1d03b56b6c8a04be9f7fa08508f9798a0
```
The command pins the repository to the commit corresponding to release v0.1.0.

---

