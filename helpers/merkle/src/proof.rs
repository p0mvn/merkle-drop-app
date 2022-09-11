use crate::hash;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub is_left_sibling: bool,
    pub hash: hash::Hash,
}

impl Entry {
    pub fn new(is_left_sibling: bool, hash: hash::Hash) -> Self {
        let entry = Entry {
            is_left_sibling: is_left_sibling,
            hash: hash,
        };
        entry
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Proof(Vec<Entry>);

impl Proof {
    pub fn push(&mut self, is_left_sibling: bool, hash: hash::Hash) {
        self.0.push(Entry {
            is_left_sibling: is_left_sibling,
            hash: hash,
        })
    }

    pub fn verify<T: AsRef<[u8]>>(&self, data: &T, root: &hash::Hash) -> bool {
        let initial_hash: hash::Hash = hash::leaf(data.as_ref());

        let result = self.0.iter().try_fold(initial_hash, |cur_hash, entry| {
            let is_entry_left: bool = entry.is_left_sibling;
            if is_entry_left {
                Some(hash::branch(&entry.hash, &cur_hash))
            } else {
                Some(hash::branch(&cur_hash, &entry.hash))
            }
        });

        if result.is_none() {
            return false;
        }

        return result.unwrap().eq(root);
    }

    pub fn get_entry_at(&self, index: usize) -> &Entry {
        return &self.0[index];
    }

    pub fn get_num_entries(&self) -> usize {
        return self.0.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;
    use crate::Tree;

    #[test]
    fn verify_works() {
        let items: Vec<&[u8]> = vec![
            test_util::OSMO,
            test_util::ION,
            test_util::WETH,
            test_util::USDC,
            test_util::AKT,
        ];

        let mt = Tree::new(&items);

        let proof = mt.find_proof(&test_util::USDC).unwrap();

        let tree_root = &mt.get_root().unwrap();

        // successfuly verify node's proof.
        assert_eq!(true, proof.verify(&test_util::USDC, tree_root));

        // fail to verify other node in tree.
        assert_eq!(false, proof.verify(&test_util::OSMO, tree_root));

        // fail to verify invalid root.
        assert_eq!(false, proof.verify(&test_util::USDC, &hash::leaf(test_util::USDC)));
    }
}
