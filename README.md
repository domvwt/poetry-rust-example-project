# Adders

A simple Python package demonstrating how to build and package a Rust extension using Poetry.

## Project Structure

```text
.
├── Makefile             # Makefile for convenience
├── poetry.lock          # Lock file for Poetry dependencies
├── pyproject.toml       # Poetry configuration file
├── python
│   ├── src
│   │   └── adders
│   │       ├── addition.py  # Python wrapper for Rust functions
│   │       └── __init__.py  # Package initializer
│   └── tests
│       ├── __init__.py
│       └── test_addition.py # Test file for the addition module
├── README.md            # This README file
└── rust
    ├── Cargo.lock       # Cargo lock file for Rust dependencies
    ├── Cargo.toml       # Cargo configuration file
    ├── pyproject.toml   # maturin configuration file
    └── src
        └── lib.rs       # Rust source file containing the extension implementation
```

## Getting Started

### Prerequisites

- Python 3.8.1 or higher
- Poetry
- Rust
- maturin

### Building the Project

1. Clone the repository:

```bash
git clone https://github.com/domvwt/poetry-rust-example-project.git
cd poetry-rust-example-project
```

2. Install the project:

```bash
make install
```

3. Activate the virtual environment:

```bash
poetry shell
```

4. Run the tests:

```bash
make test
```

5. Build the project:

```bash
make dist
```

### Using the Package

You can use the `adders` package in your Python projects like this:

```python
from adders import add

result = add(2, 3)
print(result)  # Output: 5
```
