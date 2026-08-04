# Build and run the C++ LeetCode solutions.
#
#   make            build every solution into its own test binary
#   make test       build and run all tests
#   make test T=gcd run only solutions whose file name contains "gcd"
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

SRC_DIR := src
BUILD_DIR := build

SOURCES := $(sort $(wildcard $(SRC_DIR)/*.cpp))
NAMES := $(patsubst $(SRC_DIR)/%.cpp,%,$(SOURCES))
BINARIES := $(addprefix $(BUILD_DIR)/,$(NAMES))
DEPS := $(addsuffix .d,$(BINARIES))

# T filters which solutions `make test` builds and runs, by substring.
T ?=
ifeq ($(strip $(T)),)
TEST_TARGETS := $(BINARIES)
else
TEST_TARGETS := $(foreach b,$(BINARIES),$(if $(findstring $(T),$(b)),$(b)))
endif

.PHONY: all test list clean

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

list:
	@for name in $(NAMES); do echo "$$name"; done

clean:
	rm -rf $(BUILD_DIR)

-include $(DEPS)
