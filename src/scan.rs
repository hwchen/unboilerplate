use failure::Error;
use kuchiki;
use kuchiki::traits::*;
use kuchiki::{NodeRef, NodeDataRef};
use std::cell::RefCell;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Block {
    tag: BlockTag,
    text: String,
    word_count: usize,
    anchor_word_count: usize,
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

/// Produces text block with features (in this case, just word count)
pub fn scan(document: &str) -> Result<Vec<Block>, Error> {
    let mut blocks = vec![];

    let document = kuchiki::parse_html().one(document);

    let mut text_nodes = document.descendants().text_nodes();

    loop {
        if let Some(text_node) = text_nodes.next() {
            // This filters for text block separating tags only
            let tag = match text_block_tag(&text_node) {
                Ok(t) => t,
                Err(_) => { continue; },
            };

            if tag == BlockTag::A {
                concat_or_push_anchor(text_node, &mut text_nodes, &mut blocks)?;
            } else {
                push_block(tag, text_node, &mut blocks);
            }
        } else {
            // reach the end of text_nodes
            break;
        }
    }

    Ok(blocks)
}

/// Gets parent tag from text node
/// Fails if can't parse into tag that
/// is used for separating chunk separation
fn text_block_tag(text_node: &NodeDataRef<RefCell<String>>) -> Result<BlockTag, Error> {
    try_parent_blocktag(text_node.as_node())
}

/// Tries to get parent node's tag,
/// succeeds only if the tag is a BlockTag
fn try_parent_blocktag(node: &NodeRef) -> Result<BlockTag, Error> {
    node
        .parent()
        .ok_or(format_err!("{:?} has no parent", node.to_string()))?
        .as_element()
        .ok_or(format_err!("{:?} parent not an element", node.to_string()))?
        .name.local.to_string()
        .parse()
}

fn push_block(
    tag: BlockTag,
    text_node: NodeDataRef<RefCell<String>>,
    blocks: &mut Vec<Block>,
    )
{
    let text = text_node.as_node().to_string();

    blocks.push(Block::new(tag, text));
}

/// If an anchor text is surrounded by text nodes, then
/// concatenate with the surrounding text
/// but if following is not text, just append anchor.
fn concat_or_push_anchor(
    text_node: NodeDataRef<RefCell<String>>,
    text_nodes: &mut impl Iterator<Item=NodeDataRef<RefCell<String>>>,
    blocks: &mut Vec<Block>,
    ) -> Result<(), Error>
{
    let a_node = text_node.as_node()
        .parent()
        .expect("Anchor node parent must already exist");

    // First check if parent of anchor node is text block
    // If not, then return an error that anchor is not
    // in a text block, and therefore cannot be pushed
    // TODO understand with_context better
    // TODO write tests for case where anchor node is nested
    // inside non-text block with text

    let text = text_node.as_node().to_string();
    let mut anchor_block = Block::new(BlockTag::A, text);

    // Then check if there's text to concatenate to on either side
    // first concatenate next text into anchor, then
    // anchor into previous. This will use the fewest allocations
    // and movement?
    // (algo designed for doing following node first, if changing
    // make sure to take order into consideration.

    if let Some(following_node) = a_node.following_siblings().next() {
        if following_node.as_text().is_some() {
            if let Some(following_node) = text_nodes.next() {
                let text = following_node.as_node().to_string();
                anchor_block.text.push_str(&text);
                anchor_block.word_count += count_words(&text);
                // the following node anchor word count must be 0
            }
        }
    }

    if let Some(previous_node) = a_node.preceding_siblings().next() {
        if previous_node.as_text().is_some() {
            if let Some(previous_block) = blocks.last_mut() {
                previous_block.text.push_str(&anchor_block.text);
                previous_block.word_count += anchor_block.word_count;
                previous_block.anchor_word_count += anchor_block.anchor_word_count;
            }
        }
    } else {
        blocks.push(anchor_block);
    }

    Ok(())
}

fn count_words(text_block: &str) -> usize {
    text_block.split_whitespace().count()
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_concatenate() {
        let t1 = "\n<body><p>one <a href=\"url\">two</a> three</p></body>\n";
        let t2 = "\n<body><p>one <a href=\"url\">two</a></p><p>three</p></body>\n";
        let t3 = "\n<body><p>one</p><p><a href=\"url\">two</a> three</p></body>\n";
        let t4 = "\n<body><p><a href=\"url\">two</a></p></body>\n";

        assert_eq!(
            vec![
                Block {
                        tag: BlockTag::P,
                        text: "one two three".to_owned(),
                        word_count: 3,
                        anchor_word_count: 1
                    }
            ],
            scan(t1).unwrap()
        );

        assert_eq!(
            vec![
                Block {
                    tag: BlockTag::P,
                    text: "one two".to_owned(),
                    word_count: 2,
                    anchor_word_count: 1
                },
                Block {
                    tag: BlockTag::P,
                    text: "three".to_owned(),
                    word_count: 1,
                    anchor_word_count: 0
                }
            ],
            scan(t2).unwrap()
        );

        assert_eq!(
            vec![
                Block {
                    tag: BlockTag::P,
                    text: "one".to_owned(),
                    word_count: 1,
                    anchor_word_count: 0
                },
                Block {
                    tag: BlockTag::A,
                    text: "two three".to_owned(),
                    word_count: 2,
                    anchor_word_count: 1
                }
            ],
            scan(t3).unwrap()
        );

        assert_eq!(
            vec![
                Block {
                    tag: BlockTag::A,
                    text: "two".to_owned(),
                    word_count: 1,
                    anchor_word_count: 1
                }
            ],
            scan(t4).unwrap()
        );
    }

}

