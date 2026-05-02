# Examples of how to use patisson-bybit-sdk

This directory contains a number of examples showcasing various capabilities of the `patisson-bybit-sdk` crate.

## Example list

`get-api-key-information`, `get-open-orders`, `get-position-info`, `get-transaction-log`, `instruments-info`, `kline`, `place-order`,`recent-trading-history`, `server-time`, `stream-all-liquidation`,`stream-private`,`stream-public`, `ticker`, `wallet`

All examples can be executed with:

```sh
cargo run --example $example_name
```

## Environment variables

Some examples that perform queries on private data expect these environment variables:

```sh
export API_KEY="xxxxxxxx"
export API_SECRET="xxxxxxxx"
```
