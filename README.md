<div align="center">
  <img alt="The Felys Programming Language" src="https://raw.githubusercontent.com/felys-lang/.github/main/felys.png" width="50%">
</div>

<div align="center">
  <a href="https://felys.dev/guide/quickstart.html">Quickstart</a> |
  <a href="https://felys.dev/">Documentation</a> |
  <a href="https://exec.felys.dev/">Playground</a>
</div>

## What is the Felys Programming Language?

Felys is a functional programming language written in Rust. The motivation is to create a language where Elysia exists, so regardless of how this project gets developed in the future, there must exist a built-in identifier called `__elysia__`. Although it is just a toy language, the implementation is quite elegant without ugly hacks. Feel free to try it on the online [playground](https://exec.felys.dev/). The ultimate goal is to build a virtual machine based Rust without borrow checker and lifetime checker.

## Highlights

There is a beautifully written packrat [parser](felys/parser) that supports left recursion, while maintaining an acceptable performance. It could be further optimized in memoization and memory management.

The [grammar](felys/ast) is similar to Rust, i.e. most traditionally defined statements are actually expressions in this language. As long as programmers understand the underlying principle, it is easy to write clean and readable code. This will also make the future semantic analyzer extremely powerful.

## License

Distributed under the terms of the [MIT License](LICENSE).

## Copyright

- © All rights reserved by FelysNeko
- Elysia's signet © All rights reserved by miHoYo

## Legal Statement

Other properties and any right, title, and interest thereof and therein (intellectual property rights included) not derived from Honkai Impact 3rd belong to their respective owners.
