# RL
rust analog sudo for linux

An independent `sudo` alternative written in Rust.
Authenticates passwords via system PAM and executes commands as `root`.

## Installation

Run this single command in your terminal (from the project directory):

```bash
cargo build --release && sudo cp target/release/rl /usr/local/bin/rl && sudo chown root:root /usr/local/bin/rl && sudo chmod u+s /usr/local/bin/rl
