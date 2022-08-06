#include <bit>
#include <ctime>
#include <vector>
#include <string>
#include <unordered_map>
#include <cstdio>
#include <iostream>
#include <fstream>
#include <functional>
#include <algorithm>
#include <array>
#include <chrono>

// uncomment this line to write info to stdout, which takes away precious CPU time
#define NO_OUTPUT


#ifdef NO_OUTPUT
#define OUTPUT(x) do;while(false)
#else
#define OUTPUT(x) do{x;}while(false)
#endif


using uint = unsigned int;

std::vector<uint> wordbits;
std::vector<std::vector<std::string>> wordanagrams;
std::unordered_map<uint, size_t> bitstoindex;
std::vector<uint> letterindex[26];
uint letterorder[26];

uint getbits(std::string_view word)
{
    uint r = 0;
    for (char c : word)
        r |= 1 << (c - 'a');
    return r;
}

void readwords(const char* file)
{
	struct { int f, l; } freq[26] = { };
	for (int i = 0; i < 26; i++)
		freq[i].l = i;

    // open file
    std::ifstream in(file);
    std::string line;

    // read words
    while(std::getline(in, line))
    {
        if (line.size() != 5)
            continue;
        uint bits = getbits(line);
        if (std::popcount(bits) != 5)
            continue;

        if (!bitstoindex.contains(bits))
        {
            bitstoindex[bits] = wordbits.size();
            wordbits.push_back(bits);
            wordanagrams.push_back({ line });

            // count letter frequency
            for(char c: line)
                freq[c - 'a'].f++;
        }
    }

    // rearrange letter order based on lettter frequency (least used letter gets lowest index)
    std::sort(std::begin(freq), std::end(freq), [](auto a, auto b) { return a.f < b.f; });
	uint reverseletterorder[26];
    for (int i = 0; i < 26; i++)
	{
		letterorder[i] = freq[i].l;
        reverseletterorder[freq[i].l] = i;
    }

    // build index based on least used letter
    for (uint w : wordbits)
    {
        uint m = w;
		uint letter = std::countr_zero(m);
        m -= 1 << letter;
        uint min = reverseletterorder[letter];
        while(m)
        {
            letter = std::countr_zero(m);
			m -= 1 << letter;
            min = std::min(min, reverseletterorder[letter]);
		}

        letterindex[min].push_back(w);
    }
}

using WordArray = std::array<size_t, 5>;
using OutputFn = std::function<void(const WordArray&)>;

long long start;
long long timeUS() { return std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::high_resolution_clock::now().time_since_epoch()).count(); }

int findwords(OutputFn& output, uint totalbits, int numwords, WordArray& words, uint maxLetter, bool skipped)
{
	if (numwords == 5)
	{
		output(words);
		return 1;
	}

	int numsolutions = 0;
	size_t max = wordbits.size();
	WordArray newwords = words;

    // walk over all letters in a certain order until we find an unused one
	for (uint i = maxLetter; i < 26; i++)
	{
        uint letter = letterorder[i];
        uint m = 1 << letter;
        if (totalbits & m)
            continue;

        // take all words from the index of this letter and add each word to the solution if all letters of the word aren't used before.
        for (uint w : letterindex[i])
		{
			if (totalbits & w)
				continue;

			size_t idx = bitstoindex[w];
			newwords[numwords] = idx;
			numsolutions += findwords(output, totalbits | w, numwords + 1, newwords, i + 1, skipped);

			OUTPUT(if (numwords == 0) std::cout << "\33[2K\rFound " << numsolutions << " solutions. Running time: " << (timeUS() - start) << "us");
		}

        if (skipped)
            break;
        skipped = true;
	}

	return numsolutions;
}

int findwords(OutputFn output)
{
    WordArray words = { };
    return findwords(output, 0, 0, words, 0, false);
}

int main()
{
    start = timeUS();
    readwords("words_alpha.txt");

    OUTPUT(
        std::cout << wordbits.size() << " unique words\n";
	    std::cout << "letter order: ";
	    for (int i = 0; i < 26; i++)
		    std::cout << char('a' + letterorder[i]);
	    std::cout << "\n";
    );

    std::ofstream out("solutions.txt");
    int num = findwords([&](const WordArray& words)
        {
            for (auto idx : words)
                out << wordanagrams[idx][0] << "\t";
            out << "\n";
        });

	OUTPUT(std::cout << "\n");

	long long time = timeUS() - start;
	std::cout << num << " solutions written to solutions.txt.\n";
    std::cout << "Total time: " << time << "us (" << time / 1.e6f << "s)\n";
}
