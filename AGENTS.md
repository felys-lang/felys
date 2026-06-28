# Workspace structure
- `felys/` - Main interpreter library with 4 compiler stages (cyrene→demiurge→elysia runtime) and utils
- `philia093/` - PEG parser generator (CLI binary, not published)
- `grammar/` - `.peg` grammar files for the language (felys.peg) and meta-grammar (meta.peg)

# Code generation
Grammar files must be regenerated when modified:
```bash
cargo run --bin philia093 grammar/meta.peg philia093/src/philia093/
cargo run --bin philia093 grammar/felys.peg felys/src/philia093/
```
This generates Rust parser code into the target directories.

# Testing
Integration tests in `felys/tests/` use a helper (`exec()`) that compiles Felys code, executes it, and validates output.
Run specific test suites: `cargo test --test <demo|general|stdlib>`

# Standard commands
```bash
cargo build --workspace    # Build both packages
cargo test                  # Run all tests (unit + integration)
cargo clippy --workspace    # Lint all packages
cargo fmt                   # Format all code
cargo check --workspace     # Quick typecheck without building
```

# Architecture
- Compiler pipeline: `PhiLia093::parse()` → `desugar()` → `codegen(opt_level)` → `dump()` → `III::load()` → `exec()`
- Tests verify compiler output at multiple optimization levels (0, 1, 2, usize::MAX)
- Bytecode is serialized/deserialized via `dump()` / `III::load()` for persistence