src/
├── lexer/
│   ├── mod.rs       # Exposes the main Lexer API and token definitions
│   ├── tokens.rs    # Holds the Token enum and Logos macro attributes
│   └── source.rs    # Handles source stream positions and error management


2. How to Decide What to Add Next
When expanding your programming language, you should add tokens category-by-category as you build your parser and language specifications:

Control Flow: if, else, for, match, break, continue

Data Types & Structs: struct, enum, true, false, string literals ("...")

Operators: - (Minus), * (Multiply), / (Divide), != (Not equal), >= / <=

3. How to Add New Syntax (Step-by-Step)
Whenever you want to introduce a new keyword or symbol into Aetrox, you only need to modify src/lexer/tokens.rs.

Example A: Adding an if keyword
Add the attribute and the enum variant inside Token:

Rust
#[token("if")]
If,
Example B: Adding String Literals (Text values like "Hello")
Add a regular expression variant that matches anything inside double quotes:

Rust
#[regex(r#""([^"\\]|\\.)*""#, |lex| {
    let slice = lex.slice();
    // Strip the surrounding quotation marks
    slice[1..slice.len() - 1].to_string()
})]
StringLiteral(String),