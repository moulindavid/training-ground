# CSES Java

Simple setup: one `.java` file per problem, compiled directly with `javac`.

## Run with direct input

```bash
make run PROBLEM=WeirdAlgorithm INPUT=3
```

## Run with input file

```bash
make run PROBLEM=WeirdAlgorithm FILE=tests/weird_algorithm/sample.in
```

## Test sample

```bash
make test PROBLEM=WeirdAlgorithm TEST=weird_algorithm
```

For CSES submissions, you can rename the class/file to `Main` if needed.
