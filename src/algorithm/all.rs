use ::block::Block;

pub fn all(blocks: Vec<Block>) -> String {
    blocks.iter()
        .map(|block| block.as_text())
        .collect()
}
