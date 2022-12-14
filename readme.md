### A solution to the problem of finding five English words with 25 distinct characters, as posed in this video by Matt Parker: https://www.youtube.com/watch?v=_-AfhLQfb6w

---
**While you're here, Benjamin Paassen was nice enough to run all solutions on his PC and made a sheet with the timing results. Check out the twitter thread [here](https://twitter.com/bpaassen1/status/1556900513505021953).**

---

To compile, either open parkerwords.sln in VS 2019 or later, or compile parkerwords.cpp using your favorite C++20 compiler.

It takes 0.055 seconds to run on my AMD Ryzen 5800X, and finds all the 538 solutions mentioned in the video. Result is written to solutions.txt.
```
Total time: 55332us (0.055332s)
Read:       11661us
Process:    43255us
Write:        416us
```

For an implementation using AVX2, see the [SSE branch](https://github.com/oisyn/parkerwords/tree/sse) (currently at 23ms)

Since writing to stdout now no longer takes a negligible amount of time relative to the rest of the algorithm, I've made most output conditional. Uncomment the NO_OUTPUT #define in the top of the file to reenable some verbose information and progress indication.

## Description
The algorithm handles words as bitsets stored in a 32-bit integer, where each bit position represents the inclusion of that letter in the word, with 'a' being bit 0, 'b' bit 1, and so forth, up to 26 bits in total. Using a bitwise AND, we can quickly check if two words have overlapping letters, which would then give a non-zero result.

Furthermore, it uses an index to quickly look up a list of words containing a certain letter. By leveraging the fact that the algorithm looks for the letters in a certain order, we only need to store each word in the index once; with it's lowest ordered letter as the index. To determine the order of the letters, the frequency of each letter is recorded and the order of letters is from least to most frequently used letter (using this dataset, it produces the order: "qxjzvfwbkgpmhdcytlnuroisea").

As there are 26 letters in the English alphabet, and we're looking for a list of 5 words, only one letter remains unused. The algorithm is therefore only allowed to skip a single letter.

It basically works as follows:

1. Start with an empty set of letters.
2. Look up which words contains the lowest unused letter
3. For each word in step 2, check whether all its letters are still unused by intersecting it with the current set (using bitwise AND). If this is the case, add it to the set and recursively apply step 2, 3 and 4, until you find a set of 5 words; this is a valid solution.
4. If you have not skipped a letter before, skip the lowest unused letter and redo step 2 again but with the next-lowest unused letter.
