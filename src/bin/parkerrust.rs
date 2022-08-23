use std::{collections::HashMap, fs::File, time::SystemTime};

use memmap::Mmap;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn findwords(
    letterorder: &[usize; 26],
    letterindexes: &[Vec<usize>; 26],
    bits_to_index: &HashMap<usize, usize>,
    index_to_word: &Vec<&str>,

    totalbits: usize,
    numwords: usize,
    words: &mut [usize; 5],
    max_letter: usize,
    mut skipped: i32,
) -> usize {
    if numwords == 5 {
        output(index_to_word, words);
        return 1;
    }

    let mut numsolutions: usize = 0;

    // If we don't have 5 letters left there is not point going on
    let upper: usize = 26 - 5;

    // walk over all letters in a certain order until we find an unused one
    for i in max_letter..upper {
        let m: usize = 1 << letterorder[i];
        if totalbits & m != 0 {
            continue;
        }

        // take all words from the index of this letter and add each word to the solution if all letters of the word aren't used before.

        // Use parallelism at the top level only
        if numwords == 0 || numwords == 1 {
            numsolutions += letterindexes[i]
                .par_iter()
                .map(|w| {
                    if totalbits & w != 0 {
                        0usize
                    } else {
                        let idx: usize = bits_to_index[&w];
                        let mut newwords: [usize; 5] = [0; 5];
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
        } else if numwords == 4 && skipped >= 0 {
            let candidate = !(totalbits | 1 << letterorder[skipped as usize]) & 0x3FFFFFF;
            if let Some(last_index) = bits_to_index.get(&candidate) {
                words[numwords] = *last_index;
                numsolutions += findwords(
                    letterorder,
                    letterindexes,
                    bits_to_index,
                    index_to_word,
                    totalbits | candidate,
                    numwords + 1,
                    words,
                    i + 1,
                    skipped,
                );
            }
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

        if skipped >= 0 {
            break;
        }
        skipped = i as i32;
    }

    numsolutions
}

fn output(index_to_word: &Vec<&str>, words: &[usize; 5]) -> () {
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
    let mut index_to_word: Vec<&str> = Vec::new();
    let mut letterindexes: [Vec<usize>; 26] = Default::default();
    let mut letterorder: [usize; 26] = [0; 26];

    // TODO: Add error handling
    let begin: SystemTime = SystemTime::now();
    let file: File = File::open("words_alpha.txt").unwrap();
    let file: Mmap = unsafe { Mmap::map(&file).unwrap() };
    parkerrust::readwords(
        &file,
        &mut bits_to_index,
        &mut index_to_bits,
        &mut index_to_word,
        &mut letterindexes,
        &mut letterorder,
        Some(5),
    )
    .unwrap();
    let read_time: u128 = begin.elapsed().unwrap().as_micros();

    let mid: SystemTime = SystemTime::now();

    let mut words: [usize; 5] = [0; 5];

    let solutions: usize = findwords(
        &letterorder,
        &letterindexes,
        &bits_to_index,
        &index_to_word,
        0,
        0,
        &mut words,
        0,
        -1,
    );

    let process_time: u128 = mid.elapsed().unwrap().as_micros();

    println!("{:5}us Reading time", read_time);
    println!("{:5}us Processing time", process_time);
    println!("{:5}us Total time", begin.elapsed().unwrap().as_micros());
    println!("Found {} solutions", solutions);
}
