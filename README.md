![Rust tests](https://github.com/codeblessing/co-pcmax/workflows/Rust%20tests/badge.svg?event=push)

# Task scheduling

This is solution resolving Task Scheduling Problem using optimized metaheuristics.

# Problem

P||Cmax is problem of finding optimal(minimal) time needed to complete set of indivisible tasks on given number of identical processors.

![][PCMAX1]

Simple approximate greedy algorithm takes tasks in order and puts them in processors with shortest working time.

![][PCMAX2]

Statistically better approximation is given by Longest Tasks First (LTF) algorithm, though it requires sorting task set before scheduling.

![][PCMAX3]

# Heuristics


[PCMAX1]: docs/img/pcmax1.png
[PCMAX2]: docs/img/pcmax2.png
[PCMAX3]: docs/img/pcmax3.png