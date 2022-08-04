#include <bit>
#include <ctime>
#include <vector>
#include <string>
#include <unordered_map>
#include <cstdio>
#include <iostream>
#include <fstream>
#include <functional>
#include <array>

using uint = unsigned int;

std::vector<uint> wordbits;
std::vector<std::vector<std::string>> wordanagrams;
std::unordered_map<uint, size_t> bitstoindex;
std::vector<uint> letterindex[26];

uint getbits(std::string_view word)
{
    uint r = 0;
    for (char c : word)
        r |= 1 << (c - 'a');
    return r;
}

void readwords(const char* file)
{
    std::ifstream in(file);
    std::string line;
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

            uint lowestLetter = std::countr_zero(bits);
            letterindex[lowestLetter].push_back(bits);
        }
    }
}

using WordArray = std::array<size_t, 5>;
using OutputFn = std::function<void(const WordArray&)>;

time_t start;

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

	for (uint i = maxLetter; i < 26; i++)
	{
        uint m = 1 << i;
        if (totalbits & m)
            continue;

        for (uint w : letterindex[i])
		{
			if (totalbits & w)
				continue;

			size_t idx = bitstoindex[w];
			newwords[numwords] = idx;
			numsolutions += findwords(output, totalbits | w, numwords + 1, newwords, i + 1, skipped);

			if (numwords == 0)
				std::cout << "\33[2K\rFound " << numsolutions << " solutions. Running time: " << time(0) - start << "s";
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
    start = time(0);
    readwords("words_alpha.txt");
    std::cout << wordbits.size() << "unique words\n";
    std::ofstream out("solutions.txt");
    int num = findwords([&](const WordArray& words)
        {
            for (auto idx : words)
                out << wordanagrams[idx][0] << "\t";
            out << "\n";
        });
	std::cout << "\nsolutions.txt written.\n";
}
