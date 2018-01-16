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

//mod features;

pub struct TextBlock<'a> {
    is_boilerplate: bool,
    text: &'a str,
}

pub fn unboilerplate<'b>(document: &'b str) -> Vec<TextBlock<'b>> {
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
