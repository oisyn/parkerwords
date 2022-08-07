use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Returns an Iterator to the Reader of the lines of the file.
///
/// The output is wrapped in a Result to allow matching on errors
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn getbits(word: &String) -> usize {
    let mut result: usize = 0;
    for c in word.chars() {
        result |= 1 << (c as usize - 'a' as usize);
    }
    result
}

pub fn readwords(
    bits_to_index: &mut HashMap<usize, usize>,
    index_to_bits: &mut Vec<usize>,
    index_to_word: &mut Vec<String>,
    letterindexes: &mut [Vec<usize>; 26],
    letterorder: &mut [usize; 26],
    fixed_size: Option<usize>,
) -> io::Result<()> {
    #[derive(Copy, Clone)]
    struct Frequency {
        f: usize,
        l: usize,
    }

    let mut freq: [Frequency; 26] = array_init::array_init(|i: usize| Frequency { f: 0, l: i });

    // read words
    for line in read_lines("words_alpha.txt")? {
        let line: String = line?;
        if let Some(sz) = fixed_size {
            if line.len() != sz {
                continue;
            }
        } else if line.len() < 7 {
            continue;
        }

        let bits = getbits(&line);

        if bits.count_ones() as usize != line.len() {
            continue;
        }

        if bits_to_index.contains_key(&bits) {
            continue;
        }

        // count letter frequency
        for c in line.chars() {
            freq[c as usize - 'a' as usize].f += 1;
        }

        bits_to_index.insert(bits, index_to_bits.len());
        index_to_bits.push(bits);
        index_to_word.push(line);
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

    Ok(())
}
