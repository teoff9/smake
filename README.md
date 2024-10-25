# SMAKE

**SMAKE** is a simple yet powerful Makefile generator written in Rust. Designed for C and C++ projects, SMAKE scans your source files for dependencies and automatically generates a Makefile. You can specify additional arguments, choose between `gcc` and `g++` compilers, and customize your build setup.

### Features

- **Automated Makefile Generation**: Quickly generate a Makefile for `.c` and `.cpp` files.
- **Compiler Options**: Supports both `gcc` and `g++`.
- **Extra Arguments**: Add any extra flags or arguments to customize your build process.
- **Limitations**: Currently, SMAKE’s dependency resolution is non-recursive, analyzing only direct dependencies. Future versions may include recursive dependency analysis.

---

## Installation

To install **SMAKE**, follow these steps:

### Prerequisites
Ensure that [Rust](https://www.rust-lang.org/tools/install) is installed. You’ll need Cargo, Rust’s package manager, which is included with the Rust installation.

### Steps to Install

1. **Clone the Repository**  
   Open a terminal and clone the repository:
   ```bash
   git clone https://github.com/your-username/smake.git
2. **Cd into smake**
   Write 
   ```bash 
   cd smake
3. **Install it**
    Write in the terminal 
    ```bash 
    cargo install

Check that the cargo bin folder is added to your $PATH !! and enjoy



