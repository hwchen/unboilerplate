use ::block::Block;

pub fn num_words(blocks: Vec<Block>) -> String {
    let mut res = String::new();

    // first block
    if let Some(curr) = blocks.get(0) {
        if curr.link_density() <= 0.333333 {
            if curr.word_count <= 16 {
                if let Some(next) = blocks.get(1) {
                    if next.word_count > 15 {
                        res.push_str(curr.as_text());
                    }
                }
            }
        }
    }

    // main section
    for window in blocks.windows(3) {
        let prev = &window[0];
        let curr = &window[1];
        let next = &window[2];

        if curr.link_density() <= 0.3333333 {
            if prev.link_density() <= 0.555556 {
                if curr.word_count <= 16 {
                    if next.word_count <= 15 {
                        if prev.word_count > 4 {
                            res.push_str(curr.as_text());
                        }
                    } else {
                        res.push_str(curr.as_text());
                    }
                } else {
                    res.push_str(curr.as_text());
                }
            } else {
                if curr.word_count <= 40 {
                    if next.word_count > 17 {
                        res.push_str(curr.as_text());
                    }
                } else {
                    res.push_str(curr.as_text());
                }
            }
        }
    }

    // for last block
    if let Some(curr) = blocks.last() {
        if let Some(prev) = blocks.get(blocks.len()-2) {
            if prev.link_density() <= 0.555556 {
                if curr.word_count <= 16 {
                    if prev.word_count > 4 {
                        res.push_str(curr.as_text());
                    }
                } else {
                    res.push_str(curr.as_text());
                }
            } else {
                if curr.word_count > 40 {
                    res.push_str(curr.as_text());
                }
            }
        }
    }

    res
}
