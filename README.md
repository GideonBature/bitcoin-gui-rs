<!-- # Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) -->

# bitcoin-gui-rs

## Overview of the POC

The goal of this project is to create a standalone GUI using a Rust-based GUI Framework (Tauri) that interacts with Bitcoin Core running in regtest mode. Bitcoin Core's multiprocess feature splits its components into separate processes (bitcoin-node, bitcoin-wallet, bitcoin-gui), but for this POC, I am treating the GUI as an independent process communicating via JSON-RPC, and not the internal IPC mechanism (Cap’n Proto), as RPC is more stable and easier for external applications.

## Setting up Development Environment
### Regtest