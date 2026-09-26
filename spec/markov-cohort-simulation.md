# Spec: Markov Cohort Simulation

- **Module**: [`src/markov_cohort_simulation.rs`](../src/markov_cohort_simulation.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/markov-cohort-simulation.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `advance_cohort(state: &[f64], transition_matrix: &[Vec<f64>]) -> Option<Vec<f64>>`

- Formula: `new_state[j] = Σ_i state[i] * transition_matrix[i][j]` (row-vector × matrix).
- Returns `None` iff `transition_matrix.len() != state.len()`, or any row's length `!= state.len()` (malformed shape — never indexes out of bounds).
- Worked example: `advance_cohort(&[1.0, 0.0], &[vec![0.9, 0.1], vec![0.0, 1.0]]) == Some(vec![0.9, 0.1])`

### `cycle_cost(state: &[f64], cost_per_cycle: &[f64]) -> Option<f64>`

- Formula: `Σ_s state[s] * cost_per_cycle[s]`
- Returns `None` iff `cost_per_cycle.len() != state.len()`.
- Worked example: `cycle_cost(&[0.9, 0.1], &[1_000.0, 0.0]) == Some(900.0)`

### `cycle_qalys(state: &[f64], utility: &[f64], cycle_length_years: f64) -> Option<f64>`

- Formula: `(Σ_s state[s] * utility[s]) * cycle_length_years`
- Returns `None` iff `utility.len() != state.len()`.
- Worked example: `cycle_qalys(&[0.9, 0.1], &[0.8, 0.0], 1.0) == Some(0.72)`

### `simulate_cohort(initial_distribution: &[f64], transition_matrix: &[Vec<f64>], cost_per_cycle: &[f64], utility: &[f64], cycle_length_years: f64, cycles: usize, discount_rate: f64) -> Option<(f64, f64)>`

- Formula: runs `cycle_cost`/`cycle_qalys` on each cycle's state, discounts each cycle's values by `1 / (1 + discount_rate)^t`, sums both series, and advances the cohort via `advance_cohort` between cycles. Returns `(total_discounted_cost, total_discounted_qalys)`.
- Returns `None` iff `initial_distribution`, `transition_matrix`, `cost_per_cycle`, or `utility` have mismatched shapes (propagated from the three functions above).
- Worked example: a 2-state Well/Dead model (`initial_distribution = [1.0, 0.0]`, `transition_matrix = [[0.9, 0.1], [0.0, 1.0]]`, `cost_per_cycle = [1_000.0, 0.0]`, `utility = [0.8, 0.0]`, `cycle_length_years = 1.0`, `cycles = 3`, `discount_rate = 0.035`) gives `total_discounted_cost ≈ 2_625.708_884_688_091` and `total_discounted_qalys ≈ 2.100_567_107_750_473_3` (verified by running the code, not hand-derived).

## Invariants

- `simulate_cohort` with `cycles = 0` returns `(0.0, 0.0)` — no cycles run, nothing accrues.
- An absorbing state (self-transition probability `1.0`) never loses cohort mass in subsequent cycles; omitting that self-loop causes mass to vanish, per the module's documented pitfall.
- This module does not validate that each transition-matrix row sums to `1.0` — the caller is responsible for supplying a well-formed matrix, matching this crate's usual trust-the-caller's-inputs convention for anything beyond the zero/shape checks already covered above.
