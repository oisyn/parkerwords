#include <bit>
#include <ctime>
#include <vector>
#include <string>
#include <unordered_map>
#include <cstdio>
#include <iostream>
#include <iomanip>
#include <fstream>
#include <functional>
#include <algorithm>
#include <array>
#include <chrono>
#include <intrin.h>
#include <emmintrin.h>

// uncomment this line to write info to stdout, which takes away precious CPU time
#define NO_OUTPUT


#ifdef NO_OUTPUT
#define OUTPUT(x) do;while(false)
#else
#define OUTPUT(x) do{x;}while(false)
#endif


using uint = unsigned int;

std::vector<uint> wordbits;
std::vector<std::string> allwords;
std::unordered_map<uint, size_t> bitstoindex;
std::vector<uint> letterindex[26];
uint letterorder[26];

std::string_view getword(const char*& _str, const char* end)
{
    const char* str = _str;
    while(*str == '\n' || *str == '\r')
	{
		if (++str == end)
            return (_str = str), std::string_view{};
	}

    const char* start = str;
    while(str != end && *str != '\n' && *str != '\r')
        ++str;

    _str = str;
    return std::string_view{ start, str };
}

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
    std::vector<char> buf;
    std::ifstream in(file);
    in.seekg(0, std::ios::end);
    buf.resize(in.tellg());
    in.seekg(0, std::ios::beg);
    in.read(&buf[0], buf.size());

    const char* str = &buf[0];
	const char* strEnd = str + buf.size();

    // read words
    std::string_view word;
    while(!(word = getword(str, strEnd)).empty())
    {
        if (word.size() != 5)
            continue;
        uint bits = getbits(word);
        if (std::popcount(bits) != 5)
            continue;

        if (!bitstoindex.contains(bits))
        {
            bitstoindex[bits] = wordbits.size();
            wordbits.push_back(bits);
            allwords.emplace_back(word);

            // count letter frequency
            for(char c: word)
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
        uint min = reverseletterorder[letter];
		m &= m - 1; // drop lowest set bit
        while(m)
        {
            letter = std::countr_zero(m);
            min = std::min(min, reverseletterorder[letter]);
			m &= m - 1;
		}

        letterindex[min].push_back(w);
    }

    // lets make sure the ends of our indices are padded with ffff'ffff so we can use unaligned sse256 loads
	for (int i = 0; i < 26; i++)
	{
        for (int j = 0; j < 7; j++)
    		letterindex[i].push_back(~0);
        letterindex[i].resize(letterindex[i].size() - 7);
	}
}

using WordArray = std::array<uint, 5>;
using OutputFn = std::function<void(const WordArray&)>;

long long start;
long long timeUS() { return std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::high_resolution_clock::now().time_since_epoch()).count(); }

int findwords(std::vector<WordArray>& solutions, uint totalbits, int numwords, WordArray& words, uint maxLetter, bool skipped)
{
	if (numwords == 5)
	{
		solutions.push_back(words);
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
        auto& index = letterindex[i];
        auto pWords = &index[0];
        auto pEnd = pWords + index.size();
        __m256i current = _mm256_set1_epi32(totalbits);
        for (; pWords < pEnd; pWords += 8)
		{
            __m256i wordsbits = _mm256_loadu_epi32(pWords);
            __m256i mask = _mm256_cmpeq_epi32(_mm256_and_si256(wordsbits, current), _mm256_setzero_si256());
            uint mvmask = _mm256_movemask_epi8(mask);
            mvmask &= 0x11111111;
			while(mvmask)
			{
                uint idx = std::countr_zero(mvmask) >> 2;

				uint w = pWords[idx];
				newwords[numwords] = w;
				numsolutions += findwords(solutions, totalbits | w, numwords + 1, newwords, i + 1, skipped);

                mvmask &= mvmask - 1;
			}

			OUTPUT(if (numwords == 0) std::cout << "\33[2K\rFound " << numsolutions << " solutions. Running time: " << (timeUS() - start) << "us");
		}

        if (skipped)
            break;
        skipped = true;
	}

	return numsolutions;
}

int findwords(std::vector<WordArray>& solutions)
{
    WordArray words = { };
    return findwords(solutions, 0x8000'0000, 0, words, 0, false);
}

int main()
{
    start = timeUS();
    readwords("words_alpha.txt");
    std::vector<WordArray> solutions;
    solutions.reserve(10000);

    OUTPUT(
        std::cout << wordbits.size() << " unique words\n";
	    std::cout << "letter order: ";
	    for (int i = 0; i < 26; i++)
		    std::cout << char('a' + letterorder[i]);
	    std::cout << "\n";
    );

    auto startAlgo = timeUS();
    int num = findwords(solutions);

    auto startOutput = timeUS();
	std::ofstream out("solutions.txt");
    for (auto& words : solutions)
    {
        for (auto w : words)
            out << allwords[bitstoindex[w]] << "\t";
        out << "\n";
    };

	OUTPUT(std::cout << "\n");

	long long end = timeUS();
	std::cout << num << " solutions written to solutions.txt.\n";
    std::cout << "Total time: " << end - start << "us (" << (end - start) / 1.e6f << "s)\n";
    std::cout << "Read:    " << std::setw(8) << startAlgo - start << "us\n";
	std::cout << "Process: " << std::setw(8) << startOutput - startAlgo << "us\n";
	std::cout << "Write:   " << std::setw(8) << end - startOutput << "us\n";
}
