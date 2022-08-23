use std::{collections::HashMap, fs::File, time::SystemTime};

use memmap::Mmap;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

fn findwords_parallel(
    lettermask: &[u32; 26],
    letter_to_words_bits: &[Vec<u32>; 26],
    bits_to_index: &HashMap<u32, usize>,
    index_to_word: &Vec<&[u8]>,
) -> usize {
    struct StartInfo {
        totalbits: u32,
        numwords: usize,
        words: [usize; 5],
        max_letter: usize,
        skipped: i32,
    }

    let mut words: [usize; 5] = [0; 5];

    // let (sender1, receiver1) = crossbeam::channel::unbounded::<u32>();
    // let (sender2, receiver2) = crossbeam::channel::unbounded::<u32>();
    // let (sender_output, receiver_output) = crossbeam::channel::unbounded::<u32>();

    // let mut numwords: usize = 0;

    // rayon::join(
    //     || {
    findwords(
        lettermask,
        letter_to_words_bits,
        bits_to_index,
        index_to_word,
        0,
        0,
        &mut words,
        0,
        -1,
    )
    //     },
    //     || {},
    // );

    // numwords
}

fn findwords(
    lettermask: &[u32; 26],
    letter_to_words_bits: &[Vec<u32>; 26],
    bits_to_index: &HashMap<u32, usize>,
    index_to_word: &Vec<&[u8]>,

    totalbits: u32,
    numwords: usize,
    words: &mut [usize; 5],
    max_letter: usize,
    mut skipped: i32,
) -> usize {
    let mut numsolutions: usize = 0;

    // If we don't have 5 letters left there is not point going on
    let upper: usize = 26 - 5;

    // walk over all letters in a certain order until we find an unused one
    for i in max_letter..upper {
        let m: u32 = lettermask[i];
        if totalbits & m != 0 {
            continue;
        }

        // take all words from the index of this letter and add each word to the solution if all letters of the word aren't used before.

        // Use parallelism at the top level only
        if numwords == 0 || numwords == 1 {
            numsolutions += letter_to_words_bits[i]
                .par_iter()
                .map(|w| {
                    if totalbits & w != 0 {
                        0usize
                    } else {
                        let idx: usize = bits_to_index[&w];
                        let mut newwords: [usize; 5] = words.clone();
                        newwords[numwords] = idx;
                        findwords(
                            lettermask,
                            letter_to_words_bits,
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
            if numwords == 4 && skipped >= 0 {
                let candidate = !(totalbits | lettermask[skipped as usize]) & 0x3FFFFFF;
                if let Some(last_index) = bits_to_index.get(&candidate) {
                    words[numwords] = *last_index;

                    output(index_to_word, words);
                    numsolutions += 1
                }
            } else {
                for w in letter_to_words_bits[i].iter() {
                    if totalbits & w != 0 {
                        continue;
                    }

                    let idx: usize = bits_to_index[&w];
                    words[numwords] = idx;

                    if numwords == 4 {
                        output(index_to_word, words);
                        numsolutions += 1
                    } else {
                        numsolutions += findwords(
                            lettermask,
                            letter_to_words_bits,
                            bits_to_index,
                            index_to_word,
                            totalbits | w,
                            numwords + 1,
                            words,
                            i + 1,
                            skipped,
                        )
                    }
                }
            }
        }

        if skipped >= 0 {
            break;
        }
        skipped = i as i32;
    }

    numsolutions
}

fn output(index_to_word: &Vec<&[u8]>, words: &[usize; 5]) -> () {
    // return;
    let str = format!(
        "{} {} {} {} {}",
        unsafe { std::str::from_utf8_unchecked(index_to_word[words[0]]) },
        unsafe { std::str::from_utf8_unchecked(index_to_word[words[1]]) },
        unsafe { std::str::from_utf8_unchecked(index_to_word[words[2]]) },
        unsafe { std::str::from_utf8_unchecked(index_to_word[words[3]]) },
        unsafe { std::str::from_utf8_unchecked(index_to_word[words[4]]) }
    );
    println!("{}", str);
}

fn main() {
    let mut bits_to_index: HashMap<u32, usize> = HashMap::new();
    let mut index_to_bits: Vec<u32> = Vec::new();
    let mut index_to_word: Vec<&[u8]> = Vec::new();
    let mut letter_to_words_bits: [Vec<u32>; 26] = Default::default();
    let mut lettermask: [u32; 26] = [0; 26];

    // TODO: Add error handling
    let begin: SystemTime = SystemTime::now();
    let file: File = File::open("words_alpha.txt").unwrap();
    let file: Mmap = unsafe { Mmap::map(&file).unwrap() };
    parkerrust::readwords(
        &file,
        &mut bits_to_index,
        &mut index_to_bits,
        &mut index_to_word,
        &mut letter_to_words_bits,
        &mut lettermask,
    )
    .unwrap();
    let read_time: u128 = begin.elapsed().unwrap().as_micros();

    let mid: SystemTime = SystemTime::now();

    let solutions: usize = findwords_parallel(
        &lettermask,
        &letter_to_words_bits,
        &bits_to_index,
        &index_to_word,
    );

    let process_time: u128 = mid.elapsed().unwrap().as_micros();

    println!("{:5}us Reading time", read_time);
    println!("{:5}us Processing time", process_time);
    println!("{:5}us Total time", begin.elapsed().unwrap().as_micros());
    println!("Found {} solutions", solutions);
}
