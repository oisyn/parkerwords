use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn getbits(word: &String) -> usize {
    let mut result: usize = 0;
    for c in word.chars() {
        result |= 1 << (c as usize - 'a' as usize);
    }
    result
}

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

fn readwords(
    bits_to_index: &mut HashMap<usize, usize>,
    index_to_bits: &mut Vec<usize>,
    index_to_word: &mut Vec<String>,
    letterindexes: &mut [Vec<usize>; 26],
    letterorder: &mut [usize; 26],
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
        if line.len() != 5 {
            continue;
        }

        let bits = getbits(&line);

        if bits.count_ones() != 5 {
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

fn findwords(
    letterorder: &[usize; 26],
    letterindexes: &[Vec<usize>; 26],
    bits_to_index: &HashMap<usize, usize>,
    index_to_word: &Vec<String>,

    totalbits: usize,
    numwords: usize,
    words: &mut [usize; 5],
    max_letter: usize,
    mut skipped: bool,
) -> usize {
    if numwords == 5 {
        output(index_to_word, words);
        return 1;
    }

    let mut numsolutions: usize = 0;

    // walk over all letters in a certain order until we find an unused one
    for i in max_letter..26 {
        let m: usize = 1 << letterorder[i];
        if totalbits & m != 0 {
            continue;
        }

        // take all words from the index of this letter and add each word to the solution if all letters of the word aren't used before.

        // Use parallelism at the top level only
        if numwords == 0 {
            numsolutions += letterindexes[i]
                .par_iter()
                .map(|w| {
                    if totalbits & w != 0 {
                        0usize
                    } else {
                        let idx: usize = bits_to_index[&w];
                        let mut newwords = words.clone();
                        newwords[numwords] = idx;
                        findwords(
                            letterorder,
                            letterindexes,
                            bits_to_index,
                            index_to_word,
                            totalbits | w,
                            numwords + 1,
                            &mut newwords,
                            i + 1,
                            skipped,
                        )
                    }
                })
                .sum::<usize>()
        } else {
            for w in letterindexes[i].iter() {
                if totalbits & w != 0 {
                    continue;
                }

                let idx: usize = bits_to_index[&w];
                words[numwords] = idx;
                numsolutions += findwords(
                    letterorder,
                    letterindexes,
                    bits_to_index,
                    index_to_word,
                    totalbits | w,
                    numwords + 1,
                    words,
                    i + 1,
                    skipped,
                );
            }
        }

        if skipped {
            break;
        }
        skipped = true;
    }

    numsolutions
}

fn output(index_to_word: &Vec<String>, words: &[usize; 5]) -> () {
    if true || cfg!(debug_assertions) {
        println!(
            "{} {} {} {} {}",
            index_to_word[words[0]],
            index_to_word[words[1]],
            index_to_word[words[2]],
            index_to_word[words[3]],
            index_to_word[words[4]]
        );
    }
}

fn main() {
    let mut bits_to_index: HashMap<usize, usize> = HashMap::new();
    let mut index_to_bits: Vec<usize> = Vec::new();
    let mut index_to_word: Vec<String> = Vec::new();
    let mut letterindexes: [Vec<usize>; 26] = Default::default();
    let mut letterorder: [usize; 26] = [0; 26];

    // TODO: Add error handling
    readwords(
        &mut bits_to_index,
        &mut index_to_bits,
        &mut index_to_word,
        &mut letterindexes,
        &mut letterorder,
    )
    .unwrap();

    // OUTPUT(
    //     std::cout << wordbits.size() << " unique words\n";
    //     std::cout << "letter order: ";
    //     for (int i = 0; i < 26; i++)
    // 	    std::cout << char('a' + letterorder[i]);
    //     std::cout << "\n";
    // );

    let mut words = [0usize; 5];

    let solutions = findwords(
        &letterorder,
        &letterindexes,
        &bits_to_index,
        &index_to_word,
        0,
        0,
        &mut words,
        0,
        false,
    );

    println!("Found {} solutions", solutions);
}
