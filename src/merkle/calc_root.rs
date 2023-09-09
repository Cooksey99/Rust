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
//!                   hash                               hash
//!            /                  \                 /            \
//!         hash                 hash             hash           hash
//!      /        \          /        \         /      \       /      \         
//!   hash       hash      hash      hash     hash    hash   hash    hash
//!  /   \      /    \    /    \    /   \    /   \   /   \   /   \   /  \
//! The quick brown fox jumps over the lazy dog  "" ""   "" ""   "" ""   ""
//! 
//! Notice each hash is a concatenation of the hashes of the blocks below it. 
//! 

use std::{hash::{self, Hasher}, collections::hash_map::DefaultHasher};
use crate::merkle::calc_root::hash::Hash;


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

/// The hashing algorithm is not specified, so we will use a simple DefaultHasher.
/// Let this be algorithm for hashing. We will call this function with our data to hash it.
/// Notice that we are using a trait bound here. This is so we can hash any type that implements the Hash trait.
pub fn hash<T: Hash>(t: &T) -> HashValue {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

/// This is where we concatenate the hash values.
/// There are better ways to do this, but the purpose of this is to demonstrate the concept.
pub fn concatenate_hash_values(left: HashValue, right: HashValue) -> HashValue {
    let left = left.to_le_bytes();
    let right = right.to_le_bytes();

    let mut cmb: Vec<u8> = Vec::with_capacity(left.len() + right.len());
    cmb.extend_from_slice(&left);
    cmb.extend_from_slice(&right);
    hash(&cmb)
}


pub fn calc_root(sentence: &str) -> HashValue {
    let mut leafs = sentence.split_whitespace().collect::<Vec<&str>>();
    pad_base_layer(&mut leafs);

    let mut queue = leafs
        .iter()
        .rev()
        .map(|x| hash(x))
        .collect::<Vec<HashValue>>();

    while queue.len() > 1 {
        let pair: (u64, u64) = (queue.pop().unwrap(), queue.pop().unwrap());
        let hash: u64 = concatenate_hash_values(pair.0, pair.1);
        queue.insert(0, hash);
    }

    queue[0]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_root_test() {
        let data = "My name is Jeff";
        let root = calc_root(data);

        

        for word in data.split(" ").into_iter() {
            println!("word:    {:?}", word);

        }


    }
}