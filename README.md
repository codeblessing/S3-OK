![Rust tests](https://github.com/codeblessing/co-pcmax/workflows/Rust%20tests/badge.svg?event=push)

# Task scheduling

This is solution resolving Task Scheduling Problem using optimized metaheuristics.

## Modules

This solution is divided into several modules:

- [generator](src/generator.rs) - creates sample test cases. User can set up number of available cores, number of tasks and theirs length or get randomized cases.

- [greedy](src/greedy.rs) - implements simplest, sequential, greedy algorithm for comparison with more optimized solutions given by metaheuristics.