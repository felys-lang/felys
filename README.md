<div align="center">
  <img alt="The Felys Programming Language" src="https://raw.githubusercontent.com/felys-lang/.github/main/felys.png" width="50%">
</div>

<div align="center">
  <a href="https://felys.dev/quickstart">Quickstart</a> |
  <a href="https://felys.dev/">Documentation</a> |
  <a href="https://exec.felys.dev/">Playground</a>
</div>

## What is the Felys Programming Language?

Felys is a dependency-free interpreted programming language written in Rust, featuring its own compiler and runtime. Feel free to try it out in the online [playground](https://exec.felys.dev/). Please note, however, that the language is currently in a fragile state following a major reconstruction and requires project-level refactoring to improve code quality.

## Components

I understand that the naming and code organization do not make sense, but I like it.

- [PhiLia093](felys/src/philia093): Parser and a general-purpose [generator](philia093) with self-bootstrapping capabilities
- [Cyrene](felys/src/cyrene): Control-flow graph builder and transformer for intermediate representation
- [Demiurge](felys/src/demiurge): Dead code elimination, register allocation, and code generation
- [Elysia](felys/src/elysia): Execution runtime, featuring a neural network library and bytecode loader/dumper

The design is simple, but still, here's the high-level pipeline:

```mermaid
flowchart TD
    subgraph FrontEnd [Front-end]
        direction LR
        FA{{Source}} --> FB(Syntactical Analysis)
        FB --> FC{{AST}}
        FC --> FD(Semantical Analysis)
        FD --> FE[SSA IR Construction]
        FE --> FF{{CFG}}
    end

    FrontEnd --> MidEnd

    subgraph MidEnd [Mid-end]
        direction LR
        MA{{CFG}} --> MB(SCCP Analysis)
        MB --> MC(Constant Folding)
        MC --> MD(Trivial Phi Removal)
        MD --> ME(Dead Code Elimination)
        ME --> MF(Jump Threading)
        MF -.-> MB
        MA -.-> MG
        MF --> MG{{CFG}}
    end

    MidEnd --> BackEnd

    subgraph BackEnd [Back-end]
        direction LR
        BA{{CFG}} --> BB(Phi to Copy)
        BB --> BC(Register Allocation)
        BC --> BD(Code Generation)
        BD --> BE{{Bytecode}}
    end

    BackEnd --> Runtime

    subgraph Runtime [Runtime]
        direction LR
        RA{{Binary}} --> RB(Loader)
        RB --> RC{{Bytecode}}
        RC --> VM(Virtual Machine)
    end
```

Hexagons represent the state of the program at a specific stage. Dotted lines means an optional path, but not configured in the [playground](https://exec.felys.dev/). Specifically, repeating optimization passes enables deeper optimization, though skipping them is also valid. However, a single pass is the most optimal configuration for most tasks. And yes, lexical analysis does not exist.

## Don't know what I'm talking about?

The following papers, blogs, and books helped me a lot. Also, ask LLMs.

- [Packrat Parsing: Simple, Powerful, Lazy, Linear Time](https://arxiv.org/abs/cs/0603077)
- [PEG Parsing Series Overview](https://medium.com/@gvanrossum_83706/peg-parsing-series-de5d41b2ed60)
- [Simple and Eﬃcient Construction of Static Single
  Assignment Form](https://c9x.me/compile/bib/braun13cc.pdf)
- [Compilers: Principles, Techniques, and Tools](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)

## License

Distributed under the terms of the [LICENSE](LICENSE).

## Copyright

© All rights reserved by miHoYo

## Legal Statement

Other properties and any right, title, and interest thereof and therein (intellectual property rights included) not derived from Honkai Impact 3rd and Honkai: Star Rail belong to their respective owners.
