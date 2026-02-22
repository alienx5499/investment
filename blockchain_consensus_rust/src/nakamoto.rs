// nakamoto-style chain: hash-linked blocks, longest-chain rule.

use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt;

const GENESIS_PREV: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn hash_bytes(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

#[derive(Clone, Debug)]
pub struct Block {
    pub height: u64,
    pub prev_hash: String,
    pub data: String,
    pub nonce: u64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Block {
    pub fn new(height: u64, prev_hash: String, data: impl Into<String>, nonce: u64) -> Self {
        Self {
            height,
            prev_hash,
            data: data.into(),
            nonce,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let s = format!(
            "{{\"height\":{},\"prev_hash\":\"{}\",\"data\":\"{}\",\"nonce\":{},\"timestamp\":\"{}\"}}",
            self.height,
            self.prev_hash,
            self.data.replace('"', "\\\""),
            self.nonce,
            self.timestamp.to_rfc3339()
        );
        s.into_bytes()
    }

    pub fn block_hash(&self) -> String {
        hash_bytes(&self.to_bytes())
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.block_hash();
        write!(
            f,
            "Block(height={}, prev={}..., hash={}...)",
            self.height,
            &self.prev_hash[..8.min(self.prev_hash.len())],
            &h[..8.min(h.len())]
        )
    }
}

pub struct Blockchain {
    chain: Vec<Block>,
    blocks_by_hash: HashMap<String, Block>,
    pending: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            blocks_by_hash: HashMap::new(),
            pending: Vec::new(),
        }
    }

    pub fn genesis(&mut self, data: &str) -> Block {
        let g = Block::new(0, GENESIS_PREV.to_string(), data, 0);
        let hash = g.block_hash();
        self.chain.push(g.clone());
        self.blocks_by_hash.insert(hash, g.clone());
        g
    }

    fn validate_block(&self, block: &Block) -> bool {
        if block.height == 0 {
            return block.prev_hash == GENESIS_PREV;
        }
        let prev = match self.blocks_by_hash.get(&block.prev_hash) {
            Some(p) => p,
            None => return false,
        };
        prev.height == block.height - 1
    }

    // add block; returns true if on canonical chain.
    pub fn add_block(&mut self, block: Block) -> bool {
        let hash = block.block_hash();
        if self.blocks_by_hash.contains_key(&hash) {
            return block.height < self.chain.len() as u64
                && self
                    .chain
                    .get(block.height as usize)
                    .map(|b| b.block_hash())
                    == Some(hash);
        }
        if !self.validate_block(&block) {
            return false;
        }
        self.blocks_by_hash.insert(hash.clone(), block.clone());
        let last_hash = self.chain.last().map(|b| b.block_hash());
        if last_hash.as_deref() == Some(block.prev_hash.as_str()) {
            self.chain.push(block);
            return true;
        }
        self.pending.push(block);
        false
    }

    fn path_to_genesis(&self, block: &Block) -> Option<Vec<Block>> {
        let mut path = Vec::new();
        let mut cur: Option<&Block> = Some(block);
        while let Some(b) = cur {
            path.push(b.clone());
            if b.prev_hash == GENESIS_PREV {
                path.reverse();
                return Some(path);
            }
            cur = self.blocks_by_hash.get(&b.prev_hash);
        }
        None
    }

    // recompute canonical chain as longest chain.
    pub fn reorg_to_longest(&mut self) {
        if self.chain.is_empty() {
            return;
        }
        let mut best = self.chain.clone();
        for tip in &self.pending {
            if let Some(path) = self.path_to_genesis(tip) {
                if path.len() > best.len() {
                    best = path;
                }
            }
        }
        self.chain = best;
        self.pending
            .retain(|b| !self.chain.iter().any(|c| c.block_hash() == b.block_hash()));
    }

    pub fn chain(&self) -> &[Block] {
        &self.chain
    }

    pub fn length(&self) -> usize {
        self.chain.len()
    }

    // finalized log (ordered payloads).
    pub fn get_log(&self) -> Vec<String> {
        self.chain.iter().map(|b| b.data.clone()).collect()
    }

    // chain length after each round.
    pub fn chain_growth_lengths_over_rounds(
        &mut self,
        blocks_per_round: &[Vec<Block>],
    ) -> Vec<usize> {
        let mut lengths = Vec::with_capacity(blocks_per_round.len());
        for blocks in blocks_per_round {
            for b in blocks {
                self.add_block(b.clone());
            }
            self.reorg_to_longest();
            lengths.push(self.length());
        }
        lengths
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
