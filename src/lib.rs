// Copyright 2018 unboilerplate Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Functions:
// - remove boilerplate
// - detect boilerplate?
// - switch algorithms? probably no
// - tweak?
#![feature(universal_impl_trait)]

#[macro_use]
extern crate failure;
extern crate kuchiki;

use failure::Error;
use kuchiki::traits::*;
use std::ops::Deref;
use std::str::FromStr;

// TODO needs to hold a reference to the node,
// to be able to compare to adjacent sibling nodes
// for nested anchor
#[derive(Debug, PartialEq)]
struct TextBlock {
    tag: Tag,
    text: String,
    word_count: usize,
    link_density: f32,
}

/// public entrypoint.
/// Input html document, get back non-boilerplate content with no tags.
///
/// Text blocks are any text that is separate by tags from other text.
///
/// Anchors are the only tag not considered a separator.
/// I'll check the siblings of an anchor; if it's equal to the text
/// node on either side, then concatenate.
///
/// I should do this in a third pass
///
/// - pass 1: parse and build html tree
/// - pass 2: naive scan of all text blocks
/// - pass 3: concatenate anchor-separated blocks and compute link density
pub fn unboilerplate(document: &str) -> Result<String, Error> {
    let text_blocks = naive_blocks(document);

    // Apply algorithm here
    
    Ok("".to_owned())
}


/// Produces text block with features (in this case, just word count)
fn naive_blocks(document: &str) -> Result<Vec<TextBlock>, Error> {
    let mut res = vec![];

    let document = kuchiki::parse_html().one(document);

    // TODO filter for h, p, a, div
    for text_element in document.descendants().text_nodes() {
        let tag = text_element.as_node()
            .parent().unwrap()
            .as_element().cloned().unwrap() // TODO use `?`
            .clone()
            .name.local.to_string();

        println!("<{}>: {}", tag, text_element.as_node().to_string());

        let text = text_element.as_node().to_string();
        let word_count = count_words(&text);
        let link_density = 0.;

        res.push(TextBlock{
            tag: tag.parse()?,
            text: text,
            word_count: word_count,
            link_density: link_density,
        })
    }

    Ok(res)
}

fn count_words(text_block: &str) -> usize {
    text_block.split_whitespace().count()
}

#[derive(Debug, PartialEq)]
enum Tag {
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

impl FromStr for Tag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h1" => Ok(Tag::H1),
            "h2" => Ok(Tag::H2),
            "h3" => Ok(Tag::H3),
            "h4" => Ok(Tag::H4),
            "h5" => Ok(Tag::H5),
            "h6" => Ok(Tag::H6),
            "p" => Ok(Tag::P),
            "div" => Ok(Tag::DIV),
            "a" => Ok(Tag::A),
            _ => Err(format_err!("Tag {:?} is not in allowed set", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t1 = "<body><p>adsf adsf <a href=\"url\">asdf</a>end</p></body>";

        println!("text blocks: {:#?}", scan_boilerplate(t1));
        panic!();
    }
}
