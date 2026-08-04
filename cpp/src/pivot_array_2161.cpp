// https://leetcode.com/problems/partition-array-according-to-given-pivot

#include "test.hpp"

#include <algorithm>
#include <cstddef>
#include <iterator>
#include <vector>

    std::vector<int> pivotArray(std::vector<int> nums, int pivot) {
        const auto equal = static_cast<std::size_t>(std::ranges::count(nums, pivot));

        std::vector<int> result;
        result.reserve(nums.size());
        std::ranges::copy_if(nums, std::back_inserter(result), [&](int n) { return n < pivot; });
        result.insert(result.end(), equal, pivot);
        std::ranges::copy_if(nums, std::back_inserter(result), [&](int n) { return n > pivot; });
        return result;
    }

TEST(official1) {
    check_eq({9, 5, 3, 10, 10, 12, 14}, pivotArray({9, 12, 5, 10, 14, 3, 10}, 10));
}

TEST(official2) {
    check_eq({-3, 2, 4, 3}, pivotArray({-3, 4, 3, 2}, 2));
}
