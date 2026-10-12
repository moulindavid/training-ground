RUSTC := rustc
RUSTFLAGS := -O
BUILD_DIR := build
PROBLEM ?= weird_algorithm
SRC := $(PROBLEM).rs
BIN := $(BUILD_DIR)/$(PROBLEM)

.PHONY: build run test clean

build: $(BIN)

$(BIN): $(SRC)
	mkdir -p $(BUILD_DIR)
	$(RUSTC) $(RUSTFLAGS) $< -o $@

# Usage:
#   make run PROBLEM=weird_algorithm INPUT=3
#   make run PROBLEM=weird_algorithm FILE=tests/weird_algorithm/sample.in
run: build
ifdef FILE
	./$(BIN) < $(FILE)
else
	printf '%s\n' '$(INPUT)' | ./$(BIN)
endif

# Expects tests/<problem>/*.in and matching *.out files
test: build
	@for in_file in tests/$(PROBLEM)/*.in; do \
		name=$$(basename "$$in_file" .in); \
		out_file="tests/$(PROBLEM)/$$name.out"; \
		actual_file="$(BUILD_DIR)/$(PROBLEM)-$$name.out"; \
		echo "Running $$name..."; \
		./$(BIN) < "$$in_file" > "$$actual_file"; \
		diff -u "$$out_file" "$$actual_file" || exit 1; \
	done
	@echo "All tests passed."

clean:
	rm -rf $(BUILD_DIR)
