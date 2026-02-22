// dolev-strong Byzantine broadcast: f+1 rounds, extracted set.

use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

fn sign(node_id: u32, message: &str) -> String {
    let mut h = Sha256::new();
    h.update(format!("{}:{}", node_id, message));
    hex::encode(h.finalize())[..16].to_string()
}

fn verify(node_id: u32, message: &str, sig: &str) -> bool {
    sign(node_id, message) == sig
}

fn message_after_signatures(value: &str, signers: &[u32], signatures: &[String]) -> String {
    let mut msg = value.to_string();
    for i in 0..signers.len() {
        msg.push_str(&format!(":{}:{}", signers[i], signatures[i]));
    }
    msg
}

#[derive(Clone, Debug)]
pub struct SignedMessage {
    pub value: String,
    pub signers: Vec<u32>,
    pub signatures: Vec<String>,
}

impl SignedMessage {
    pub fn is_valid(&self, n: u32) -> bool {
        if self.signers.len() != self.signatures.len() {
            return false;
        }
        let mut msg = self.value.clone();
        for i in 0..self.signers.len() {
            let node_id = self.signers[i];
            if node_id < 1 || node_id > n {
                return false;
            }
            if !verify(node_id, &msg, &self.signatures[i]) {
                return false;
            }
            msg = format!("{}:{}:{}", msg, node_id, self.signatures[i]);
        }
        true
    }

    pub fn extend(&self, node_id: u32, _n: u32) -> Self {
        let msg = message_after_signatures(&self.value, &self.signers, &self.signatures);
        let sig = sign(node_id, &msg);
        let mut signers = self.signers.clone();
        signers.push(node_id);
        let mut signatures = self.signatures.clone();
        signatures.push(sig);
        SignedMessage {
            value: self.value.clone(),
            signers,
            signatures,
        }
    }
}

pub struct DolevStrongNode {
    pub node_id: u32,
    n: u32,
    f: u32,
    is_sender: bool,
    pub input_bit: Option<String>,
    pub extracted: HashSet<String>,
    received_this_round: Vec<SignedMessage>,
}

impl DolevStrongNode {
    pub fn new(node_id: u32, n: u32, f: u32, is_sender: bool, input_bit: Option<String>) -> Self {
        Self {
            node_id,
            n,
            f,
            is_sender,
            input_bit,
            extracted: HashSet::new(),
            received_this_round: Vec::new(),
        }
    }

    pub fn round_zero_send(&self) -> Vec<(u32, SignedMessage)> {
        if !self.is_sender {
            return Vec::new();
        }
        let bit = match &self.input_bit {
            Some(b) => b.clone(),
            None => return Vec::new(),
        };
        let sig = sign(self.node_id, &bit);
        let m = SignedMessage {
            value: bit,
            signers: vec![self.node_id],
            signatures: vec![sig],
        };
        (1..=self.n).map(|j| (j, m.clone())).collect()
    }

    pub fn receive(&mut self, messages: &[SignedMessage]) {
        self.received_this_round = messages
            .iter()
            .filter(|m| m.is_valid(self.n) && (m.signers.len() as u32) <= self.f + 1)
            .cloned()
            .collect();
    }

    pub fn round_r_send(&mut self, r: u32) -> Vec<(u32, SignedMessage)> {
        let mut out = Vec::new();
        for m in &self.received_this_round {
            if m.signers.len() as u32 != r {
                continue;
            }
            let b = m.value.clone();
            if !self.extracted.contains(&b) {
                self.extracted.insert(b.clone());
                if m.signers.contains(&self.node_id) {
                    continue;
                }
                let extended = m.extend(self.node_id, self.n);
                for j in 1..=self.n {
                    out.push((j, extended.clone()));
                }
            }
        }
        out
    }

    pub fn output(&self) -> String {
        if self.extracted.len() == 1 {
            return self.extracted.iter().next().cloned().unwrap();
        }
        "0".to_string()
    }
}

// returns node_id -> output bit for each honest node.
pub fn dolev_strong_protocol(
    n: u32,
    f: u32,
    sender_input: &str,
    corrupt_ids: &HashSet<u32>,
) -> HashMap<u32, String> {
    let mut nodes: Vec<DolevStrongNode> = (1..=n)
        .map(|i| {
            DolevStrongNode::new(
                i,
                n,
                f,
                i == 1,
                if i == 1 {
                    Some(sender_input.to_string())
                } else {
                    None
                },
            )
        })
        .collect();

    let mut inbox: HashMap<u32, Vec<SignedMessage>> = (1..=n).map(|i| (i, Vec::new())).collect();

    // round 0
    for node in &nodes {
        if corrupt_ids.contains(&node.node_id) {
            continue;
        }
        for (recipient, msg) in node.round_zero_send() {
            inbox.get_mut(&recipient).unwrap().push(msg);
        }
    }
    for node in &mut nodes {
        let msgs = inbox.get(&node.node_id).cloned().unwrap_or_default();
        node.receive(&msgs);
    }
    inbox = (1..=n).map(|i| (i, Vec::new())).collect();

    // rounds 1..f+1
    for r in 1..=f + 1 {
        for node in &mut nodes {
            if corrupt_ids.contains(&node.node_id) {
                continue;
            }
            for (recipient, msg) in node.round_r_send(r) {
                inbox.get_mut(&recipient).unwrap().push(msg);
            }
        }
        for node in &mut nodes {
            let msgs = inbox.get(&node.node_id).cloned().unwrap_or_default();
            node.receive(&msgs);
        }
        inbox = (1..=n).map(|i| (i, Vec::new())).collect();
    }

    nodes
        .into_iter()
        .filter(|n| !corrupt_ids.contains(&n.node_id))
        .map(|n| (n.node_id, n.output()))
        .collect()
}
