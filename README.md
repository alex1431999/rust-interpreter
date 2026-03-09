# Rust Interpreter

A simple interpreter built in Rust. This project is a learning exercise to explore the concepts of building a programming language, including tokenizing, parsing, and evaluation.

## Features

*   **Variables:** Declare variables using the `remember` keyword.
    ```
    remember x = 10;
    ```
*   **Data Types:**
    *   Integers (`i64`)
    *   Booleans (`true`, `false`)
    *   Strings (`String`)
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

*   **More Data Types:**
    *   Floats
    *   Arrays/Lists
*   **Control Flow:**
    *   `while` and `for` loops
    *   `break` and `continue` statements
*   **Functions:**
    *   User-defined functions
    *   Closures
*   **Standard Library:**
    *   A small set of built-in functions.
*   **Improved Error Handling:**
    *   More descriptive error messages with line and column numbers.
*   **File Execution:**
    *   Ability to execute scripts from files.
