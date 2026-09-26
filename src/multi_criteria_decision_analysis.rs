//! # Multi-Criteria Decision Analysis (MCDA)
//!
//! Multi-criteria decision analysis (MCDA) is a weighted-sum scoring model
//! used in health technology assessment when a single ICER/willingness-to-pay
//! threshold doesn't capture everything a decision-maker cares about:
//! equity, unmet need, innovation, budget impact, disease severity. Each
//! criterion gets a weight reflecting its importance (elicited from
//! stakeholders, weights summing to 1) and each option gets a normalized
//! score per criterion (typically 0–1); the overall score is the weighted
//! sum — the same mathematical shape as a software vendor-selection
//! scorecard.
//!
//! ## Formula
//!
//! ```text
//! MCDA score = Σ_i (weight_i × score_i)
//!
//! weights should sum to 1 (elicited via stakeholder methods such as swing
//! weighting or the Analytic Hierarchy Process)
//! ```
//!
//! ## Why it matters
//!
//! MCDA is used in frameworks like EVIDEM, and by some HTA bodies for
//! orphan/rare-disease appraisals where a strict cost-per-QALY threshold
//! approach is considered too narrow. The ISPOR MCDA Emerging Good Practices
//! Task Force formalized good-practice guidance for eliciting weights and
//! scores defensibly, precisely because an informally weighted decision is
//! easy to construct and easy to game.
//!
//! ## Example
//!
//! An HTA committee scores a digital therapeutic on four criteria: clinical
//! benefit, cost impact, disease severity/unmet need, and innovation.
//!
//! ```rust
//! use health_economics::multi_criteria_decision_analysis::{
//!     mcda_score, weights_sum_to_one, WeightedCriterion,
//! };
//!
//! let criteria = vec![
//!     WeightedCriterion { weight: 0.4, score: 0.8 }, // clinical benefit
//!     WeightedCriterion { weight: 0.3, score: 0.5 }, // cost impact
//!     WeightedCriterion { weight: 0.2, score: 0.9 }, // disease severity / unmet need
//!     WeightedCriterion { weight: 0.1, score: 0.6 }, // innovation
//! ];
//!
//! // 0.4×0.8 + 0.3×0.5 + 0.2×0.9 + 0.1×0.6 = 0.32 + 0.15 + 0.18 + 0.06 = 0.71.
//! let score = mcda_score(&criteria);
//! assert!((score - 0.71).abs() < 1e-9);
//!
//! // 0.4 + 0.3 + 0.2 + 0.1 = 1.0.
//! assert!(weights_sum_to_one(&criteria));
//! ```
//!
//! The committee compares 0.71 against a pre-agreed threshold, or ranks it
//! against competing technologies scored the same way.
//!
//! ## Software engineering connection
//!
//! This is exactly the same math as a weighted vendor-selection scorecard,
//! an RFP evaluation matrix, or a feature-prioritization scoring model — see
//! `build_vs_buy`, a classic weighted-scorecard use case in software
//! procurement. It's also worth contrasting with `wsjf_and_cd3`: WSJF/CD3 is
//! a *ratio*-based prioritization method (cost of delay divided by job size
//! or duration), whereas MCDA is a weighted *sum*. MCDA and WSJF/CD3 are two
//! structurally different answers to "how do we rank competing options."
//!
//! ## Pitfalls
//!
//! - **Weight elicitation bias**: whoever sets the weights effectively
//!   predetermines the ranking, so a "formula" can launder a political or
//!   commercial decision as an objective calculation.
//! - **Double counting a criterion already captured elsewhere**: scoring
//!   "cost-effectiveness" as one criterion while *also* separately scoring
//!   "cost impact" over-weights money relative to the other criteria.
//! - **False precision**: a two-decimal weighted score implies more rigor
//!   than the underlying 0–10 stakeholder ratings actually support, and
//!   inter-rater variability in those ratings is often not reported at all.
//!
//! ## Sources
//!
//! - Thokala P, Devlin N, Marsh K, et al. "Multiple Criteria Decision
//!   Analysis for Health Care Decision Making — An Introduction: Report 1 of
//!   the ISPOR MCDA Emerging Good Practices Task Force." Value Health.
//!   2016;19(1):1-13.
//! - Goetghebeur MM, Wagner M, Khoury H, et al. "Evidence and Value: Impact
//!   on `DEcisionMaking` — the EVIDEM framework and potential applications."
//!   BMC Health Serv Res. 2008;8:270.
//!
//! Topic doc: health-economics-metrics/topics/multi-criteria-decision-analysis.md

/// One criterion's contribution to an MCDA score: its relative importance
/// and an option's normalized performance on it.
pub struct WeightedCriterion {
    /// The criterion's relative importance, elicited from stakeholders
    /// (e.g. via swing weighting or the Analytic Hierarchy Process).
    /// Weights across all criteria in a scoring exercise should sum to 1;
    /// see [`weights_sum_to_one`].
    pub weight: f64,
    /// The option's normalized performance on this criterion, typically in
    /// the range 0–1 (0 = worst, 1 = best).
    pub score: f64,
}

/// MCDA score: the weighted sum of an option's scores across all criteria.
///
/// `Σ_i (weight_i × score_i)`.
///
/// # Arguments
///
/// * `criteria` — the criteria being scored, each with its weight and the
///   option's normalized score on it.
///
/// # Returns
///
/// The overall weighted-sum MCDA score; `0.0` for an empty slice.
///
/// # Examples
///
/// ```rust
/// use health_economics::multi_criteria_decision_analysis::{mcda_score, WeightedCriterion};
///
/// let criteria = vec![
///     WeightedCriterion { weight: 0.4, score: 0.8 },
///     WeightedCriterion { weight: 0.3, score: 0.5 },
///     WeightedCriterion { weight: 0.2, score: 0.9 },
///     WeightedCriterion { weight: 0.1, score: 0.6 },
/// ];
///
/// // 0.32 + 0.15 + 0.18 + 0.06 = 0.71.
/// let score = mcda_score(&criteria);
/// assert!((score - 0.71).abs() < 1e-9);
/// ```
#[must_use]
pub fn mcda_score(criteria: &[WeightedCriterion]) -> f64 {
    criteria.iter().map(|c| c.weight * c.score).sum()
}

/// Validity check: do the criteria weights sum to 1 (within a small
/// tolerance)?
///
/// A stakeholder-elicited weight set (swing weighting, the Analytic
/// Hierarchy Process, or similar) should be normalized so its weights sum to
/// 1; this is a sanity check before trusting an [`mcda_score`] result as a
/// genuine weighted average.
///
/// # Arguments
///
/// * `criteria` — the criteria whose weights should sum to 1.
///
/// # Returns
///
/// `true` when the weights sum to 1 within `1e-9`.
///
/// # Examples
///
/// ```rust
/// use health_economics::multi_criteria_decision_analysis::{weights_sum_to_one, WeightedCriterion};
///
/// let criteria = vec![
///     WeightedCriterion { weight: 0.4, score: 0.8 },
///     WeightedCriterion { weight: 0.3, score: 0.5 },
///     WeightedCriterion { weight: 0.2, score: 0.9 },
///     WeightedCriterion { weight: 0.1, score: 0.6 },
/// ];
///
/// // 0.4 + 0.3 + 0.2 + 0.1 = 1.0.
/// assert!(weights_sum_to_one(&criteria));
/// ```
#[must_use]
pub fn weights_sum_to_one(criteria: &[WeightedCriterion]) -> bool {
    (criteria.iter().map(|c| c.weight).sum::<f64>() - 1.0).abs() < 1e-9
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    fn digital_therapeutic_criteria() -> Vec<WeightedCriterion> {
        vec![
            WeightedCriterion { weight: 0.4, score: 0.8 }, // clinical benefit
            WeightedCriterion { weight: 0.3, score: 0.5 }, // cost impact
            WeightedCriterion { weight: 0.2, score: 0.9 }, // disease severity / unmet need
            WeightedCriterion { weight: 0.1, score: 0.6 }, // innovation
        ]
    }

    /// 0.4×0.8 + 0.3×0.5 + 0.2×0.9 + 0.1×0.6 = 0.32 + 0.15 + 0.18 + 0.06 = 0.71.
    #[test]
    fn digital_therapeutic_scores_0_71() {
        let score = mcda_score(&digital_therapeutic_criteria());
        assert!((score - 0.71).abs() < TOL);
    }

    /// 0.4 + 0.3 + 0.2 + 0.1 = 1.0.
    #[test]
    fn digital_therapeutic_weights_sum_to_one() {
        assert!(weights_sum_to_one(&digital_therapeutic_criteria()));
    }

    /// A weight set that does not sum to 1 fails the validity check.
    #[test]
    fn mis_specified_weights_fail_validity_check() {
        let criteria = vec![
            WeightedCriterion { weight: 0.5, score: 0.8 },
            WeightedCriterion { weight: 0.3, score: 0.5 },
        ];
        assert!(!weights_sum_to_one(&criteria));
    }

    /// An empty criteria slice scores zero.
    #[test]
    fn empty_criteria_scores_zero() {
        let score = mcda_score(&[]);
        assert!((score - 0.0).abs() < TOL);
    }
}
