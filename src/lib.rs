use std::{collections::HashMap, fs, io};

const MIN_WORD_SIZE: usize = 1;

pub fn readwords(
    bits_to_index: &mut HashMap<usize, usize>,
    index_to_bits: &mut Vec<usize>,
    index_to_word: &mut Vec<String>,
    letterindexes: &mut [Vec<usize>; 26],
    letterorder: &mut [usize; 26],
    fixed_size: Option<usize>,
) -> io::Result<String> {
    #[derive(Copy, Clone)]
    struct Frequency {
        f: usize,
        l: usize,
    }

    let file: String = fs::read_to_string("words_alpha.txt")?;
    println!("{}", file.len());

    let mut freq: [Frequency; 26] = array_init::array_init(|i: usize| Frequency { f: 0, l: i });

    // read words
    let mut word_begin: usize = 0;
    let mut bits: usize = 0;
    for (i, char) in file.chars().enumerate() {
        // _technically_ this loop will not work for the last word
        // In practice the last word has a duplicate letter so we don't care
        if char != '\n' {
            bits |= 1 << (char as usize - 'a' as usize);
            continue;
        }

        let len = i - word_begin;
        let this_bits = bits;
        let this_word_begin = word_begin;
        word_begin = i + 1;
        bits = 0;
        if let Some(sz) = fixed_size {
            if len != sz {
                continue;
            }
        } else if len < MIN_WORD_SIZE {
            continue;
        }

        if this_bits.count_ones() as usize != len {
            continue;
        }

        if bits_to_index.contains_key(&this_bits) {
            continue;
        }

        // count letter frequency
        for c in file[this_word_begin..i].chars() {
            freq[c as usize - 'a' as usize].f += 1;
        }

        bits_to_index.insert(this_bits, index_to_bits.len());
        index_to_bits.push(this_bits);
        index_to_word.push(file[this_word_begin..i].to_string());
    }

    // rearrange letter order based on lettter frequency (least used letter gets lowest index)
    freq.sort_by(|a, b| a.f.cmp(&b.f));

    let mut reverseletterorder: [usize; 26] = [0; 26];

    for i in 0..26 {
        letterorder[i] = freq[i].l;
        reverseletterorder[freq[i].l] = i;
    }

    for w in index_to_bits {
        let mut m: usize = *w;
        let mut min = 26;

        while m != 0 {
            let letter = m.trailing_zeros() as usize;
            min = std::cmp::min(min, reverseletterorder[letter]);

            m ^= 1 << letter;
        }

        letterindexes[min].push(*w);
    }

    // build index based on least used letter

    Ok(file)
}
