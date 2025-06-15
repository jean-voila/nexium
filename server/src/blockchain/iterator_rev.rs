use super::{
    blockchain::Blockchain,
    structure::{block::Block, block_header::HeaderPreviousBlockHash},
};
use std::fs::File;

pub struct BlockchainIteratorReverse<'a> {
    pub blockchain: &'a Blockchain,
    pub file: File,
    pub hash: HeaderPreviousBlockHash,
}

impl<'a> Iterator for BlockchainIteratorReverse<'a> {
    type Item = Result<Block, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hash == HeaderPreviousBlockHash::default() {
            return None;
        }

        let block = match self
            .blockchain
            .get_block_file_from_hash(&mut self.file, &self.hash)
        {
            Ok(block) => block,
            Err(e) => return Some(Err(e.to_string())),
        };
        self.hash = block.header.previous_block_hash;

        Some(Ok(block))
    }
}
