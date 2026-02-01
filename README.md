<div align="center">
  <img alt="The Felys Programming Language" src="https://raw.githubusercontent.com/felys-lang/.github/main/felys.png" width="50%">
</div>

<div align="center">
  <a href="https://felys.dev/quickstart">Quickstart</a> |
  <a href="https://felys.dev/">Documentation</a> |
  <a href="https://exec.felys.dev/">Playground</a>
</div>

## What is the Felys Programming Language?

Felys is an interpreted programming language written in Rust that comes with a compiler and a runtime. Feel free to try it using the online [playground](https://exec.felys.dev/). Please note, however, that it is based on a legacy version of Felys. Once the ongoing reconstruction for the compilation framework is done, the websites will be updated.

## Components

- [PhiLia093](felys/src/philia093): Parser and the [generator](philia093) that bootstraps itself
- [Cyrene](felys/src/cyrene): Control flow graph builder and transformer to IR
- [Demiurge](felys/src/demiurge): Dead code elimination, register allocation, and codegen
- [Elysia](felys/src/elysia): Execution runtime and bytecode loader/dumper

## License

Distributed under the terms of the [LICENSE](LICENSE).

## Copyright

© All rights reserved by miHoYo

## Legal Statement

Other properties and any right, title, and interest thereof and therein (intellectual property rights included) not derived from Honkai Impact 3rd and Honkai: Star Rail belong to their respective owners.
