// integration tests for chain and dolev-strong.

use blockchain_consensus::{dolev_strong_protocol, Block, Blockchain};
use std::collections::HashSet;

#[test]
fn nakamoto_genesis_and_append() {
    let mut chain = Blockchain::new();
    let g = chain.genesis("genesis");
    assert_eq!(g.height, 0);
    assert_eq!(chain.length(), 1);
    assert_eq!(chain.get_log(), vec!["genesis"]);

    let b1 = Block::new(1, g.block_hash(), "tx1", 0);
    assert!(chain.add_block(b1));
    assert_eq!(chain.length(), 2);
    assert_eq!(chain.get_log(), vec!["genesis", "tx1"]);
}

#[test]
fn nakamoto_longest_chain_reorg() {
    let mut chain = Blockchain::new();
    let g = chain.genesis("genesis");
    let tip = g.block_hash();

    let b1 = Block::new(1, tip.clone(), "A", 0);
    let b2 = Block::new(2, b1.block_hash(), "B", 0);
    chain.add_block(b1);
    chain.add_block(b2);
    assert_eq!(chain.length(), 3);

    let alt1 = Block::new(1, tip, "C", 0);
    chain.add_block(alt1);
    chain.reorg_to_longest();
    assert_eq!(chain.length(), 3);
    assert_eq!(chain.get_log(), vec!["genesis", "A", "B"]);
}

#[test]
fn dolev_strong_all_honest_sender_one() {
    let outputs = dolev_strong_protocol(4, 1, "1", &HashSet::new());
    for (_id, out) in &outputs {
        assert_eq!(out, "1");
    }
}

#[test]
fn dolev_strong_all_honest_sender_zero() {
    let outputs = dolev_strong_protocol(4, 1, "0", &HashSet::new());
    for (_id, out) in &outputs {
        assert_eq!(out, "0");
    }
}

#[test]
fn dolev_strong_consistency_n5_f2() {
    let outputs = dolev_strong_protocol(5, 2, "1", &HashSet::new());
    let vals: Vec<&String> = outputs.values().collect();
    assert!(!vals.is_empty());
    let first = vals[0];
    for v in &vals[1..] {
        assert_eq!(*v, first);
    }
}
