// https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor

#include "test.hpp"

#include <utility>
#include <vector>

    int longestSubsequence(const std::vector<int>& nums) {
        size_t n{nums.size()};
        if (n == 0 || !std::in_range<int>(n)) {
            return 0;
        }
        int total_xor{0};
        bool non_zero{false};
        for (int i : nums) {
            total_xor ^= i;
            non_zero |= i != 0;
        }
        if (total_xor != 0) {
            return static_cast<int>(n);
        }
        if (non_zero) {
            return static_cast<int>(n - 1); // does not underflow because n > 0
        }
        return 0;
    }

TEST(official1) {
    check_eq(2, longestSubsequence({1, 2, 3}));
}

TEST(official2) {
    check_eq(3, longestSubsequence({2, 3, 4}));
}
