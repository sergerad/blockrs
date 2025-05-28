# Blockrs

Blockrs is a TUI for observing chain data.

![alt text](https://github.com/sergerad/blockrs/blob/main/image.png?raw=true)

## Installation

Use cargo to install the [crate](https://crates.io/crates/blockrs):

```
cargo install blockrs
blockrs -h
```

## Usage

Currently supports Ethereum and Miden RPC:

```
blockrs eth https://rpc.flashbots.net
blockrs miden https://testnet.miden.io
```

If you want to watch account balances, you need to provide a list via the config file.

For example, for Ethereum RPC:
```
cat <<EOF > /tmp/config.yaml
tick_rate: 5s
addresses:
  - 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
EOF
export BLOCKRS_CONFIG=/tmp/
blockrs eth https://rpc.flashbots.net
```

And, similarly, for Miden RPC:
```
blockrs miden https://rpc.testnet.miden.io
```

### User Input

The app will run in a mode which follows the HEAD of the chain by default.

If you hit `j/k/Up/Down` at any time, the app will stop following head and allow you to observe any previously-processed block.

To re-enter follow mode, hit `f/Space/Enter`.

## Roadmap

The following features are required for 1.0:

1. ~Interactive mode (stop tail, select block)~
2. ~Add implementations for other `ChainProvider` instances beyond Ethereum RPC~

## Acknowledgements

[Ratatui](https://ratatui.rs/) is a fantastic crate for creating TUI applications in Rust. This project is based on its template found [here](https://ratatui.rs/templates/).
