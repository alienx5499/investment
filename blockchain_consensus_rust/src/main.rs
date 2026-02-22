// cli: chain growth or dolev demo, output JSON.

use blockchain_consensus::{dolev_strong_protocol, Block, Blockchain};
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "dolev" {
        run_dolev_strong_demo();
    } else {
        run_chain_growth_demo();
    }
}

fn run_chain_growth_demo() {
    let mut chain = Blockchain::new();
    chain.genesis("genesis");

    let mut blocks_per_round: Vec<Vec<Block>> = Vec::new();
    let mut prev_hash = chain.chain().last().unwrap().block_hash();
    for round in 1..=10 {
        let mut blocks = Vec::new();
        let b = Block::new(round, prev_hash.clone(), format!("tx round {}", round), 0);
        prev_hash = b.block_hash();
        blocks.push(b);
        blocks_per_round.push(blocks);
    }

    let lengths = chain.chain_growth_lengths_over_rounds(&blocks_per_round);
    let log = chain.get_log();

    let out = serde_json::json!({
        "chain_lengths_per_round": lengths,
        "final_log": log,
        "final_length": chain.length(),
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

fn run_dolev_strong_demo() {
    let n = 4u32;
    let f = 1u32;
    let corrupt: HashSet<u32> = HashSet::new();
    let outputs = dolev_strong_protocol(n, f, "1", &corrupt);
    let out = serde_json::json!({
        "n": n,
        "f": f,
        "sender_input": "1",
        "honest_outputs": outputs,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
