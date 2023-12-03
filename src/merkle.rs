use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};

pub fn create_merkle_tree(state: &[u64]) -> MerkleTree<Sha256> {
    let leaves: Vec<[u8; 32]> = state
        .iter()
        .map(|x| Sha256::hash(&x.to_le_bytes()))
        .collect();

    MerkleTree::<Sha256>::from_leaves(&leaves)
}
