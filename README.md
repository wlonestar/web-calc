# web-calc

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Overview

web-calc is a simple web-based calculator that supports both integer and floating-point numbers. It allows users to perform basic arithmetic operations such as addition, subtraction, multiplication, division, modulus, logical AND, logical OR, and parenthesis operations.

## Technology Stack

- WebAssembly: Leveraging the power of WebAssembly for efficient computation in the browser.
- Rust: Calculation logic is written in Rust, ensuring reliability and performance.
- NPM: The web interface is developed using NPM, providing a seamless user experience.
- lalrpop: Utilizing 'lalrpop' for lexer and parser, simplifying expression handling and improving efficiency.

## Usage

### Prerequisites

Before building it, make sure you hava setup the environment properly.

For detailed instructions, refer to the official documentation on: [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

### Build and Deployment

Follow these steps to build and deploy the web-calc project:

1. Clone the repository:

```bash
git clone https://github.com/wlonestar/web-calc.git
```

2. Build the project using the provided Makefile:

```bash
make
```

3. Deploy the project using npm

```bash
make deploy
```
