# Examples of how to use patisson-bybit-sdk

This directory contains a number of examples showcasing various capabilities of the `patisson-bybit-sdk` crate.

## Example list

`instruments-info`, `kline`, `recent-trading-history`, `ticker`, `server-time`, `stream-public`, `stream-all-liquidation`

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
