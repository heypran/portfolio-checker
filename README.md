# Portfolio Checker

A simple Rust command-line tool to fetch token balances for EVM addresses using the Alchemy API.

## Features
- Fetches all token balances for given addresses across multiple networks (Ethereum, Optimism, Base).
- Outputs balances in a clean table format.
- Supports one address per line in input file.

## Prerequisites
- Rust (install from [rustup.rs](https://rustup.rs/))
- Alchemy API key (get from [Alchemy](https://www.alchemy.com/))

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd portfolio-checker
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage
```bash
cargo run <file> <your-api-key>
```

### Arguments
- `<file>`: Path to file containing EVM addresses (one per line).
- `<key>`: Your Alchemy API key.

### Example
1. Create a file `addresses.txt` with addresses:
   ```
   0x1
   0x2
   0xAnotherAddressHere
   ```

2. Run the tool:
   ```bash
   cargo run addresses.txt sdf23123
   ```

3. Output: A table of token balances.

## License
MIT
