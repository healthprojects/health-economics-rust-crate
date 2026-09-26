//! # Markov Cohort Simulation
//!
//! The standard HTA modelling technique for interventions whose effects
//! unfold over multiple time periods (cycles): a hypothetical cohort starts
//! entirely in one health state, and each cycle a fixed set of transition
//! probabilities move fractions of the cohort between states. Costs and
//! QALYs accrue each cycle in proportion to how much of the cohort occupies
//! each state, and are discounted back to present value.
//!
//! ## Formula
//!
//! ```text
//! One cycle's cohort update (row-vector x matrix):
//!   new_state[j] = sum_i state[i] * transition_matrix[i][j]
//!
//! One cycle's cost:
//!   cycle_cost = sum_s state[s] * cost_per_cycle[s]
//!
//! One cycle's QALYs:
//!   cycle_qalys = sum_s state[s] * utility[s] * cycle_length_years
//!
//! Full simulation over `cycles` cycles, discounted at `discount_rate`:
//!   total_discounted_cost  = sum_{t=0}^{cycles-1} cycle_cost(state_t)  / (1+discount_rate)^t
//!   total_discounted_qalys = sum_{t=0}^{cycles-1} cycle_qalys(state_t) / (1+discount_rate)^t
//!   where state_0 = initial_distribution, state_{t+1} = advance_cohort(state_t, transition_matrix)
//! ```
//!
//! ## Why it matters
//!
//! Most real HTA decisions are not one-shot: a disease progresses, relapses,
//! or kills over years, and a single-period [`crate::cost_effectiveness_analysis`]
//! cannot represent that. A Markov cohort model is how
//! [`crate::health_technology_assessment`] bodies actually simulate a chronic
//! condition — states such as Well, Progressed, and Dead, with annual
//! transition probabilities calibrated from trial or registry data, run for
//! enough cycles to cover a lifetime horizon, then
//! [discounted][`crate::discounting_and_time_preference`] back to present
//! value like any other multi-year stream.
//!
//! ## Example
//!
//! A 2-state model — `Well` and `Dead` — where 10% of the cohort dies each
//! cycle and `Dead` is absorbing (its self-transition probability is 1.0).
//! The cohort starts entirely `Well`, costs £1,000/cycle while `Well`
//! (£0 once `Dead`), and gains 0.8 QALYs/year while `Well`. Simulated for 3
//! annual cycles at a 3.5% discount rate:
//!
//! ```
//! use health_economics::markov_cohort_simulation::simulate_cohort;
//!
//! let initial_distribution = vec![1.0, 0.0]; // [Well, Dead]
//! let transition_matrix = vec![
//!     vec![0.9, 0.1], // Well -> 90% stay Well, 10% die
//!     vec![0.0, 1.0], // Dead -> Dead (absorbing self-loop)
//! ];
//! let cost_per_cycle = vec![1_000.0, 0.0];
//! let utility = vec![0.8, 0.0];
//!
//! let (total_discounted_cost, total_discounted_qalys) = simulate_cohort(
//!     &initial_distribution,
//!     &transition_matrix,
//!     &cost_per_cycle,
//!     &utility,
//!     1.0,
//!     3,
//!     0.035,
//! )
//! .unwrap();
//!
//! assert!((total_discounted_cost - 2_625.708_884_688_091).abs() < 1e-6);
//! assert!((total_discounted_qalys - 2.100_567_107_750_473_3).abs() < 1e-6);
//! ```
//!
//! ## Software engineering connection
//!
//! A Markov cohort model is structurally a state machine with probabilistic
//! transitions, run for a fixed number of ticks, discounting each tick's
//! value. The same shape simulates a user cohort's retention/state
//! transitions over time (see [`crate::retention_and_churn`]) or a reliability
//! model's state transitions accruing cost each tick (see [`crate::dora_metrics`]'s
//! downtime-harm framing) — "what fraction of users/systems are in which
//! state this period, and what does that cost or earn us" is the same
//! question whether the states are clinical or operational.
//!
//! ## Pitfalls
//!
//! - **Transition probabilities that don't sum to 1 per row** silently
//!   produce a cohort that "leaks" or "grows" mass. This module does not
//!   validate that rows sum to 1 (matching this crate's usual
//!   trust-the-caller's-inputs convention) — a caller should check this
//!   themselves before trusting the output.
//! - **Too coarse a cycle length** relative to how fast the modelled disease
//!   actually progresses biases results; a `cycle_length_years` much longer
//!   than the disease's real dynamics understates transitions that happen
//!   mid-cycle.
//! - **Forgetting an absorbing state's self-loop.** An absorbing state (such
//!   as `Dead`) needs a self-transition probability of exactly 1.0 — omit it
//!   and cohort mass vanishes from the model after one cycle in that state.
//!
//! ## Sources
//!
//! - Sonnenberg FA, Beck JR. "Markov models in medical decision making: a
//!   practical guide." `Med Decis Making`. 1993;13(4):322-38.
//! - Briggs A, Sculpher M. "An introduction to Markov modelling for economic
//!   evaluation." `Pharmacoeconomics`. 1998;13(4):397-409.
//!
//! Topic doc: health-economics-metrics/topics/markov-cohort-simulation.md

/// Advances the cohort one cycle: `new_state[j] = sum_i state[i] * transition_matrix[i][j]`.
///
/// # Arguments
///
/// * `state` — the cohort's distribution across states this cycle (fractions
///   or counts, one entry per state).
/// * `transition_matrix` — one row per state; row `i` gives the
///   probabilities of moving from state `i` to every state `j`.
///
/// # Returns
///
/// The cohort's distribution next cycle, or `None` if
/// `transition_matrix.len() != state.len()`, or any row's length
/// `!= state.len()`.
///
/// # Examples
///
/// ```
/// use health_economics::markov_cohort_simulation::advance_cohort;
///
/// let state = vec![1.0, 0.0];
/// let transition_matrix = vec![vec![0.9, 0.1], vec![0.0, 1.0]];
/// let next = advance_cohort(&state, &transition_matrix).unwrap();
/// assert!((next[0] - 0.9).abs() < 1e-9);
/// assert!((next[1] - 0.1).abs() < 1e-9);
/// ```
#[must_use]
pub fn advance_cohort(state: &[f64], transition_matrix: &[Vec<f64>]) -> Option<Vec<f64>> {
    let n = state.len();
    if transition_matrix.len() != n || transition_matrix.iter().any(|row| row.len() != n) {
        return None;
    }
    Some(
        (0..n)
            .map(|j| (0..n).map(|i| state[i] * transition_matrix[i][j]).sum())
            .collect(),
    )
}

/// One cycle's cost: `sum_s state[s] * cost_per_cycle[s]`.
///
/// # Arguments
///
/// * `state` — the cohort's distribution across states this cycle.
/// * `cost_per_cycle` — the cost incurred per unit of cohort occupying each
///   state, for one cycle.
///
/// # Returns
///
/// The cycle's total cost, or `None` if `cost_per_cycle.len() != state.len()`.
///
/// # Examples
///
/// ```
/// use health_economics::markov_cohort_simulation::cycle_cost;
///
/// let state = vec![0.9, 0.1];
/// let cost_per_cycle = vec![1_000.0, 0.0];
/// let cost = cycle_cost(&state, &cost_per_cycle).unwrap();
/// assert!((cost - 900.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn cycle_cost(state: &[f64], cost_per_cycle: &[f64]) -> Option<f64> {
    if state.len() != cost_per_cycle.len() {
        return None;
    }
    Some(state.iter().zip(cost_per_cycle).map(|(s, c)| s * c).sum())
}

/// One cycle's QALYs: `sum_s state[s] * utility[s] * cycle_length_years`.
///
/// # Arguments
///
/// * `state` — the cohort's distribution across states this cycle.
/// * `utility` — the health-state utility (0-1 scale) of each state.
/// * `cycle_length_years` — the length of one cycle, in years (e.g. `1.0`
///   for an annual cycle).
///
/// # Returns
///
/// The cycle's total QALYs, or `None` if `utility.len() != state.len()`.
///
/// # Examples
///
/// ```
/// use health_economics::markov_cohort_simulation::cycle_qalys;
///
/// let state = vec![0.9, 0.1];
/// let utility = vec![0.8, 0.0];
/// let qalys = cycle_qalys(&state, &utility, 1.0).unwrap();
/// assert!((qalys - 0.72).abs() < 1e-9);
/// ```
#[must_use]
pub fn cycle_qalys(state: &[f64], utility: &[f64], cycle_length_years: f64) -> Option<f64> {
    if state.len() != utility.len() {
        return None;
    }
    Some(state.iter().zip(utility).map(|(s, u)| s * u).sum::<f64>() * cycle_length_years)
}

/// Runs a full multi-cycle Markov cohort simulation, discounting each
/// cycle's cost and QALYs back to present value.
///
/// Each cycle: the current state's cost and QALYs are computed (via
/// [`cycle_cost`] and [`cycle_qalys`]), discounted by
/// `1 / (1 + discount_rate)^t`, and added to the running totals; then the
/// cohort is advanced (via [`advance_cohort`]) to the next cycle's state.
///
/// # Arguments
///
/// * `initial_distribution` — the cohort's starting distribution across
///   states (cycle `t = 0`).
/// * `transition_matrix` — one row per state; row `i` gives the
///   probabilities of moving from state `i` to every state `j`.
/// * `cost_per_cycle` — the cost incurred per unit of cohort occupying each
///   state, for one cycle.
/// * `utility` — the health-state utility (0-1 scale) of each state.
/// * `cycle_length_years` — the length of one cycle, in years.
/// * `cycles` — the number of cycles to simulate (`t = 0..cycles`).
/// * `discount_rate` — annual discount rate as a fraction (e.g. `0.035`).
///
/// # Returns
///
/// `Some((total_discounted_cost, total_discounted_qalys))`, or `None` if
/// `initial_distribution`, `transition_matrix`, `cost_per_cycle`, or
/// `utility` have mismatched shapes.
///
/// # Examples
///
/// ```
/// use health_economics::markov_cohort_simulation::simulate_cohort;
///
/// let initial_distribution = vec![1.0, 0.0];
/// let transition_matrix = vec![vec![0.9, 0.1], vec![0.0, 1.0]];
/// let cost_per_cycle = vec![1_000.0, 0.0];
/// let utility = vec![0.8, 0.0];
///
/// let (cost, qalys) = simulate_cohort(
///     &initial_distribution,
///     &transition_matrix,
///     &cost_per_cycle,
///     &utility,
///     1.0,
///     3,
///     0.035,
/// )
/// .unwrap();
/// assert!((cost - 2_625.708_884_688_091).abs() < 1e-6);
/// assert!((qalys - 2.100_567_107_750_473_3).abs() < 1e-6);
/// ```
#[must_use]
pub fn simulate_cohort(
    initial_distribution: &[f64],
    transition_matrix: &[Vec<f64>],
    cost_per_cycle: &[f64],
    utility: &[f64],
    cycle_length_years: f64,
    cycles: usize,
    discount_rate: f64,
) -> Option<(f64, f64)> {
    let n = initial_distribution.len();
    if transition_matrix.len() != n
        || transition_matrix.iter().any(|row| row.len() != n)
        || cost_per_cycle.len() != n
        || utility.len() != n
    {
        return None;
    }

    let mut state = initial_distribution.to_vec();
    let mut total_discounted_cost = 0.0;
    let mut total_discounted_qalys = 0.0;

    for t in 0..cycles {
        let cost = cycle_cost(&state, cost_per_cycle)?;
        let qalys = cycle_qalys(&state, utility, cycle_length_years)?;
        // `cycles` is a small human-scale cycle count (single/double digits
        // to low hundreds for a lifetime horizon), well within f64's 52-bit
        // mantissa.
        #[allow(clippy::cast_precision_loss)]
        let discount_factor = 1.0 / (1.0 + discount_rate).powf(t as f64);
        total_discounted_cost += cost * discount_factor;
        total_discounted_qalys += qalys * discount_factor;
        state = advance_cohort(&state, transition_matrix)?;
    }

    Some((total_discounted_cost, total_discounted_qalys))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-6;

    fn worked_example_inputs() -> (Vec<f64>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
        (
            vec![1.0, 0.0],
            vec![vec![0.9, 0.1], vec![0.0, 1.0]],
            vec![1_000.0, 0.0],
            vec![0.8, 0.0],
        )
    }

    #[test]
    fn advance_cohort_moves_ten_percent_to_dead() {
        let (state, transition_matrix, _, _) = worked_example_inputs();
        let next = advance_cohort(&state, &transition_matrix).unwrap();
        assert!((next[0] - 0.9).abs() < TOL, "got {next:?}");
        assert!((next[1] - 0.1).abs() < TOL, "got {next:?}");
    }

    #[test]
    fn advance_cohort_none_on_shape_mismatch() {
        let state = vec![1.0, 0.0];
        let bad_matrix = vec![vec![0.9, 0.1]]; // only one row for a 2-state cohort
        assert!(advance_cohort(&state, &bad_matrix).is_none());

        let bad_row_matrix = vec![vec![0.9, 0.1, 0.0], vec![0.0, 1.0]];
        assert!(advance_cohort(&state, &bad_row_matrix).is_none());
    }

    #[test]
    fn cycle_cost_weights_by_state_occupancy() {
        let (_, _, cost_per_cycle, _) = worked_example_inputs();
        let state = vec![0.9, 0.1];
        let cost = cycle_cost(&state, &cost_per_cycle).unwrap();
        assert!((cost - 900.0).abs() < TOL, "got {cost}");
    }

    #[test]
    fn cycle_cost_none_on_shape_mismatch() {
        assert!(cycle_cost(&[1.0, 0.0], &[1_000.0]).is_none());
    }

    #[test]
    fn cycle_qalys_weights_by_state_and_utility() {
        let (_, _, _, utility) = worked_example_inputs();
        let state = vec![0.9, 0.1];
        let qalys = cycle_qalys(&state, &utility, 1.0).unwrap();
        assert!((qalys - 0.72).abs() < TOL, "got {qalys}");
    }

    #[test]
    fn cycle_qalys_none_on_shape_mismatch() {
        assert!(cycle_qalys(&[1.0, 0.0], &[0.8], 1.0).is_none());
    }

    // Worked example: 2-state Well/Dead model, 3 annual cycles at 3.5%
    // discount, reproduced exactly from the module doc's doctest.
    #[test]
    fn worked_example_simulate_cohort_matches_doctest() {
        let (initial_distribution, transition_matrix, cost_per_cycle, utility) =
            worked_example_inputs();
        let (cost, qalys) = simulate_cohort(
            &initial_distribution,
            &transition_matrix,
            &cost_per_cycle,
            &utility,
            1.0,
            3,
            0.035,
        )
        .unwrap();
        assert!((cost - 2_625.708_884_688_091).abs() < TOL, "got {cost}");
        assert!((qalys - 2.100_567_107_750_473_3).abs() < TOL, "got {qalys}");
    }

    #[test]
    fn simulate_cohort_none_on_shape_mismatch() {
        let initial_distribution = vec![1.0, 0.0];
        let transition_matrix = vec![vec![0.9, 0.1], vec![0.0, 1.0]];
        let bad_cost = vec![1_000.0]; // wrong length
        let utility = vec![0.8, 0.0];
        assert!(simulate_cohort(
            &initial_distribution,
            &transition_matrix,
            &bad_cost,
            &utility,
            1.0,
            3,
            0.035
        )
        .is_none());
    }

    #[test]
    fn zero_cycles_returns_zero_totals() {
        let (initial_distribution, transition_matrix, cost_per_cycle, utility) =
            worked_example_inputs();
        let (cost, qalys) = simulate_cohort(
            &initial_distribution,
            &transition_matrix,
            &cost_per_cycle,
            &utility,
            1.0,
            0,
            0.035,
        )
        .unwrap();
        assert!((cost - 0.0).abs() < TOL, "got {cost}");
        assert!((qalys - 0.0).abs() < TOL, "got {qalys}");
    }
}
