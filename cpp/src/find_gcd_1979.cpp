// https://leetcode.com/problems/find-greatest-common-divisor-of-array

#include "test.hpp"

#include <algorithm>
#include <numeric>
#include <vector>

    int findGcd(std::vector<int> nums) {
        const auto [min, max] = std::ranges::minmax(nums);
        return std::gcd(min, max);
    }

TEST(official1) {
    check_eq(2, findGcd({2, 5, 6, 9, 10}));
}

TEST(official2) {
    check_eq(1, findGcd({7, 5, 6, 8, 3}));
}

TEST(official3) {
    check_eq(3, findGcd({3, 3}));
}
