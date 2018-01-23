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

mod algorithm;
mod block;
mod scan;
mod util;

use failure::Error;

use scan::scan;

/// public entrypoint.
/// Input html document, get back non-boilerplate content with no tags.
///
/// Text blocks are any text that is separate by tags from other text.
///
/// Anchors are the only tag not considered a separator.
/// I'll check the siblings of an anchor; if it's equal to the text
/// node on either side, then concatenate.
///
/// - pass 1: parse and build html tree
/// - pass 2: naive scan of all text blocks and concatenate anchor blocks
/// - pass 3: compute link density
///
/// For the algorithm selection, should I use enums and match, or
/// Algorithm trait?
///
/// TODO how do I make Block not a public interface?
pub fn unboilerplate(document: &str) -> Result<String, Error> {
    // pass 1 and 2: parse to html tree, naive scan
    // of all text blocks
    let blocks = scan(document)?;

    // Apply algorithm here

    Ok(algorithm::all(blocks))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_unboilerplate() {
        let t1 = "\n<body><p>adsf adsf <a href=\"url\">asdf</a> end</p>
        <div>horseshoes are round</div></body>\n";

        println!("{}", t1);
        println!("text blocks: {:#?}", unboilerplate(t1));
        panic!();
    }
}
