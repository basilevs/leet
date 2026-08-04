// Minimal test harness for LeetCode solution files.
//
// Each cpp/src/<name>_<id>.cpp is compiled into its own self-contained test
// binary; this header supplies the registry, the assertions and `main`.
//
// Usage:
//     #include "test.hpp"
//
//     TEST(official1) {
//         check_eq(2, findGcd({2, 5, 6, 9, 10}));
//     }
//
// Run one binary directly, or `make test` from cpp/. An optional argument
// filters tests by substring: `./build/find_gcd_1979 official2`.
#pragma once

#include <cstddef>
#include <exception>
#include <iostream>
#include <iterator>
#include <ostream>
#include <source_location>
#include <sstream>
#include <string>
#include <string_view>
#include <type_traits>
#include <vector>

namespace leet_test {

// --- value printing -------------------------------------------------------

template <typename T>
concept Streamable = requires(std::ostream& os, const T& v) { os << v; };

template <typename T>
concept Iterable = requires(const T& v) {
    std::begin(v);
    std::end(v);
};

template <typename T>
concept PairLike = requires(const T& v) {
    v.first;
    v.second;
};

template <typename T>
void print(std::ostream& os, const T& value) {
    if constexpr (std::is_same_v<T, bool>) {
        os << (value ? "true" : "false");
    } else if constexpr (std::is_same_v<T, char>) {
        os << '\'' << value << '\'';
    } else if constexpr (std::is_convertible_v<T, std::string_view>) {
        os << '"' << std::string_view(value) << '"';
    } else if constexpr (PairLike<T>) {
        os << '(';
        print(os, value.first);
        os << ", ";
        print(os, value.second);
        os << ')';
    } else if constexpr (Iterable<T>) {
        os << '[';
        bool first = true;
        for (const auto& item : value) {
            if (!first) {
                os << ", ";
            }
            first = false;
            print(os, item);
        }
        os << ']';
    } else if constexpr (Streamable<T>) {
        os << value;
    } else {
        os << "<unprintable>";
    }
}

template <typename T>
std::string describe(const T& value) {
    std::ostringstream os;
    print(os, value);
    return os.str();
}

// --- failures -------------------------------------------------------------

struct Failure {
    std::string message;
};

inline std::string at(const std::source_location& loc) {
    std::ostringstream os;
    os << loc.file_name() << ':' << loc.line();
    return os.str();
}

/// Marks an unimplemented training scaffold. Fails the test that reaches it,
/// the C++ counterpart of Rust's `todo!()`.
[[noreturn]] inline void todo(
    std::string_view what = "training scaffold: implement solution",
    const std::source_location& loc = std::source_location::current()) {
    throw Failure{at(loc) + ": not yet implemented (" + std::string(what) + ')'};
}

/// Silences unused-parameter warnings for a scaffold's arguments.
template <typename... Ts>
void unused(const Ts&... /*args*/) {}

// `Expected` is deduced from `actual` only, so braced initialisers work on the
// left: check_eq({9, 5, 3}, pivotArray({...}, 10)).
template <typename Actual>
void check_eq(const std::type_identity_t<Actual>& expected, const Actual& actual,
              const std::source_location& loc = std::source_location::current()) {
    if (!(expected == actual)) {
        throw Failure{at(loc) + ": expected " + describe(expected) + ", got " + describe(actual)};
    }
}

inline void check(bool condition,
                  std::string_view message = "condition is false",
                  const std::source_location& loc = std::source_location::current()) {
    if (!condition) {
        throw Failure{at(loc) + ": " + std::string(message)};
    }
}

// --- registry -------------------------------------------------------------

struct Case {
    const char* name;
    void (*run)();
};

inline std::vector<Case>& registry() {
    static std::vector<Case> cases;
    return cases;
}

struct Registrar {
    Registrar(const char* name, void (*run)()) { registry().push_back(Case{name, run}); }
};

inline int run_all(std::string_view filter) {
    std::vector<std::string> failures;
    std::size_t selected = 0;
    std::size_t passed = 0;

    for (const Case& c : registry()) {
        if (!filter.empty() && std::string_view(c.name).find(filter) == std::string_view::npos) {
            continue;
        }
        ++selected;
        std::cout << "test " << c.name << " ... " << std::flush;
        try {
            c.run();
            ++passed;
            std::cout << "ok\n";
        } catch (const Failure& f) {
            failures.push_back(std::string(c.name) + '\n' + "    " + f.message);
            std::cout << "FAILED\n";
        } catch (const std::exception& e) {
            failures.push_back(std::string(c.name) + '\n' + "    threw std::exception: " + e.what());
            std::cout << "FAILED\n";
        } catch (...) {
            failures.push_back(std::string(c.name) + '\n' + "    threw an unknown exception");
            std::cout << "FAILED\n";
        }
    }

    if (!failures.empty()) {
        std::cout << "\nfailures:\n\n";
        for (const std::string& f : failures) {
            std::cout << "---- " << f << "\n\n";
        }
    }

    std::cout << "test result: " << (failures.empty() ? "ok" : "FAILED") << ". " << passed
              << " passed; " << failures.size() << " failed";
    if (selected != registry().size()) {
        std::cout << "; " << registry().size() - selected << " filtered out";
    }
    std::cout << '\n';

    return failures.empty() ? 0 : 1;
}

}  // namespace leet_test

using leet_test::check;
using leet_test::check_eq;
using leet_test::todo;
using leet_test::unused;

#define TEST(name)                                                       \
    static void name();                                                  \
    static const ::leet_test::Registrar leet_test_registrar_##name(#name, &name); \
    static void name()

int main(int argc, char** argv) {
    const std::string_view filter = argc > 1 ? argv[1] : "";
    return ::leet_test::run_all(filter);
}
