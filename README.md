<!-- # Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) -->

# bitcoin-gui-rs

## Overview

`bitcoin-gui-rs` is a Proof of Concept (POC) for a standalone GUI application that interacts with Bitcoin Core using a Rust-based GUI framework (Tauri). Bitcoin Core's multiprocess feature separates its components into different executables (`bitcoin-node`, `bitcoin-wallet`, `bitcoin-gui`), and each of them can be run as different processes, with the `bitcoin-node` connected to the bitcoin network and the `bitcoin-wallet` and `bitcoin-gui` connected to the `bitcoin-node` through a socket pair, using an IPC for communication between them everything through a capnp proton, but in this POC, the GUI is treated as an independent binary (`bitcoin-node`) that communicates with Bitcoin Core via a Rust IPC interface.

You can check more about Bitcoin Multiprocess from this [document](https://github.com/ryanofsky/bitcoin/blob/pr/ipc/doc/design/multiprocess.md) by ryanofsky. This work was heavily inspired by His work.

## Features
- Standalone GUI built with Rust (Tauri).
- Communicates with Bitcoin Core through an IPC interface.
- Supports interactions with the Bitcoin Core Regtest network.
- Provides basic chain functionalities.

## Prerequisites

Ensure you have the following dependencies installed:

- **System Dependencies (Linux, macOS, Windows)** - [System Dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies)
- **Rust** – [Rust](https://v2.tauri.app/start/prerequisites/#rust)
- **Bitcoin Core Build (with multiprocess enabled)** – [blocktalk](https://github.com/pseudoramdom/blocktalk?tab=readme-ov-file#setup-guide) or [bitcoin multiprocess setup](https://github.com/ryanofsky/bitcoin/blob/pr/ipc/doc/multiprocess.md).

## Setting up the Development Environment

### 1. Start Bitcoin Core in Regtest Mode

Regtest (Regression Test Mode) allows local Bitcoin connect which is great for quickly testing out the bitcoin blockchain without connecting to the mainnet nor testnet.

1. Create a data directory
```sh
mkdir -p datadir
```

2. Start the Bitcoin node in regtest
```sh
./src/bitcoin-node \
-regtest \
-datadir=$PWD/datadir \
-server=0 \
-port=19444 \
-connect=127.0.0.1:18444 \
-ipcbind=unix \
-debug=ipc
```

## Usage

### Run the Tauri Application

1. Clone this repository and navigate to the project directory:
```sh
# clone the repository
git clone https://github.com/GideonBature/bitcoin-gui-rs.git

# move into the project directory
cd bitcoin-gui-rs/bitcoin-gui
```

2. Install dependencies:
```sh
npm install  # or yarn install
```
3.  Go to [lib.rs](https://github.com/GideonBature/bitcoin-gui-rs/blob/main/bitcoin-gui/src-tauri/src/lib.rs) on line 85, add the path to the socket file created by running bitcoin-node in regtest with multiprocess.

4. Run the application:
```sh
npm run tauri dev
```
- The bitcoin-gui will open with a dashboard as shown below:
![Bitcoin GUI Dashboard](./gui-images/Rust%20GUI%20Dashboard.png)
For now only the some of the Blockchain functions have been implemented, others will still be added.

- To check for some Blockchain Information, simply click on `Blockchain Information` button and you will be met with the `Bitcoin Information Dashboard` as shown in the image below:
![Blockchain Information Dashboard](./gui-images/Blockchain%20Information%20Dashboard.png)

## Note
This project is in no way production ready, with some functions both from the bitcoin-node and bitcoin-wallet that are yet to be implemented. For now it is still in active development.

## Acknowledgements

I found this [bitcoin-ipc tool](https://github.com/bitcoin-dev-tools/bitcoin-ipc) useful for generating a rust IPC interface for bitcoin core via rust-capnproto. Also, was inspired by pseudorandom's work on [blocktalk](https://github.com/pseudoramdom/blocktalk/tree/main).

## Contributing
Feel free to open issues and pull requests to improve the project.
