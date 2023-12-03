use crate::Block;
use anyhow::Result;

pub fn update_state(state: &mut [u64], block: &Block) {
    block.iter().for_each(|x| state[x.index as usize] = x.value);
}

pub fn persist_state(state: &[u64]) -> Result<()> {
    let state_string = state
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    std::fs::write("state.db", state_string)?;
    Ok(())
}

pub fn load_state() -> [u64; 256] {
    let mut state = [0_u64; 256];
    if let Ok(s) = std::fs::read_to_string("state.db") {
        s.split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .enumerate()
            .for_each(|(index, value)| state[index] = value);
    }

    state
}
