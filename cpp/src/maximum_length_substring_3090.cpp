// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences

#include "test.hpp"

#include <string>

    int maximumLengthSubstring(const std::string& s) {
        // Index of the next element leaving the window.
        std::size_t back = 0;
        int length = 0;
        int longest_length = 0;
        int freq[26] = {};
        for (char ch : s) {
            int num = ch - 'a';
            int& bucket = freq[num];
            ++bucket;
            ++length;
            if (bucket > 2) {
                // Only `num` can exceed `k`, and by exactly one, so shrinking
                // until its earliest occurrence leaves the window is enough.
                while (true) {
                    int evicted = s[back] - 'a';
                    ++back;
                    --freq[evicted];
                    --length;
                    if (evicted == num) {
                        break;
                    }
                }
            }
            longest_length = std::max(longest_length, length);
        }
        return longest_length;
    }

TEST(official1) {
    check_eq(4, maximumLengthSubstring("bcbbbcba"));
}

TEST(official2) {
    check_eq(2, maximumLengthSubstring("aaaa"));
}
