# Rust Interpreter

A simple interpreter built in Rust. This project is a learning exercise to explore the concepts of building a programming language, including tokenizing, parsing, and evaluation.

## Features

*   **Variables:** Declare variables using the `remember` keyword.
    ```
    remember x = 10;
    ```
*   **Data Types:**
    *   Integers (`i64`)
    *   Floats (`f64`)
    *   Booleans (`true`, `false`)
    *   Strings (`String`)
    *   Null
    *   Lists
*   **Arithmetic Operations:**
    *   Addition (`+`)
    *   Subtraction (`-`)
    *   Multiplication (`*`)
    *   Division (`/`)
    *   Unary plus and minus
*   **Comparisons:**
    *   Equality (`==`)
    *   Greater than (`>`)
    *   Less than (`<`)
*   **Control Flow:**
    *   `if/else` statements
    *    `while` loop
    *    `for` loops
*   **Functions:**
    *   User-defined functions
*   **Scoping:**
    *   Block-level scope using curly braces `{}`.
*   **Output:**
    *   Print expressions to the console with `yell()`.
    ```
    yell(5 + 5);
    ```

## Getting Started

### Prerequisites

*   Rust and Cargo installed on your system.

### Running

1.  Clone the repository:
    ```sh
    git clone <repository-url>
    ```
2.  Navigate to the project directory:
    ```sh
    cd rust-interpreter
    ```
3.  Run the interpreter with a given input:
    ```sh
    cargo run -- "remember x = 10; yell(x * 2);"
    ```

## Roadmap

This interpreter is still under development. Here are some features planned for the future:

*   **Control Flow:**
    *    `break` and `continue` statements
*   **Functions:**
    *   Closures
*   **Standard Library:**
    *   Read in from the command line

## Usage of AI
This project has been purely hand-written and no AI was used to generate any of the code.
It's a learning project to teach myself about interpreters/compilers so any AI usage for code
generation would be counterproductive.