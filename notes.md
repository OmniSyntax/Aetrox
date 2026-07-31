aetrox/
├── .github/
│   ├── workflows/
│   │   └── ci.yml             # Automated testing, clippy, and formatting check
│   └── assets/
│       └── logo.png           # Your cyan AX logo banner
├── docs/
│   └── syntax.md              # Language specification and documentation
├── examples/
│   └── hello.ax               # Sample Aetrox code files for testing
├── src/
│   ├── main.rs                # CLI entry point (handles arguments like 'aetrox run file.ax')
│   ├── lexer.rs               # Tokenizer (turns text into tokens using 'logos')
│   ├── parser.rs              # Parser (builds the Abstract Syntax Tree)
│   ├── ast.rs                 # AST node definitions
│   └── diagnostics.rs         # Error reporting engine using 'miette'
├── tests/
│   └── integration_tests.rs   # End-to-end compilation tests
├── Cargo.toml                 # Rust dependency manager & workspace config
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md                  # Your elite GitHub landing page