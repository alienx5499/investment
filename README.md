# Investment

Code related to the investment section of the website.

See [market_sim](market_sim/README.md) for more details on the market simulation framework

### Blockchain consensus (Rust)

Concepts from *Foundations of Distributed Consensus and Blockchains* (Elaine Shi) are implemented in **Rust** under [`blockchain_consensus_rust/`](blockchain_consensus_rust/):

- **Nakamoto-style blockchain** (Ch 6, 14): hash-linked blocks, longest-chain rule.
- **Dolev-Strong Byzantine Broadcast** (Ch 3): f+1 rounds, consistency and validity.

```bash
cd blockchain_consensus_rust && cargo build && cargo test
cargo run          # chain growth demo (JSON)
cargo run -- dolev # Dolev-Strong demo
```

**Use consensus in a scenario:** pass `consensus='rust'` to `create_market_making_scenario(...)`; results will include `results['consensus']` with chain data. Default is no consensus.

See [blockchain_consensus_rust/README.md](blockchain_consensus_rust/README.md) for details.

## Instructions (test)

1. Read this readme and all the docs.
2. Create a fork of this repo with your contribution.
3. Check other branches (blockchain_integration_test_v0.1).
4. Contact.

## Contact

juan.diez@torbellino.tech

## Further docs

test: https://drive.google.com/file/d/19vKu5HmRJWuzrDGBsTaEiHyJlznYUZKO/view?usp=drive_link
book: https://drive.google.com/file/d/1l9_uCBWikmX-XX5E15n3T65Nkh_cNnEW/view?usp=drive_link

## Usage

```
python3 test_db_operations.py
```

## Market Dynamics and Trading Simulation

Implements a framework for simulating, analyzing, and learning about financial markets, trading strategies, and blockchain integration.

Currently v0.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The MIT License was chosen to:
- Encourage wide adoption and collaboration
- Allow commercial and academic use
- Keep compliance simple
- Protect contributors from liability
- Maintain compatibility with most open-source projects

## Notes

Project log reinitialized on 2025-01-19.
