use super::{blockchain::Blockchain, structure::block::Block};
use std::fs::File;

pub struct BlockchainIterator<'a> {
    pub blockchain: &'a Blockchain,
    pub file: File,
    pub offset: u64,
}

impl<'a> Iterator for BlockchainIterator<'a> {
    type Item = Result<Block, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.blockchain.size {
            return None;
        }

        let buff = match self
            .blockchain
            .read_block_buffer_file_at(&mut self.file, self.offset)
        {
            Ok((buff, _)) => buff,
            Err(e) => return Some(Err(e)),
        };

        self.offset += buff.len() as u64;
        Some(Block::from_buffer(&buff))
    }
}
