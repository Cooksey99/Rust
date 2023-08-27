//! This module contains the functions to calculate the root of a merkle tree.
//! 
//! In a merkle tree, the root is the hash of all the hashes of the blocks.
//! Unlike a binary tree, data is not stored in each node. Instead, data is stored in the leaves.
//! For non-leaf nodes (branches), the "data" is the hash of the data of its children.
//! 
//! Take this example:
//! Assume we are storing the following sentence in a merkle tree:
//! "The quick brown fox jumps over the lazy dog"
//! 
//! The first thing we check is if the length of the data/blocks is a power of 2.
//! In this case, there are 9 words to store, therefore, it is not a power of two. 
//! We need to pad the base layer with empty strings (or whatever value the protocol specifies) before we can continue.
//! That would then give us 16 blocks, which is a power of 2:
//! "The", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog", "", "", "", "", "", "", ""
//! 
//! To calculate the root, we concatenate the hashes of the blocks, as pairs, from left to right.
//! We move up a layer, and concatenate the hashes of the hashes, as pairs and so on, until we reach the root.
//! Drawing it out would look like this (The "" represents the empty strings we padded the base layer with):
//!                                     root
//!                     /                                \                   
//!                   hash                              hash
//!            /                  \                 /            \
//!         hash                 hash             hash           hash
//!      /        \          /        \         /      \       /      \         
//!   hash       hash      hash      hash     hash    hash   hash    hash
//!  /   \      /    \    /    \    /   \    /   \   /   \   /   \   /  \
//! The quick brown fox jumps over the lazy dog  "" ""   "" ""   "" ""   ""
//! 
//! Notice each hash is a concatenation of the hashes of the blocks below it. 
//! 

use std::hash;



/// A hash value is a 64 bit unsigned integer.
/// We could choose to use a u64, rather than a type, but this is more explicit and allows easier modifications.
pub type HashValue = u64;


/// A merkle tree must be balanced, so we pad the base layer with empty strings
/// More specifically, a merkle tree must be a power of 2.
pub fn pad_base_layer(blocks: &mut Vec<&str>) {
    let value: &str = "";

    while blocks.len() & (blocks.len() - 1) != 0 {
        blocks.push(value);
    }
}

/// This is where we concatenate the hash values.
/// The hashing algorithm is not specified, so we will use a simple SHA256.
pub fn concatenate_hash_values(mut left: HashValue, mut right: HashValue) -> HashValue {
    todo!()
}


pub fn calc_root() {
    todo!()
}



#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        // ...
    }
}