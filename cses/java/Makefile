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

# Expects tests/<problem_snake_case>/sample.in and sample.out
# Usage: make test PROBLEM=WeirdAlgorithm TEST=weird_algorithm
test: build
	$(JAVA) -cp $(BUILD_DIR) $(PROBLEM) < tests/$(TEST)/sample.in > $(BUILD_DIR)/$(PROBLEM).out
	diff -u tests/$(TEST)/sample.out $(BUILD_DIR)/$(PROBLEM).out

clean:
	rm -rf $(BUILD_DIR)
