// https://leetcode.com/problems/distribute-elements-into-two-arrays-i

#include "test.hpp"

#include <vector>

    std::vector<int> resultArray(const std::vector<int>& nums) {
        auto result = std::vector<int>(nums.size());
        auto i = nums.begin();
        size_t cursor1{0};
        size_t cursor2{nums.size() - 1};
        result[cursor1] = *i++;
        result[cursor2] = *i++;
        for (; i != nums.end(); ++i) {
            if (result[cursor1] > result[cursor2]) {
                result[++cursor1] = *i;
            } else {
                result[--cursor2] = *i;
            }
        }
        std::reverse(result.begin() + cursor2, result.end());
        return result;
    }

TEST(official1) {
    check_eq({2, 3, 1}, resultArray({2, 1, 3}));
}

TEST(official2) {
    check_eq({5, 3, 4, 8}, resultArray({5, 4, 3, 8}));
}
