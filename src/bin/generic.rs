use std::collections::HashMap;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn findwords(
    letterorder: &[usize; 26],
    letterindexes: &[Vec<usize>; 26],
    bits_to_index: &HashMap<usize, usize>,
    index_to_word: &Vec<String>,

    totalbits: usize,
    mut words: &mut Vec<usize>,
    max_letter: usize,
) -> usize {
    if totalbits.count_ones() == 26 {
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
        if words.is_empty() {
            numsolutions += letterindexes[i]
                .par_iter()
                .map(|w| {
                    if totalbits & w != 0 {
                        0usize
                    } else {
                        let idx: usize = bits_to_index[&w];
                        let mut newwords = words.clone();
                        newwords.push(idx);
                        findwords(
                            letterorder,
                            letterindexes,
                            bits_to_index,
                            index_to_word,
                            totalbits | w,
                            &mut newwords,
                            i + 1,
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
                words.push(idx);
                numsolutions += findwords(
                    letterorder,
                    letterindexes,
                    bits_to_index,
                    index_to_word,
                    totalbits | w,
                    &mut words,
                    i + 1,
                );
                words.pop();
            }
        }
        break;
    }

    numsolutions
}

fn output(index_to_word: &Vec<String>, words: &Vec<usize>) -> () {
    let mut first: bool = true;
    for word in words.iter() {
        if first {
            print!("{}", index_to_word[*word]);
        } else {
            print!(" {}", index_to_word[*word])
        }
        first = false;
    }
    println!();
}

fn main() {
    let mut bits_to_index: HashMap<usize, usize> = HashMap::new();
    let mut index_to_bits: Vec<usize> = Vec::new();
    let mut index_to_word: Vec<String> = Vec::new();
    let mut letterindexes: [Vec<usize>; 26] = Default::default();
    let mut letterorder: [usize; 26] = [0; 26];

    // TODO: Add error handling
    parkerrust::readwords(
        &mut bits_to_index,
        &mut index_to_bits,
        &mut index_to_word,
        &mut letterindexes,
        &mut letterorder,
        None,
    )
    .unwrap();

    let mut words = Vec::new();

    let solutions = findwords(
        &letterorder,
        &letterindexes,
        &bits_to_index,
        &index_to_word,
        0,
        &mut words,
        0,
    );

    println!("Found {} solutions", solutions);
}
