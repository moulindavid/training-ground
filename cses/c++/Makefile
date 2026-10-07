CXX := g++
CXXFLAGS := -std=c++17 -O2 -Wall -Wextra -pedantic
BUILD_DIR := build
PROBLEM ?= weird_algorithm
SRC := $(PROBLEM).cpp
BIN := $(BUILD_DIR)/$(PROBLEM)

.PHONY: build run test clean

build: $(BIN)

$(BIN): $(SRC)
	mkdir -p $(BUILD_DIR)
	$(CXX) $(CXXFLAGS) $< -o $@

# Usage:
#   make run PROBLEM=weird_algorithm INPUT=3
#   make run PROBLEM=weird_algorithm FILE=tests/weird_algorithm/sample.in
run: build
ifdef FILE
	./$(BIN) < $(FILE)
else
	printf '%s\n' '$(INPUT)' | ./$(BIN)
endif

# Expects tests/<problem>/sample.in and tests/<problem>/sample.out
test: build
	./$(BIN) < tests/$(PROBLEM)/sample.in > $(BUILD_DIR)/$(PROBLEM).out
	diff -u tests/$(PROBLEM)/sample.out $(BUILD_DIR)/$(PROBLEM).out

clean:
	rm -rf $(BUILD_DIR)
