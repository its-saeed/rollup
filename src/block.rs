use anyhow::{anyhow, Result};
use std::{ops::Deref, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Transaction {
    pub index: u8,
    pub value: u64,
}

impl Transaction {
    pub fn new(index: u8, value: u64) -> Self {
        Self { index, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block(Vec<Transaction>);

impl Deref for Block {
    type Target = Vec<Transaction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<Transaction> for Block {
    fn from_iter<T: IntoIterator<Item = Transaction>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl FromStr for Block {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        line.split(", ")
            .map(|txn_str| {
                let split_index = txn_str
                    .find(' ')
                    .ok_or(anyhow!("Invalid transaction, no space found\"{}\"", line))?;
                let index = txn_str[..split_index].parse()?;
                let value = txn_str[split_index + 1..].parse()?;
                Ok(Transaction { index, value })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, Transaction};

    #[test]
    fn test_parse_sequencer_block_with_valid_input() {
        let line = "183 17515376014989053548, 91 15969664550135497955, 112 9701954221909584715";
        let block: Block = line.parse().unwrap();

        assert_eq!(
            block,
            Block(vec![
                Transaction::new(183, 17515376014989053548),
                Transaction::new(91, 15969664550135497955),
                Transaction::new(112, 9701954221909584715)
            ])
        );
    }
}
