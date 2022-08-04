#include <ctime>
#include <vector>
#include <string>
#include <unordered_map>
#include <format>
#include <iostream>
#include <fstream>
#include <functional>

using uint = unsigned int;

std::vector<uint> wordbits;
std::vector<std::vector<std::string>> wordanagrams;
std::unordered_map<uint, size_t> bitstoindex;

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
        }
    }
}

using WordArray = std::array<size_t, 5>;
using OutputFn = std::function<void(const WordArray&)>;
int findwords(OutputFn& output, uint totalbits, int numwords, WordArray& words, size_t maxId, time_t start)
{
    if (numwords == 5)
	{
        output(words);
		return 1;
	}

    int numsolutions = 0;
    size_t max = wordbits.size();
	WordArray newwords = words;

    for (size_t idx = maxId; idx < max; idx++)
    {
        uint w = wordbits[idx];
        if (totalbits & w)
            continue;

        if (numwords == 0)
            std::cout << std::format("\33[2K\r{} / {} ({}%). Found {} solutions. Running time: {}s", idx, max, 100 * idx / max, numsolutions, std::time(0) - start);

        newwords[numwords] = idx;
        numsolutions += findwords(output, totalbits | w, numwords + 1, newwords, idx + 1, start);
    }

    return numsolutions;
}

int findwords(OutputFn output)
{
    WordArray words = { };
    return findwords(output, 0, 0, words, 0, std::time(0));
}

int main()
{
    readwords("words_alpha.txt");
    std::cout << std::format("{} unique words\n", wordbits.size());
    std::ofstream out("solutions.txt");
    int num = findwords([&](const WordArray& words)
        {
            for (auto idx : words)
                out << wordanagrams[idx][0] << "\t";
            out << "\n";
        });
	std::cout << "\nsolutions.txt written.\n";
}
