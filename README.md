# OpenQASM parser

A tool to parse OpenQASM2 files and generate Qiskit circuits from these.

## About This Project

The parser and generator are written in Rust.

This project is packaged using nix.

The current grammar is stored inside the `grammar.ebnf` file. The whole
OpenQASM2 grammar is stored inside the `openqasm.ebnf` file.

## Bibliography

[OpenQASM2](https://arxiv.org/pdf/1707.03429)

[Qiskit](https://www.ibm.com/quantum/qiskit)
