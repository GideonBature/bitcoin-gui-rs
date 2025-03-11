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

1. **System Dependencies (Linux, macOS, Windows)** - [System Dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies)
2. **Rust** – [Rust](https://v2.tauri.app/start/prerequisites/#rust)
3. **Compile Bitcoin Core Build (with multiprocess enabled - PR #29409)**

    ```sh
    # Clone Bitcoin Core and checkout the PR
    git clone https://github.com/bitcoin/bitcoin.git
    cd bitcoin
    git fetch origin pull/29409/head:pr29409
    git checkout pr29409

    # Build dependencies with multiprocess support
    make -C depends HOST=aarch64-apple-darwin MULTIPROCESS=1 NO_QT=1

    # Configure and build Bitcoin Core
    export HOST_PLATFORM="aarch64-apple-darwin"

    # Build (works for macOS)
    cmake -B multiprocbuild/ \
        --toolchain=depends/$HOST_PLATFORM/toolchain.cmake \
        -DOPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3 \
        -DOPENSSL_CRYPTO_LIBRARY=/opt/homebrew/opt/openssl@3/lib/libcrypto.dylib \
        -DOPENSSL_SSL_LIBRARY=/opt/homebrew/opt/openssl@3/lib/libssl.dylib \
        -DOPENSSL_INCLUDE_DIR=/opt/homebrew/opt/openssl@3/include \
        -DZLIB_ROOT=/opt/homebrew/opt/zlib \
        -DZLIB_LIBRARY=/opt/homebrew/opt/zlib/lib/libz.dylib \
        -DZLIB_INCLUDE_DIR=/opt/homebrew/opt/zlib/include

    # Final build process
    cmake --build multiprocbuild/ --parallel $(sysctl -n hw.logicalcpu)
    ```

## Setting up the Development Environment

### 1. Start Bitcoin Core in Regtest Mode

Regtest (Regression Test Mode) allows local Bitcoin connect which is great for quickly testing out the bitcoin blockchain without connecting to the mainnet nor testnet.

- **Run two regtest nodes, one `bitcoind` and one `bitcoin-node`**

    Inorder to have a readily available funded wallet, we'll use a `bitcoind` with a regular Bitcoin Core wallet on it, connected to the `bitcoin-node` node.

    Start the `bitcoind` node, create a wallet on it (I'll call it bene), and fund the wallet by mining some blocks:

    ```sh
    mkdir regular_bitcoind_wallet

    ./multiprocbuild/src/bitcoind -regtest -datadir=$PWD/regular_bitcoind_wallet -daemon

    ./multiprocbuild/src/bitcoin-cli -regtest -datadir=$PWD/regular_bitcoind_wallet createwallet bene

    ./multiprocbuild/src/bitcoin-cli -regtest -datadir=$PWD/regular_bitcoind_wallet -rpcwallet=bene generatetoaddress 110 $(./multiprocbuild/src/bitcoin-cli -regtest -datadir=$PWD/regular_bitcoind_wallet -rpcwallet=bene getnewaddress)
    ```

    Now start the `bitcoin-node` node in a different datadir, connected to the first node, with no JSONRPC server:
    ```sh
    mkdir datadir_bitcoin-node

    ./multiprocbuild/src/bitcoin-node -regtest -datadir=$PWD/datadir_bdk_wallet -server=0 -port=19444 -connect=127.0.0.1:18444 -ipcbind=unix -debug=ipc
    ```
    Take note of the *`-ipcbind=unix`* to create the interface and optional *`-debug=ipc`* to observe IPC messages.

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
```rust
let socket_path = "/path/to/bitcoin-core/socket";
```

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
