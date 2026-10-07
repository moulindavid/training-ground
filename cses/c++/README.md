# CSES C++

Simplest setup: one `.cpp` file per problem, plus optional sample tests.

## Run with direct input

```bash
make run PROBLEM=weird_algorithm INPUT=3
```

## Run with input file

```bash
make run PROBLEM=weird_algorithm FILE=tests/weird_algorithm/sample.in
```

## Test sample

```bash
make test PROBLEM=weird_algorithm
```

Add a new problem by creating:

- `<problem_name>.cpp`
- `tests/<problem_name>/sample.in`
- `tests/<problem_name>/sample.out`
