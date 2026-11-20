# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)

## Benchmark Results

Run `bin/test-benchmarks` to measure three representative workloads in release mode:

- a scalar net-monetary-benefit calculation;
- a 40-cycle, two-state Markov cohort simulation;
- a deterministic 10,000-draw migration PSA.

The benchmark uses `std::hint::black_box` and `std::time::Instant`, so it adds no
dependency. Results are informational rather than pass/fail: absolute timing is
machine- and toolchain-specific, and a hard wall-clock CI threshold would be
flaky. Compare runs on the same machine and toolchain when investigating a
regression.

One Apple Silicon development-machine run on 2026-11-19 with Rust 1.100.0
reported approximately 0.64 ns per scalar NMB call, 668 ns per 40-cycle
two-state Markov run, and 0.21 ms per 10,000-draw PSA. These figures are a
dated orientation point, not a portable guarantee.

The library's dominant operations are constant-time scalar arithmetic. Slice
metrics are linear in input length; exact AUROC is quadratic in the product of
positive and negative sample counts; Markov simulation scales with cycles times
the square of the number of states; Monte Carlo scales linearly with draw count.

