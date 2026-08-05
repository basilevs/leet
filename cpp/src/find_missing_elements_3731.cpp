// https://leetcode.com/problems/find-missing-elements

#include "test.hpp"

#include <algorithm>
#include <cstdint>
#include <ranges>
#include <unordered_set>
#include <vector>

std::vector<int> findMissingElements(std::vector<int> nums) {
    std::ranges::sort(nums);
    std::vector<int> result;
    // `zip` with a dropped-by-one copy is the pairwise view libc++ lacks; it also
    // sidesteps the `nums.size() - 1` underflow on an empty input.
    for (const auto [lo, hi] : std::views::zip(nums, nums | std::views::drop(1))) {
        result.append_range(std::views::iota(lo + 1, hi));
    }
    return result;
}

// Alternative: mark presence in a `std::vector<bool>`, then scan the full `[min, max]`
// range collecting values that are still `false`.
std::vector<int> findMissingElementsBool(std::vector<int> nums) {
    const auto [min_val, max_val] = std::ranges::minmax(nums);
    const auto span = static_cast<std::size_t>(max_val - min_val + 1);

    std::vector<bool> present(span, false);
    for (int v : nums) {
        present[static_cast<std::size_t>(v - min_val)] = true;
    }

    std::vector<int> result;
    for (int v = min_val; v <= max_val; ++v) {
        if (!present[static_cast<std::size_t>(v - min_val)]) {
            result.push_back(v);
        }
    }
    return result;
}

// Alternative: collect into an `std::unordered_set`, then scan the full open range
// `(min, max)` collecting values absent from the set.
std::vector<int> findMissingElementsHashSet(const std::vector<int>& nums) {
    std::unordered_set<int> st(nums.begin(), nums.end());
    const auto [min_val, max_val] = std::ranges::minmax(nums);

    std::vector<int> result;
    for (int x = min_val + 1; x < max_val; ++x) {
        if (st.find(x) == st.end()) {
            result.push_back(x);
        }
    }
    return result;
}

// Alternative: mark presence in a bitset, then scan the full `[min, max]`
// range collecting values whose bit is unset.
std::vector<int> findMissingElementsBitset(const std::vector<int>& nums) {
    const auto [min_val, max_val] = std::ranges::minmax(nums);
    const auto span = static_cast<std::size_t>(max_val - min_val + 1);
    const auto num_words = (span + 63) / 64;
    std::vector<std::uint64_t> bits(num_words, 0);

    for (int v : nums) {
        const auto i = static_cast<std::size_t>(v - min_val);
        bits[i / 64] |= 1ULL << (i % 64);
    }

    std::vector<int> result;
    for (int v = min_val; v <= max_val; ++v) {
        const auto i = static_cast<std::size_t>(v - min_val);
        if ((bits[i / 64] & (1ULL << (i % 64))) == 0) {
            result.push_back(v);
        }
    }
    return result;
}

TEST(official1) {
    check_eq({3}, findMissingElements({1, 4, 2, 5}));
}

TEST(official2) {
    check_eq(std::vector<int>{}, findMissingElements({7, 8, 6, 9}));
}

TEST(official3) {
    check_eq({2, 3, 4}, findMissingElements({5, 1}));
}

TEST(bool_matches_official1) {
    check_eq({3}, findMissingElementsBool({1, 4, 2, 5}));
}

TEST(bool_matches_official2) {
    check_eq(std::vector<int>{}, findMissingElementsBool({7, 8, 6, 9}));
}

TEST(bool_matches_official3) {
    check_eq({2, 3, 4}, findMissingElementsBool({5, 1}));
}

TEST(hashset_matches_official1) {
    check_eq({3}, findMissingElementsHashSet({1, 4, 2, 5}));
}

TEST(hashset_matches_official2) {
    check_eq(std::vector<int>{}, findMissingElementsHashSet({7, 8, 6, 9}));
}

TEST(hashset_matches_official3) {
    check_eq({2, 3, 4}, findMissingElementsHashSet({5, 1}));
}

TEST(bitset_matches_official1) {
    check_eq({3}, findMissingElementsBitset({1, 4, 2, 5}));
}

TEST(bitset_matches_official2) {
    check_eq(std::vector<int>{}, findMissingElementsBitset({7, 8, 6, 9}));
}

TEST(bitset_matches_official3) {
    check_eq({2, 3, 4}, findMissingElementsBitset({5, 1}));
}
