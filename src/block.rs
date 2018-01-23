/// Block is for internal use only, so it's pub from
/// this module, but not pub mod in lib.rs
use failure::Error;
use std::str::FromStr;

use ::util::count_words;

#[derive(Debug, PartialEq)]
pub struct Block {
    pub tag: BlockTag,
    pub text: String,
    pub word_count: usize,
    pub anchor_word_count: usize,
}

impl Block {
    pub fn new(tag: BlockTag, text: String) -> Self {
        let word_count = count_words(&text);

        let anchor_word_count = if tag == BlockTag::A {
            word_count
        } else {
            0
        };

        Block {
            tag: tag,
            text: text,
            word_count: word_count,
            anchor_word_count: anchor_word_count,
        }
    }

    pub fn tag(&self) -> &BlockTag {
        &self.tag
    }

    pub fn as_text(&self) -> &str {
        self.text.as_str()
    }

    pub fn link_density(&self) -> f32 {
        self.anchor_word_count as f32 / self.word_count as f32
    }
}

/// Tags used to delinieate the text blocks
/// used for analysis
#[derive(Debug, PartialEq)]
pub enum BlockTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    DIV,
    A,
}

impl FromStr for BlockTag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h1" => Ok(BlockTag::H1),
            "h2" => Ok(BlockTag::H2),
            "h3" => Ok(BlockTag::H3),
            "h4" => Ok(BlockTag::H4),
            "h5" => Ok(BlockTag::H5),
            "h6" => Ok(BlockTag::H6),
            "p" => Ok(BlockTag::P),
            "div" => Ok(BlockTag::DIV),
            "a" => Ok(BlockTag::A),
            _ => Err(format_err!("Tag {:?} is not a text block tag", s)),
        }
    }
}

