# Blockchain Consensus (Rust)

Implementation of concepts from **Foundations of Distributed Consensus and Blockchains** (Elaine Shi, distributedconsensus.net) for the Torbellino Tech investment/market_sim project.

## Concepts Implemented

- **Nakamoto-style blockchain** (Ch 6, Ch 14): collision-resistant hash-linked blocks (Ch 2), longest-chain rule, ever-growing log (state machine replication).
- **Dolev-Strong Byzantine Broadcast** (Ch 3): synchronous rounds, designated sender, extracted set, f+1 rounds for consistency and validity under up to f corrupt nodes.

## Why Rust

Rust is used here for implementation clarity and familiarity; the assignment allows languages other than Python when there is a reason. A Rust implementation also aligns with performance-sensitive and safety-critical consensus code in production (e.g. many blockchain nodes use Rust or C++).

## Build and Test

```bash
cd blockchain_consensus_rust
cargo build
cargo test
```

## Run (CLI)

**Chain growth demo** (outputs JSON for visualization):

```bash
cargo run
```

**Dolev-Strong demo** (n=4, f=1, sender input "1"):

```bash
cargo run -- dolev
```

## Layout

- `src/lib.rs` — re-exports.
- `src/nakamoto.rs` — `Block`, `Blockchain` (genesis, add_block, reorg_to_longest, get_log, chain_growth_lengths_over_rounds).
- `src/dolev_strong.rs` — `SignedMessage`, `DolevStrongNode`, `dolev_strong_protocol`.
- `src/main.rs` — CLI: chain growth and Dolev-Strong demos (JSON output).
- `tests/integration_tests.rs` — tests for chain and Dolev-Strong.

## Integration with market_sim

This crate lives under the investment repo as the **blockchain/consensus** implementation. The Python `market_sim` package can call the binary for demos/visualizations (see `market_sim/analysis/visualization/` or run this crate from the repo root).
