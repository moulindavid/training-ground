JAVAC := javac
JAVA := java
BUILD_DIR := build
PROBLEM ?= WeirdAlgorithm
SRC := $(PROBLEM).java

.PHONY: build run test clean

build: $(SRC)
	mkdir -p $(BUILD_DIR)
	$(JAVAC) -d $(BUILD_DIR) $(SRC)

# Usage:
#   make run PROBLEM=WeirdAlgorithm INPUT=3
#   make run PROBLEM=WeirdAlgorithm FILE=tests/weird_algorithm/sample.in
run: build
ifdef FILE
	$(JAVA) -cp $(BUILD_DIR) $(PROBLEM) < $(FILE)
else
	printf '%s\n' '$(INPUT)' | $(JAVA) -cp $(BUILD_DIR) $(PROBLEM)
endif

# Expects tests/<problem_snake_case>/*.in and matching *.out files
# Usage: make test PROBLEM=WeirdAlgorithm TEST=weird_algorithm
test: build
	@for in_file in tests/$(TEST)/*.in; do \
		name=$$(basename "$$in_file" .in); \
		out_file="tests/$(TEST)/$$name.out"; \
		actual_file="$(BUILD_DIR)/$(PROBLEM)-$$name.out"; \
		echo "Running $$name..."; \
		$(JAVA) -cp $(BUILD_DIR) $(PROBLEM) < "$$in_file" > "$$actual_file"; \
		diff -u "$$out_file" "$$actual_file" || exit 1; \
	done
	@echo "All tests passed."

clean:
	rm -rf $(BUILD_DIR)
