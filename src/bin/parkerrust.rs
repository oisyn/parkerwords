use std::{collections::HashMap, time::SystemTime};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

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
    // return;
    println!(
        "{} {} {} {} {}",
        index_to_word[words[0]],
        index_to_word[words[1]],
        index_to_word[words[2]],
        index_to_word[words[3]],
        index_to_word[words[4]]
    );
}

fn main() {
    let mut bits_to_index: HashMap<usize, usize> = HashMap::new();
    let mut index_to_bits: Vec<usize> = Vec::new();
    let mut index_to_word: Vec<String> = Vec::new();
    let mut letterindexes: [Vec<usize>; 26] = Default::default();
    let mut letterorder: [usize; 26] = [0; 26];

    // TODO: Add error handling
    let begin = SystemTime::now();
    parkerrust::readwords(
        &mut bits_to_index,
        &mut index_to_bits,
        &mut index_to_word,
        &mut letterindexes,
        &mut letterorder,
        Some(5),
    )
    .unwrap();
    let read_time: u128 = begin.elapsed().unwrap().as_micros();

    let mid = SystemTime::now();

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

    let process_time: u128 = mid.elapsed().unwrap().as_micros();

    println!("Found {} solutions", solutions);
    println!("Reading time: {:8}us", read_time);
    println!("Processing time: {:5}us", process_time);
    println!("Total time: {:10}us", begin.elapsed().unwrap().as_micros());
}
