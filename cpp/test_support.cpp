#include "test_support.h"
#include <random>

extern bool g_showOutput;
void SetNodeShowDeleteOutput(bool doOutput) noexcept {
    g_ShowOutput = doOutput;
}

unordered_set<int> GetLargeData() noexcept {

    unordered_set<int> uset;
    uset.reserve(1000000);

    // Random device to seed the generator 
    std::random_device rd;
    // Mersenne Twister engine seeded with random device
    std::mt19937 gen(rd());
    // Distribution range 1 to 2,000,000 
    std::uniform_int_distribution<> dis(1, 2000000);

    for (int i = 0; i < 1000000; ++i) {
        int randomNumber = dis(gen);
        uset.insert(randomNumber);
    }

    return uset;
}