# Solana Vanity Wallet Generator

A multi-threaded Rust-based tool to generate Solana wallet keypairs that match a specific prefix and/or postfix. This tool is designed for users who want to create vanity wallets for Solana while maximizing generation speed using all available CPU threads.

## Features

- **Custom Prefix/Postfix**: Define a prefix or postfix (or both) for the public keys.
- **Case Sensitivity**: Choose between case-sensitive or case-insensitive matching.
- **Multi-threaded Performance**: Utilize all available CPU threads for faster key generation.
- **Progress Tracking**: View real-time progress, including attempts, speed, and elapsed time.
- **Key Storage**: Save all generated keys in a file (keys.txt) for later use.

## Requirements

- Rust (tested with the latest stable version)
- A computer with multiple CPU cores (recommended for optimal performance)

## Installation

1. Clone this repository:<br>
   `git clone https://github.com/therealaleph/solana-vanity-wallet-generator.git` <br>
   `cd solana-vanity-wallet-generator`
2. Install dependencies:<br>
  `cargo build --release`

3. Run:<br>
  `cargo run --release`
