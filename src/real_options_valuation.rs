//! # Real Options Valuation
//!
//! Real options valuation applies financial option-pricing logic to real
//! (non-financial-market) investment decisions — specifically here, the
//! *option to expand* a project later if it succeeds, without being
//! obligated to. A simplified one-period binomial model (Cox, Ross,
//! Rubinstein, 1979) values this: the project's value either rises
//! (`up_factor`) or falls (`down_factor`) by the next decision point; the
//! option to expand is only exercised if it is profitable in that state
//! (payoff floored at zero, since it is a genuine *option*, not an
//! obligation); a risk-neutral probability weights the two states.
//!
//! ## Formula
//!
//! ```text
//! Risk-neutral probability of the "up" state:
//!   p = ((1 + risk_free_rate) − down_factor) / (up_factor − down_factor)
//!
//! Expansion payoff in each state (floored at zero — expanding is optional):
//!   payoff_up   = max(project_value × up_factor   − expansion_cost, 0)
//!   payoff_down = max(project_value × down_factor − expansion_cost, 0)
//!
//! Option value (discounted expected payoff):
//!   option_value = (p × payoff_up + (1 − p) × payoff_down) / (1 + risk_free_rate)
//!
//! Expanded NPV = static_npv + option_value
//! ```
//!
//! ## Why it matters
//!
//! A static NPV calculation prices a project as an all-or-nothing bet: fund
//! it or don't, at today's scale, forever. Real projects — and especially
//! phased digital health rollouts — are rarely bet that way: a health
//! system can fund a small pilot, watch what happens, and only commit
//! further money if it works. That flexibility has real value, and ignoring
//! it systematically undervalues staged investments relative to
//! one-shot ones. Real options valuation prices the flexibility itself, so
//! a phased proposal can be compared fairly against a full-commitment
//! alternative rather than penalised for looking smaller on a naive NPV
//! line.
//!
//! ## Example
//!
//! A digital service pilot with `project_value = £1,000,000`, a possible
//! rise to 1.5× or fall to 0.5× by the next decision point, an 8%
//! risk-free rate, and an expansion cost of £600,000:
//!
//! ```rust
//! use health_economics::real_options_valuation::{
//!     risk_neutral_probability, option_to_expand_value, expanded_npv,
//! };
//!
//! // p = (1.08 - 0.5) / (1.5 - 0.5) = 0.58
//! let p = risk_neutral_probability(0.08, 0.5, 1.5).unwrap();
//! assert!((p - 0.58).abs() < 1e-9);
//!
//! // payoff_up   = max(1,000,000 x 1.5 - 600,000, 0) =  900,000
//! // payoff_down = max(1,000,000 x 0.5 - 600,000, 0) = max(-100,000, 0) = 0
//! // (the floor matters: the option would NOT be exercised if the market
//! // disappoints)
//! //
//! // option_value = (0.58 x 900,000 + 0.42 x 0) / 1.08 = 522,000 / 1.08
//! //              ~= 483,333.333333...
//! let option_value = option_to_expand_value(1_000_000.0, 1.5, 0.5, 600_000.0, 0.08).unwrap();
//! assert!((option_value - 483_333.333_333).abs() < 1e-6);
//!
//! // Adding the option's value to a static-NPV baseline of £200,000:
//! let npv = expanded_npv(200_000.0, option_value);
//! assert!((npv - 683_333.333_333).abs() < 1e-6);
//! ```
//!
//! ## Software engineering connection
//!
//! This is the formal version of "ship a minimum version now, keep the
//! option to invest further if it takes off" — directly relevant to a
//! phased digital health product rollout, structurally parallel to
//! [`crate::cost_of_delay`]/[`crate::wsjf_and_cd3`]'s sequencing-under-uncertainty
//! framing, and complementary to
//! [`crate::expected_value_of_perfect_information`]/[`crate::expected_value_of_sample_information`]
//! (all three price flexibility or information under uncertainty, from
//! different angles).
//!
//! ## Pitfalls
//!
//! - **Borrowing risk-neutral pricing without the traded-asset assumption
//!   it relies on**: real options models borrow risk-neutral probability
//!   from financial option pricing, which assumes the underlying value is
//!   a *traded* asset — for a genuinely non-traded real project this is a
//!   modelling convenience, not a literal market fact.
//! - **Treating `up_factor`/`down_factor` as free parameters**: the
//!   binomial `up_factor`/`down_factor` inputs are themselves assumptions
//!   requiring justification, not free parameters chosen to produce a
//!   desired answer.
//! - **Reporting the option value alone**: real options value is *additive*
//!   to a standalone project's static NPV — [`expanded_npv`] makes this
//!   explicit as a sum, but a common error is reporting only the option
//!   value and dropping the base case.
//!
//! ## Sources
//!
//! - Cox JC, Ross SA, Rubinstein M. "Option pricing: a simplified
//!   approach." J Financ Econ. 1979;7(3):229-63.
//! - Trigeorgis L. "Real Options: Managerial Flexibility and Strategy in
//!   Resource Allocation." MIT Press. 1996.
//! - Driffield T, Smith PC. "A real options approach to watchful waiting:
//!   theory and an illustration." Med Decis Making. 2007;27(2):178-88 (ties
//!   real options directly to a health-economics decision context).
//!
//! Topic doc: health-economics-metrics/topics/real-options-valuation.md

/// Risk-neutral probability of the "up" state in a one-period binomial model.
///
/// `p = ((1 + risk_free_rate) − down_factor) / (up_factor − down_factor)`.
///
/// # Arguments
///
/// * `risk_free_rate` — the per-period risk-free rate (e.g. `0.08` for 8%).
/// * `down_factor` — the multiplier applied to the project's value in the
///   "down" state (e.g. `0.5` for a 50% fall).
/// * `up_factor` — the multiplier applied to the project's value in the "up"
///   state (e.g. `1.5` for a 50% rise).
///
/// # Returns
///
/// `Some(p)`, or `None` if `up_factor == down_factor` (the two states are
/// indistinguishable, so no probability can be recovered).
///
/// # Examples
///
/// ```rust
/// use health_economics::real_options_valuation::risk_neutral_probability;
///
/// // p = (1.08 - 0.5) / (1.5 - 0.5) = 0.58.
/// let p = risk_neutral_probability(0.08, 0.5, 1.5).unwrap();
/// assert!((p - 0.58).abs() < 1e-9);
///
/// // Indistinguishable up/down states are undefined.
/// assert_eq!(risk_neutral_probability(0.08, 1.0, 1.0), None);
/// ```
#[must_use]
pub fn risk_neutral_probability(
    risk_free_rate: f64,
    down_factor: f64,
    up_factor: f64,
) -> Option<f64> {
    // A deliberate exact-equality guard, not an approximate comparison: this
    // rejects the degenerate model input where the two binomial states
    // collapse into one (the division below would otherwise be by zero).
    #[allow(clippy::float_cmp)]
    let states_are_indistinguishable = up_factor == down_factor;

    if states_are_indistinguishable {
        None
    } else {
        Some(((1.0 + risk_free_rate) - down_factor) / (up_factor - down_factor))
    }
}

/// Value of the option to expand a project at the next decision point.
///
/// Computes the risk-neutral probability of the "up" state, the expansion
/// payoff in each state (floored at zero, since expanding is optional), and
/// the discounted expected payoff.
///
/// # Arguments
///
/// * `project_value` — the project's current value (e.g. `£1,000,000`).
/// * `up_factor` — the multiplier applied to `project_value` in the "up"
///   state.
/// * `down_factor` — the multiplier applied to `project_value` in the
///   "down" state.
/// * `expansion_cost` — the cost of exercising the expansion option.
/// * `risk_free_rate` — the per-period risk-free rate.
///
/// # Returns
///
/// `Some(option_value)`, or `None` if [`risk_neutral_probability`] returns
/// `None` (i.e. `up_factor == down_factor`).
///
/// # Examples
///
/// ```rust
/// use health_economics::real_options_valuation::option_to_expand_value;
///
/// // payoff_up = 900,000, payoff_down = 0 (floored), option_value ~= 483,333.33.
/// let option_value = option_to_expand_value(1_000_000.0, 1.5, 0.5, 600_000.0, 0.08).unwrap();
/// assert!((option_value - 483_333.333_333).abs() < 1e-6);
/// ```
#[must_use]
pub fn option_to_expand_value(
    project_value: f64,
    up_factor: f64,
    down_factor: f64,
    expansion_cost: f64,
    risk_free_rate: f64,
) -> Option<f64> {
    let p = risk_neutral_probability(risk_free_rate, down_factor, up_factor)?;

    let payoff_up = (project_value * up_factor - expansion_cost).max(0.0);
    let payoff_down = (project_value * down_factor - expansion_cost).max(0.0);

    Some((p * payoff_up + (1.0 - p) * payoff_down) / (1.0 + risk_free_rate))
}

/// Expanded NPV: a static NPV plus the value of the option to expand.
///
/// Real options value is *additive* to a standalone project's static NPV —
/// this function makes that sum explicit, so it is never reported (or
/// mistakenly compared) in isolation from the base case.
///
/// # Arguments
///
/// * `static_npv` — the project's NPV without the expansion option.
/// * `option_value` — the value of the option to expand (e.g. from
///   [`option_to_expand_value`]).
///
/// # Returns
///
/// `static_npv + option_value`.
///
/// # Examples
///
/// ```rust
/// use health_economics::real_options_valuation::expanded_npv;
///
/// let npv = expanded_npv(200_000.0, 483_333.333_333);
/// assert!((npv - 683_333.333_333).abs() < 1e-6);
/// ```
#[must_use]
pub fn expanded_npv(static_npv: f64, option_value: f64) -> f64 {
    static_npv + option_value
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real options worked examples don't always land on a terminating
    // decimal (the 522,000 / 1.08 division below repeats), so this module
    // uses a looser tolerance than the crate's usual 1e-9.
    const TOL: f64 = 1e-6;

    // Worked example: p = (1.08 - 0.5) / (1.5 - 0.5) = 0.58.
    #[test]
    fn worked_example_risk_neutral_probability_is_0_58() {
        let p = risk_neutral_probability(0.08, 0.5, 1.5).unwrap();
        assert!((p - 0.58).abs() < 1e-9, "got {p}");
    }

    // Worked example: payoff_up = max(1,000,000 x 1.5 - 600,000, 0) = 900,000.
    #[test]
    fn worked_example_up_payoff_is_900_000() {
        let payoff_up = (1_000_000.0_f64 * 1.5 - 600_000.0).max(0.0);
        assert!((payoff_up - 900_000.0).abs() < TOL, "got {payoff_up}");
    }

    // Worked example: payoff_down = max(1,000,000 x 0.5 - 600,000, 0)
    // = max(-100,000, 0) = 0 -- the floor matters.
    #[test]
    fn worked_example_down_payoff_is_floored_at_zero() {
        let payoff_down = (1_000_000.0_f64 * 0.5 - 600_000.0).max(0.0);
        assert!((payoff_down - 0.0).abs() < TOL, "got {payoff_down}");
    }

    // Worked example: option_value = (0.58 x 900,000 + 0.42 x 0) / 1.08
    // ~= 483,333.333333.
    #[test]
    fn worked_example_option_value_is_483_333_33() {
        let option_value = option_to_expand_value(1_000_000.0, 1.5, 0.5, 600_000.0, 0.08).unwrap();
        assert!(
            (option_value - 483_333.333_333).abs() < TOL,
            "got {option_value}"
        );
    }

    // Worked example: expanded NPV = 200,000 + 483,333.33 ~= 683,333.33.
    #[test]
    fn worked_example_expanded_npv_is_683_333_33() {
        let option_value = option_to_expand_value(1_000_000.0, 1.5, 0.5, 600_000.0, 0.08).unwrap();
        let npv = expanded_npv(200_000.0, option_value);
        assert!((npv - 683_333.333_333).abs() < TOL, "got {npv}");
    }

    // Edge case: indistinguishable up/down states are undefined.
    #[test]
    fn equal_up_and_down_factors_return_none() {
        assert!(risk_neutral_probability(0.08, 1.0, 1.0).is_none());
        assert!(option_to_expand_value(1_000_000.0, 1.0, 1.0, 600_000.0, 0.08).is_none());
    }

    // Edge case: when both states are unprofitable, the option is worthless.
    #[test]
    fn unprofitable_in_both_states_has_zero_option_value() {
        let option_value =
            option_to_expand_value(1_000_000.0, 1.1, 0.9, 5_000_000.0, 0.08).unwrap();
        assert!((option_value - 0.0).abs() < TOL, "got {option_value}");
    }
}
