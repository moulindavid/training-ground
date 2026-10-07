# CSES Rust

Simple setup: one `.rs` file per problem, compiled directly with `rustc`.

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

- `<problem_name>.rs`
- `tests/<problem_name>/sample.in`
- `tests/<problem_name>/sample.out`

No Cargo needed for simple CSES-style single-file problems.
