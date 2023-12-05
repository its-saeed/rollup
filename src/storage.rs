use anyhow::Result;
use std::{collections::VecDeque, fmt::Display};

use colored::Colorize;
use rs_merkle::{algorithms::Sha256, MerkleTree};

use crate::{Block, State};
pub const SEQ_BLOCKS_PER_DA: usize = 5;

pub struct Storage {
    trusted_blocks: VecDeque<Block>,
    on_da_pending_blocks: VecDeque<Block>,
    on_da_approved_blocks: Vec<Block>,
    state: State,
    current_merkle_tree: Option<MerkleTree<Sha256>>,
    sequencer_lied: usize,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            trusted_blocks: VecDeque::new(),
            on_da_pending_blocks: VecDeque::new(),
            on_da_approved_blocks: vec![],
            state: State::new(),
            current_merkle_tree: None,
            sequencer_lied: 0,
        }
    }

    pub fn load() -> Result<Self> {
        todo!()
    }

    pub fn persist(self) {
        todo!()
    }

    pub fn add_trusted_block(&mut self, block: Block) {
        self.state.update(&block);
        self.current_merkle_tree = Some(self.state.to_merkle_tree());
        self.trusted_blocks.push_back(block);
    }

    pub fn add_da_block(&mut self, block: Block) {
        self.state.update(&block);
        self.on_da_pending_blocks.push_back(block);
        if self.try_approve_da_blocks() {
            println!("  DA block approved");
        }

        if let Some(tree) = &self.current_merkle_tree {
            let next_tree = self.state.to_merkle_tree();
            if tree.root_hex() != next_tree.root_hex() {
                println!("{}", "  Sequencer lied".red());
                self.sequencer_lied += 1;
            }
            self.current_merkle_tree = Some(next_tree);
        }
    }

    pub fn add_da_approved_block(&mut self, block: Block) {
        self.on_da_approved_blocks.push(block);
    }

    pub fn reorg(&mut self, r: usize) {
        if self.on_da_pending_blocks.len() < r {
            println!("  {}: Invalid reorg, only {} blocks are pending but reorg wants to invalidate {} blocks.",
                            "Error".bright_red(), self.on_da_pending_blocks.len(), r);
        } else {
            self.on_da_pending_blocks
                .drain(self.on_da_pending_blocks.len() - r..);
            self.trusted_blocks.drain(
                self.trusted_blocks.len() - ((r + 1) * SEQ_BLOCKS_PER_DA)
                    ..self.trusted_blocks.len() - SEQ_BLOCKS_PER_DA,
            );
        }
    }

    pub fn try_approve_da_blocks(&mut self) -> bool {
        if self.on_da_pending_blocks.len() == 4 {
            self.trusted_blocks.drain(..SEQ_BLOCKS_PER_DA);
            self.on_da_approved_blocks
                .push(self.on_da_pending_blocks.pop_front().unwrap());
            true
        } else {
            false
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\n{}{}\n{}{}\n{}{}",
            "Trusted Blocks: ".bold(),
            self.trusted_blocks.len(),
            "Pending blocks: ".bold().yellow(),
            self.on_da_pending_blocks.len(),
            "Approved blocks: ".bold().bright_green(),
            self.on_da_approved_blocks.len(),
            "Sequencer lied: ".bold().red(),
            self.sequencer_lied
        )
    }
}
