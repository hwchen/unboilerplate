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

extern crate failure;

struct TextBlock<'a> {
    text: &'a str,
    word_count: usize,
}

impl<'a> TextBlock<'a> {

    fn new(text: &'a str, is_boilerplate: bool) -> Self {
        TextBlock {
            text: text,
            is_boilerplate: is_boilerplate,
        }
    }
}

/// public entrypoint.
/// Input html document, get back content with outer tags removed.
/// Tags internal to text are not removed.
/// What about nested div tags? Do we just look at the block from the
/// outermost div tag?
pub fn unboilerplate(document: &str) -> String {
    analyze_boilerplate(document)
        .iter()
        .filter(|text_block| !text_block.is_boilerplate())
        .map(|text_block| text_block.text())
        .collect()
}

/// Produces text block with features (in this case, just word count)
fn scan_boilerplate<'b>(document: &'b str) -> Vec<TextBlock<'b>> {
    let mut input = document.char_indices();

    let mut chunks = Vec::new();

    loop {
        match input.next() {
            Some((_, '<')) => {
                match get_block(&mut input) {
                    Some(chunk) => chunks.push(chunk),
                    None => continue,
                }
            }
            Some(_) => continue,
            None => break,
        };
    }
    vec![]
}

const HEADER_NUMS: [char; 6] = ['1', '2', '3', '4', '5', '6'];

fn get_block(input: impl Iterator<Item=(usize,char)>) -> Option<Chunk> {
    // first check tag
    // Angle brackets always indicate tag
    match input.next() {
        Some(_, c) if c == 'h' or c == 'H' => {
            match input.next() {
                Some(_, c) if HEADER_NUMS.contains(c) => {
                    
                },
                _ => return None,
            }
        },
        Some(_, c) if c == 'p' or c == 'P' => {
        },
        Some(_, c) if c == 'd' or c == 'D' => {
        },
        None => return None,
    }
    None
}

// All the tags that we are checking for.
enum Tag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    DIV,
//    A,
}

fn count_words(text_block: &str) -> usize {
    text_block.split_whitespace().count()
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let t1 = "<body><p>adsf adsf asdf</p></body>";
        let t2 = "<body> < <p>adsf adsf asdf</p></body>";

        assert_eq!(2 + 2, 4);
    }
}
