# Build and run the C++ LeetCode solutions.
#
#   make            build every solution into its own test binary
#   make test       build and run all tests
#   make test T=gcd run only solutions whose file name contains "gcd"
#   make tidy       lint solutions with clang-tidy (style/correctness, no format)
#   make tidy T=gcd lint only solutions whose file name contains "gcd"
#   make list       show the discovered solutions
#   make clean      remove build/
#
# Each src/*.cpp is a standalone translation unit: it carries the solution, its
# tests, and (via include/test.hpp) its own main(). There is no shared library
# and no link step beyond the single object, so files never interfere.

CXX ?= c++
CXXSTD ?= c++23
OPT ?= -O1
CXXFLAGS ?= -std=$(CXXSTD) -g $(OPT) -Wall -Wextra -Wpedantic -Wshadow -Wconversion
CPPFLAGS += -I include -MMD -MP

# Prefer a clang-tidy on PATH; fall back to the Homebrew LLVM install (the Xcode
# command-line tools do not ship clang-tidy). Override with CLANG_TIDY=... .
CLANG_TIDY ?= $(shell command -v clang-tidy 2>/dev/null || echo /opt/homebrew/opt/llvm/bin/clang-tidy)

SRC_DIR := src
BUILD_DIR := build

SOURCES := $(sort $(wildcard $(SRC_DIR)/*.cpp))
NAMES := $(patsubst $(SRC_DIR)/%.cpp,%,$(SOURCES))
BINARIES := $(addprefix $(BUILD_DIR)/,$(NAMES))
DEPS := $(addsuffix .d,$(BINARIES))

# T filters which solutions `make test` / `make tidy` act on, by substring.
T ?=
ifeq ($(strip $(T)),)
TEST_TARGETS := $(BINARIES)
TIDY_SOURCES := $(SOURCES)
else
TEST_TARGETS := $(foreach b,$(BINARIES),$(if $(findstring $(T),$(b)),$(b)))
TIDY_SOURCES := $(foreach s,$(SOURCES),$(if $(findstring $(T),$(s)),$(s)))
endif

.PHONY: all test tidy list clean

all: $(BINARIES)

$(BUILD_DIR)/%: $(SRC_DIR)/%.cpp | $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) $(LDFLAGS) -o $@ $<

$(BUILD_DIR):
	mkdir -p $@

test: $(TEST_TARGETS)
	@if [ -z "$(strip $(TEST_TARGETS))" ]; then \
	    echo "no solutions match T=$(T)"; exit 1; \
	fi
	@status=0; \
	for binary in $(TEST_TARGETS); do \
	    echo "==== $$binary"; \
	    "$$binary" || status=1; \
	    echo; \
	done; \
	exit $$status

# Lint only: clang-tidy reads .clang-tidy and compile_flags.txt from this dir.
# The config keeps warnings non-fatal for editor use; the gate makes any finding
# fail the build via --warnings-as-errors.
tidy:
	@if [ -z "$(strip $(TIDY_SOURCES))" ]; then \
	    echo "no solutions match T=$(T)"; exit 1; \
	fi
	@command -v "$(CLANG_TIDY)" >/dev/null 2>&1 || { \
	    echo "clang-tidy not found (looked for: $(CLANG_TIDY))"; exit 1; \
	}
	@status=0; \
	for src in $(TIDY_SOURCES); do \
	    echo "==== $$src"; \
	    "$(CLANG_TIDY)" --quiet --warnings-as-errors='*' "$$src" || status=1; \
	done; \
	exit $$status

list:
	@for name in $(NAMES); do echo "$$name"; done

clean:
	rm -rf $(BUILD_DIR)

-include $(DEPS)
