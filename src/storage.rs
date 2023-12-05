use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};

use colored::Colorize;

use crate::{Block, State};
pub const SEQ_BLOCKS_PER_DA: usize = 5;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    trusted_blocks: VecDeque<Block>,
    on_da_pending_blocks: VecDeque<Block>,
    on_da_approved_blocks: Vec<Block>,
    #[serde(skip)]
    state: State,
    sequencer_lied: usize,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            trusted_blocks: VecDeque::new(),
            on_da_pending_blocks: VecDeque::new(),
            on_da_approved_blocks: vec![],
            state: State::new(),
            sequencer_lied: 0,
        }
    }

    pub fn add_trusted_block(&mut self, block: Block) {
        self.state.update(&block);
        self.trusted_blocks.push_back(block);
    }

    pub fn add_da_block(&mut self, block: Block) {
        let prev_root_hash = self.state.root_hash();
        self.state.update(&block);
        self.on_da_pending_blocks.push_back(block);
        self.state.pin();
        if self.try_approve_da_blocks() {
            println!("  DA block approved");
        }

        let current_root_hash = self.state.root_hash();
        if prev_root_hash != current_root_hash {
            println!("{}", "  Sequencer lied".red());
            self.sequencer_lied += 1;
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
            self.state.rollback(r);
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
