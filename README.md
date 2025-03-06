<!-- # Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) -->

# bitcoin-gui-rs

## Overview

`bitcoin-gui-rs` is a Proof of Concept (POC) for a standalone GUI application that interacts with Bitcoin Core using a Rust-based GUI framework (Tauri). Bitcoin Core's multiprocess feature separates its components into different executables (`bitcoin-node`, `bitcoin-wallet`, `bitcoin-gui`), and each of them can be run as different processes, with the `bitcoin-node` connected to the bitcoin network and the `bitcoin-wallet` and `bitcoin-gui` connected to the `bitcoin-node` through a socket pair, using an IPC for communication between them everything through a capnp proton, but in this POC, the GUI is treated as an independent binary (`bitcoin-node`) that communicates with Bitcoin Core via a Rust IPC interface.

## Features
- Standalone GUI built with Rust (Tauri).
- Communicates with Bitcoin Core through an IPC interface.
- Supports interactions with the Bitcoin Core Regtest network.
- Provides basic chain functionalities.

## Prerequisites

Ensure you have the following dependencies installed:

- **System Dependencies (Linux, macOS, Windows)** - [System Dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies)
- **Rust** – [Rust](https://v2.tauri.app/start/prerequisites/#rust)
- **Bitcoin Core Setup (multiprocess enabled)** – [blocktalk](https://github.com/pseudoramdom/blocktalk?tab=readme-ov-file#setup-guide) or [bitcoin multiprocess setup](https://github.com/ryanofsky/bitcoin/blob/pr/ipc/doc/multiprocess.md).

## Setting up the Development Environment

### 1. Start Bitcoin Core in Regtest Mode

Regtest (Regression Test Mode) allows local Bitcoin transactions without connecting to the main network.

```sh
bitcoind -regtest -daemon
```

To verify the node is running:
```sh
bitcoin-cli -regtest getblockchaininfo
```

### 2. Create a New Wallet
```sh
bitcoin-cli -regtest createwallet "test_wallet"
```

### 3. Generate Initial Blocks
Regtest does not have miners by default, so you need to mine some blocks:
```sh
bitcoin-cli -regtest generatetoaddress 101 $(bitcoin-cli -regtest getnewaddress)
```

### 4. Run the Tauri Application

Clone this repository and navigate to the project directory:
```sh
git clone https://github.com/yourusername/bitcoin-gui-rs.git
cd bitcoin-gui-rs/bitcoin-gui
```

Install dependencies:
```sh
npm install  # or yarn install
```

Run the application:
```sh
npm run tauri dev
```

## Contributing
Feel free to open issues and pull requests to improve the project.
