# Unboilerplate

A library to remove boilerplate from web pages.

Based on [this](http://www.l3s.de/~kohlschuetter/publications/wsdm187-kohlschuetter.pdf) paper:

Christian Kohlschütter, Peter Fankhauser and Wolfgang Nejdl,
Boilerplate Detection using Shallow Text Features,
WSDM 2010 -- The Third ACM International Conference on Web Search and Data Mining New York City, NY USA.

## Some thoughts on implentation algorithm:

Algorithm requires 1 pass through the text:
    divide into blocks and count as we go.

Next, apply the algorithm knowing each block's relative
position and word count

Don't parse the html. Just go character by character? Will need to
    store state for that.

What's the alternative? I think the libs parse everything to create a tree
that I don't need.

I don't think that I can just split on whitespace;
- does the iterator split lazily?
- But in any case, the splitting is a little complicated, on both
    spaces and on tags. Can I use split for that? The split on tags
    also needs to identify the tag.

Or would it still be faster to use something like aho-corasick
to find the tag indexes to split on first?

Probably not, since the boilerplate algorithm requires looking
over every character anyways, and using the aho-corasick would
create two passes (even if one pass was fast).

Choices:
- use html parsing lib first (check implementation)
- use string matching to find tags (check implementation)
- use a parser library
- hand parse by splitting (on whitespace?) (How to split out tags?)
- hand parse by character

Hand parsing is probably best, because I want to count as I parse.
But counting using split would be very fast

First round I'll do pest and split.
Second round I'll try hand parsing

See which is faster

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
