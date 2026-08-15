---
title: Measuring performance without fooling yourself
date: 2026-08-07
tags: [benchmarking, tools, ideas]
summary: Benchmarks lie. Here is how to read them, and how to run them so they tell the truth.
---

Benchmarks lie. They lie because the person running them usually wants a
particular answer. Here is how to read a benchmark, and how to run one so it
tells the truth.

## The three lies of benchmarking

1. **The cherry-picked case.** Someone shows you the one input where their
   code wins. They do not show the other ninety-nine.
2. **The missing baseline.** A number with no comparison is not a benchmark.
   It is a factoid.
3. **The wrong metric.** Throughput and latency are different things. P50 and
   P99 are different things. Pick the one that matches your problem.

## What to measure

| Metric | What it tells you |
| ------ | ----------------- |
| Latency | How long one request takes |
| Throughput | How many requests per second |
| P99 | The worst normal case |
| Error rate | How often it fails |

## A fair comparison

A fair comparison is boring. That is the point. You control everything except
the one thing you are testing.

```text
same machine
same input
same warm-up
same number of runs
same measurement
```

## A small Rust benchmark

```rust
use std::time::Instant;

fn main() {
    let n = 1_000_000;
    let start = Instant::now();
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(i);
    }
    let elapsed = start.elapsed();
    println!("sum={sum} in {:?}", elapsed);
}
```

## The honest conclusion

> A benchmark does not prove your code is fast. It proves it was fast once,
> on one machine, for one input.

Run it many times. Change one thing at a time. Write down what you did. Then
you can trust the number.
