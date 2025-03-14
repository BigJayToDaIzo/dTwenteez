# dTwenteez

[![Rust Version](https://img.shields.io/badge/Rust-1.85+-blue.svg)](https://www.rust-lang.org)

A dicing tool written in Rust.

---

## Roadmap
 - See [CHANGELOG.md](CHANGELOG.md)

### MVP 0.0.1

   - [x] Interactively ask user for roll input
        - ex: "d20" "6d6+1 a" "2d10-1 d"
   - [x] accept cli args
   - [x] Give user interactive quit option
   - [ ] Give user interactive help option
   - [x] Handle input errors gracefully
   - [x] Display roll total, and individual rolls in brackets
   - [x] 5)85+% code coverage in test framework
### MVP 0.0.2

   - [ ] logging to text file
### MVP 0.0.3

   - [ ] real logging / database system 
   - [ ] public api for client apps
   - [ ] a client app/UI
### Task Checklist (moves to changelog when complete)

   - [x] Task 1: Lay scaffold for testing
   - [x] Task 1.5: Implement README.md with roadmap
   - [x] Task 2: Display greeting
   - [x] Task 3: Capture user input
   - [x] Task 4: display results of roll
   - [x] Task 5: Ask for next roll/quit
---

## Features

- Command line arguments for a quick single roll.
- Interactive mode to save typing while gaming.
- Coming Soon: roll logging and beautiful UI.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Usage

1. **Clone the Template**:
   ```bash
   git clone https://github.com/BigJayToDaIzo/dTwenteez.git dTwenteez
   cd dtwenteez
2. **Initialize a New Repository**:
   ```bash
   rm -rf .git
   git init
3. **Update Project Metadata**:
   1. Update the Cargo.toml file with your project's name, version, and other details.
   2. Update the README.md file to reflect your project's description and usage.

4. Build and Run the Project:
   ```bash
   cargo build
   target/debug/dTwenteez -h
