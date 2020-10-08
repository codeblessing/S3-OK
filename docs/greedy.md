# GREEDY MODULE

## Overview

This module consists of implementation of simplest, sequential greedy algorithm solving TSP. It's used as comparison with metaheuristic solution in terms of speed and optimization of solution.

## Core

### Struct

```rust
struct Core {
    working_time: u128,
}
```
Core struct is used for representing some computing core and its `working_time` (time spent on processing tasks).

### Functions

```rust
fn new() -> Self
```
Creates `Core` object with zeroed working time.

```rust
fn add(&mut self, time: u128) -> &mut Self
```
Adds up `time` to core's working time.

## Unassociated functions

```rust
pub fn schedule(tasks: &Case) -> u128
```
Implementation of simplest, sequential greedy algoritm solving TSP. Returns endtime - longest working time from cores, time after completing all tasks.

```
_____________
|___|____|__|
|_____|_|__|____
|____|_________| <- this is endtime
```