# dTwenteez

[![Rust Version](https://img.shields.io/badge/Rust-1.85+-blue.svg)](https://www.rust-lang.org)

A ready-to-use template for initializing Rust projects with a standard structure and common configurations. This template is designed to help you start your Rust projects quickly while adhering to best practices.

---

## Roadmap

### MVP in notebook
    - [x] 1)Ask user for #d20 to roll
    - [ ] 2)Give user quit option
    - [x] 3)Handle input errors gracefully
    - [x] 4)display roll total, and individual rolls in brackets
    - [x] 5)80% code coverage in test framework
### MVP 2.0
    // logging to text
    // accept cli args
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // env::args good enough?  Need args_os robustosity?
    // ex: d20 6d20 adv mod
    // manual cli at first
    // then on to crates.io to find something that already exists
    // MVP 3.0
    // real logging system
### Task Checklist (moves to changelog when complete)
    -[x] Task 1: Lay scaffold for testing
    -[ ] Task 1.5: Implement README.md with roadmap
    -[x]Task 2: Display greeting
    -[x] Task 3: Capture user input
    -[x] Task 4: display results of roll
    -[ ] Task 5: Ask for next roll/quit
---

## Features

- **Standard Directory Structure**: Includes `src`, `tests`, `examples`, and more.
- **Pre-configured `Cargo.toml`**: Common dependencies and configurations are set up.
- **Linting and Formatting**: Pre-configured with `clippy` and `rustfmt`.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Usage

1. **Clone the Template**:
   ```bash
   git clone https://github.com/BigJayToDaIzo/dtwenteez.git dtwenteez
   cd dtwenteez
2. **Initialize a New Repository**:
   ```bash
   rm -rf .git
   git init
3. **Update Project Metadata**:
   1. Update the Cargo.toml file with your project's name, version, and other details.
   2. Update the README.md file to reflect your project's description and usage.
