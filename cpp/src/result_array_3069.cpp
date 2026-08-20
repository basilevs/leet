// https://leetcode.com/problems/distribute-elements-into-two-arrays-i

#include "test.hpp"

#include <vector>

    std::vector<int> resultArray(const std::vector<int>& nums) {
        auto result = std::vector<int>(nums.size());
        auto front = result.begin();
        auto back = result.end();
        auto i = nums.begin();
        *front = *i++;
        *--back = *i++;
        for (; i != nums.end(); ++i) {
            if (*front > *back) {
                *++front = *i;
            } else {
                *--back = *i;
            }
        }
        std::reverse(back, result.end());
        return result;
    }

TEST(official1) {
    check_eq({2, 3, 1}, resultArray({2, 1, 3}));
}

TEST(official2) {
    check_eq({5, 3, 4, 8}, resultArray({5, 4, 3, 8}));
}
