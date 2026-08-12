// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency

#include "test.hpp"

#include <algorithm>
#include <unordered_map>
#include <vector>

    int maxSubarrayLength(const std::vector<int>& nums, int k) {
        std::unordered_map<int, int> freq;
        // Index of the next element leaving the window.
        std::size_t back = 0;
        int length = 0;
        int longest_length = 0;
        for (int num : nums) {
            int& bucket = freq[num];
            ++bucket;
            ++length;
            if (bucket > k) {
                // Only `num` can exceed `k`, and by exactly one, so shrinking
                // until its earliest occurrence leaves the window is enough.
                while (true) {
                    int evicted = nums[back];
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
    check_eq(6, maxSubarrayLength({1, 2, 3, 1, 2, 3, 1, 2}, 2));
}

TEST(official2) {
    check_eq(2, maxSubarrayLength({1, 2, 1, 2, 1, 2, 1, 2}, 1));
}

TEST(official3) {
    check_eq(4, maxSubarrayLength({5, 5, 5, 5, 5, 5, 5}, 4));
}

// The officials all evict at least once, so they never measure a window
// that still starts at index 0. These do.
TEST(single_element) {
    check_eq(1, maxSubarrayLength({1}, 1));
}

TEST(whole_array_is_good) {
    check_eq(3, maxSubarrayLength({1, 2, 3}, 3));
}

TEST(longest_window_is_a_prefix) {
    check_eq(4, maxSubarrayLength({2, 3, 1, 1, 1}, 2));
}

TEST(longest_window_is_a_suffix) {
    check_eq(4, maxSubarrayLength({1, 1, 1, 2, 3, 1, 1}, 2));
}
