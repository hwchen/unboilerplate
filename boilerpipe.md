# UnBoilerplate

# Introduction

This is an article on reimplementing the java library [Boilerpipe](https://code.google.com/archive/p/boilerpipe/), which uses shallow text features to remove boilerplate from web pages (base on this [paper](http://www.l3s.de/~kohlschuetter/publications/wsdm187-kohlschuetter.pdf)).

I'm write this library as an exercise in implementing a CS paper. This was actually my first time trying this, inspired by a post by [burntsushi](https://www.reddit.com/r/rust/comments/7om1t3/article_extraction_in_rust/dsawqww/)

I'm a mostly self-taught programmer, and most of my projects so far have been more utilitarian: cli interfaces to services, or ports of utility libraries. However, I've been itching for more of a challenge, especially into a more algorithmic arena (as opposed to architectural or engineering).

The reimplementation of the Boilerpipe algorithms met the following criteria which made it a good project for my level:

- Not yet implemented in Rust, so I feel like I'll be doing something useful.
- Doesn't seem too difficult on an algorithmic level, from skimming the paper.
- Relatively small in scope.
- Relatively self-contained (i.e. didn't require a lot of domain knowledge, like say machine learning or GIS might)
- Recommended by somebody I respect (gives a little extra motivation).

I work in a mostly data engineering role, and web scraping in particular is not a subject I care much about. (I tend to think more about dataframes and databases on a day-to-day basis). But I think that the subject matter was not as important as the fact that it was both useful to somebody and didn't require too much preparation in the domain before starting.

I started this project on 1/14/2019 in earnest (starting this blog post).

# Boilerplate Detection: Background

[The paper](http://www.l3s.de/~kohlschuetter/publications/wsdm187-kohlschuetter.pdf)).

The goal is to look at an HTML document and remove "boilerplate": the parts of the document that aren't the main content. This may include navigation, footnotes, ads, etc.

One approach is to inspect the structure (i.e. html tags or css classes). The main downsides are that 1) the usage of structural elements is generally site-specific, and 2)understanding the document requires doing enough rendering to understand the semantics of the css.

Although HTML structures are not ideal for doing complete analysis of boilerplate, they are still useful for some basic comprehension. The paper looks at the tags: `H1`, `h2`, `H3`, `H4`, `H5`, `H6`, `P`, `DIV`, and `A`.

Instead of using structural features for most of the analysis, the paper uses shallow text features. The meaning of each word (i.e. for example, finding the meaning of a word by measuring its relationship to surrounding words) or the topic of a text is not important. Rather, we look at features that describe more generic, higher-level, language-independent aspects of text.

Features that were considered include:

- Average word length
- Average sentence length
- Position of text block, absolute and relative
- Absolute number of capitalized words (titlecase or all uppercase)
- Relative number of capitalized words (titlecase or all uppercase)
- Relative number of full stops (`.` per word)
- Number of date/time related tokens
- Number of vertical bars (`|`)
- Link density (i.e. tokens in an `A` block per tokens in surrounding text block)
- Text density

Of these, a combination of the position of the text block with either text density or number of words was the most effective for finding boilerplate blocks using the minimal number of features.

### Position of Text Block

The position of a text block generally relates to whether it's boilerplate. An example fo absolute position is that text at the end of a page is generally boilerplate, even if it has sentences resembling main text; this may be a footnote or copyright notices.

For relative position, we can notice that main text is generally surrounded by boilerplate, and not the other way around.

### Text Density and Number of Words

The ratio of:

1) Tokens in text block

divided by

2) Lines in text block (after word-wrapping the text in a fixed-width column)

In general, a text density >= 10 will be normal text (whether main text, user comments, or supplemental text). A lower density will often indicate boilerplate.

Number of words are closely related to text density, in that more words will often mean a higher text density.

(More on linguistic interpretation)
short text -> functional text
long text -> descriptive text (content)

# Implementation

Goals: Accuracy and speed

Choice of features

## Algorithm
