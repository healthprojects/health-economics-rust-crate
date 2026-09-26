//! # Number Needed to Screen (NNS)
//!
//! NNS is the number of people who must be screened — not merely treated —
//! to prevent **one** adverse outcome over a defined follow-up period, given
//! the population's baseline risk and the relative risk reduction that
//! early detection and treatment achieve. It is NNT's
//! screening-programme-level analogue: NNT asks how many must be *treated*
//! to prevent one outcome; NNS asks how many must go through the whole
//! screen-and-then-treat pathway to get there.
//!
//! ## Formula
//!
//! ```text
//! NNS = 1 / (baseline_risk × relative_risk_reduction)
//!
//! baseline_risk           = probability of the outcome in the screened
//!                            population over the follow-up period (0–1)
//! relative_risk_reduction = proportional risk reduction achieved by
//!                            screening-enabled early treatment (0–1)
//!
//! Program cost per outcome prevented = NNS × cost_per_screen
//! ```
//!
//! ## Why it matters
//!
//! Rembold introduced NNS in 1998 so screening programmes could be compared
//! on the same footing as treatments, because a screening test's headline
//! relative risk reduction hides two things a treatment's does not: the
//! baseline risk of the population actually invited to screen, and the fact
//! that everyone screened bears the test's cost and false-positive burden,
//! not just the minority who go on to benefit. A screening programme with an
//! impressive relative risk reduction in a low-baseline-risk population can
//! still have an NNS in the thousands, at which point programme cost per
//! outcome prevented becomes the real question.
//!
//! ## Example
//!
//! A screening programme's target population has a 2% baseline event risk
//! over the study period, and early detection achieves a 25% relative risk
//! reduction.
//!
//! ```rust
//! use health_economics::number_needed_to_screen::{
//!     number_needed_to_screen, screening_program_cost_per_outcome_prevented,
//! };
//!
//! // NNS = 1 / (0.02 × 0.25) = 1 / 0.005 = 200.
//! let nns = number_needed_to_screen(0.02, 0.25).unwrap();
//! assert!((nns - 200.0).abs() < 1e-9);
//!
//! // At £50 per screen, programme cost per outcome prevented = 200 × 50 = £10,000.
//! let cost = screening_program_cost_per_outcome_prevented(nns, 50.0);
//! assert!((cost - 10_000.0).abs() < 1e-9);
//! ```
//!
//! ## Software engineering connection
//!
//! NNS is "how many users, events, or requests must run through a detection
//! or triage flow to catch one true positive worth acting on" — directly
//! relevant to alert-based monitoring and triage systems, where a
//! low-prevalence target condition inflates NNS the same way it collapses
//! positive predictive value (see [`crate::clinical_ai_evaluation`]). A
//! monitoring rule that must process 200 events per real catch is only
//! worth running if the catch is worth at least 200 times the per-event
//! triage cost — the identical arithmetic as the worked example above.
//!
//! ## Pitfalls
//!
//! - **Ignoring baseline risk dependence**: the same screening test or
//!   programme has a very different NNS — and cost-effectiveness — in a
//!   high-risk population versus a low-risk one. Never quote an NNS without
//!   stating the population it was computed for.
//! - **Counting the wrong denominator**: NNS counts people *screened*, not
//!   people who test positive or start treatment — it already embeds the
//!   whole funnel's effectiveness, so it should never be compared to a
//!   metric counted over positives only.
//! - **Comparing across follow-up periods**: a shorter follow-up period
//!   generally inflates NNS, because fewer events are observed in the
//!   window. NNS figures are only comparable when computed over the same
//!   follow-up duration.
//!
//! ## Sources
//!
//! - Rembold CM. "Number needed to screen: development of a statistic for
//!   disease screening." BMJ. 1998;317(7154):307-12.
//!
//! Topic doc: health-economics-metrics/topics/number-needed-to-screen.md

/// NNS = 1 / (`baseline_risk` × `relative_risk_reduction`).
///
/// The number of people who must be screened to prevent one adverse
/// outcome over the follow-up period the inputs were measured over.
///
/// # Arguments
///
/// * `baseline_risk` — probability of the outcome in the screened
///   population over the follow-up period (0–1).
/// * `relative_risk_reduction` — proportional risk reduction achieved by
///   screening-enabled early treatment (0–1).
///
/// # Returns
///
/// `Some(NNS)`, or `None` when `baseline_risk × relative_risk_reduction` is
/// zero (no effect — no finite number of people screened yields one
/// prevented outcome).
///
/// # Examples
///
/// ```rust
/// use health_economics::number_needed_to_screen::number_needed_to_screen;
///
/// // 2% baseline risk, 25% relative risk reduction → NNS = 200.
/// let nns = number_needed_to_screen(0.02, 0.25).unwrap();
/// assert!((nns - 200.0).abs() < 1e-9);
///
/// assert!(number_needed_to_screen(0.0, 0.25).is_none());
/// ```
#[must_use]
pub fn number_needed_to_screen(baseline_risk: f64, relative_risk_reduction: f64) -> Option<f64> {
    let denominator = baseline_risk * relative_risk_reduction;
    if denominator == 0.0 {
        None
    } else {
        Some(1.0 / denominator)
    }
}

/// Programme cost per outcome prevented: `NNS × cost_per_screen`.
///
/// Everyone screened pays the screening cost, but only one in NNS is
/// prevented an outcome — so preventing one outcome costs the whole
/// screened cohort's screening bill.
///
/// # Arguments
///
/// * `nns` — the number needed to screen.
/// * `cost_per_screen` — cost of screening one person (currency).
///
/// # Returns
///
/// The cost of preventing one outcome (`nns × cost_per_screen`).
///
/// # Examples
///
/// ```rust
/// use health_economics::number_needed_to_screen::screening_program_cost_per_outcome_prevented;
///
/// // NNS 200 at £50/screen → cost per outcome prevented = £10,000.
/// let cost = screening_program_cost_per_outcome_prevented(200.0, 50.0);
/// assert!((cost - 10_000.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn screening_program_cost_per_outcome_prevented(nns: f64, cost_per_screen: f64) -> f64 {
    nns * cost_per_screen
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// Worked example: 2% baseline risk, 25% RRR → NNS = 200.
    #[test]
    fn nns_worked_example_is_200() {
        let nns = number_needed_to_screen(0.02, 0.25).unwrap();
        assert!((nns - 200.0).abs() < TOL);
    }

    /// NNS 200 at £50/screen → cost per outcome prevented = £10,000.
    #[test]
    fn cost_per_outcome_prevented_worked_example_is_10000() {
        let cost = screening_program_cost_per_outcome_prevented(200.0, 50.0);
        assert!((cost - 10_000.0).abs() < TOL);
    }

    /// Zero baseline risk or zero relative risk reduction is undefined.
    #[test]
    fn zero_effect_edge_cases_are_none() {
        assert!(number_needed_to_screen(0.0, 0.25).is_none());
        assert!(number_needed_to_screen(0.02, 0.0).is_none());
    }

    /// A lower baseline risk inflates NNS for the same relative risk reduction.
    #[test]
    fn lower_baseline_risk_gives_higher_nns() {
        let high_risk_nns = number_needed_to_screen(0.02, 0.25).unwrap();
        let low_risk_nns = number_needed_to_screen(0.01, 0.25).unwrap();
        assert!(low_risk_nns > high_risk_nns);
    }
}
