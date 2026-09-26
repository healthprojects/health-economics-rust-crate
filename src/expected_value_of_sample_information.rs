//! # Expected Value of Sample Information (EVSI)
//!
//! EVSI is the value of a *specific proposed study* — a given design, a given
//! sample size — before it is run, as opposed to expected value of perfect
//! information (EVPI), which prices eliminating all uncertainty outright.
//! EVSI answers the question a research funder actually faces: "is *this*
//! trial, at *this* size, worth its cost?"
//!
//! Because a finite sample can only partially resolve uncertainty, EVSI is
//! always less than or equal to EVPI — a calculation that produces
//! `EVSI > EVPI` is a modeling bug, not a real result.
//!
//! ## Formula
//!
//! ```text
//! General:
//! EVSI(n) = E_data[ max_d E_θ|data[NB(d,θ)] ]  −  max_d E_θ[NB(d,θ)]
//!   (nested expectation: outer over possible study results, inner over the
//!   posterior belief about θ after seeing that result — usually estimated by
//!   nested Monte Carlo / Bayesian updating over probabilistic sensitivity
//!   analysis draws)
//!
//! Closed-form normal approximation (single uncertain parameter, conjugate
//! normal-normal model — a standard shortcut, not exact for every model):
//! EVSI(n) = EVPI × n / (n + n0)
//!
//! n  = proposed study's sample size
//! n0 = "prior-equivalent sample size" — the size of an imaginary sample that
//!      would carry the same information as the current prior, derived from
//!      the ratio of data variance to prior variance
//! ENBS(n) = EVSI(n) − Cost(n)
//! Population EVSI = per-decision EVSI × decisions affected
//! ```
//!
//! ## Why it matters
//!
//! EVPI tells you the ceiling on what any research could be worth; it never
//! tells you whether the trial in front of you clears the bar. A national
//! research funder choosing between a 50-patient pilot and a 500-patient
//! definitive trial needs to know how much *each specific design* is worth,
//! not just the value of omniscience. EVSI supplies that number, and because
//! it scales with sample size, it lets a funder find the sample size that
//! maximizes expected net benefit rather than guessing. The closed-form
//! normal approximation trades the general method's computational cost for a
//! single ratio — valid when the uncertain parameter and the data are
//! (approximately) normally distributed and conjugate.
//!
//! ## Example
//!
//! Building on the EVPI worked example — rolling out an AI documentation
//! assistant to 5,000 clinicians, where EVPI was found to be £1.2M —
//! expressed here in whole pounds: EVPI = £1,200,000. A proposed pilot of 50
//! clinicians has a prior-equivalent sample size of 75, derived from the
//! variance ratio of the prior belief versus the pilot's measurement
//! precision.
//!
//! ```rust
//! use health_economics::expected_value_of_sample_information::{
//!     evsi_normal_approximation, expected_net_benefit_of_sampling, population_evsi,
//! };
//!
//! // EVSI(50) = 1,200,000 × 50 / (50 + 75) = 1,200,000 × 0.4 = £480,000.
//! let evsi = evsi_normal_approximation(1_200_000.0, 50.0, 75.0).unwrap();
//! assert!((evsi - 480_000.0).abs() < 1e-9);
//!
//! // The pilot costs £120,000: ENBS = 480,000 − 120,000 = £360,000 → fund it.
//! let enbs = expected_net_benefit_of_sampling(evsi, 120_000.0);
//! assert!((enbs - 360_000.0).abs() < 1e-9);
//!
//! // The same procurement decision recurs across 3 regional trusts.
//! let population = population_evsi(evsi, 3.0);
//! assert!((population - 1_440_000.0).abs() < 1e-9);
//! ```
//!
//! ## Software engineering connection
//!
//! - **Sample-size-as-investment decision.** A 50-user beta and a
//!   5,000-user staged rollout are different "studies" with different EVSIs
//!   and different costs — EVSI lets you compare them on the same basis
//!   instead of defaulting to "more data is always better."
//! - **ENBS, not EVSI alone, is the commissioning test.** A study with high
//!   EVSI but a cost that eats most of it is a weak proposal; the decision
//!   rule is expected net benefit of sampling, exactly as a business case
//!   nets benefit against cost rather than reporting benefit alone.
//! - **Diminishing returns are explicit.** Because EVSI(n) rises with
//!   `n / (n + n0)`, doubling a pilot's size never doubles its value — a
//!   formal version of the engineering instinct that a bigger experiment has
//!   diminishing marginal information value.
//!
//! ## Pitfalls
//!
//! - **Applying the normal approximation outside its assumptions.** It only
//!   holds for roughly-conjugate, single-parameter uncertainty; a genuinely
//!   nonlinear or multi-parameter decision model needs full nested Monte
//!   Carlo, not this shortcut.
//! - **Comparing EVSI to cash cost alone.** EVSI must be weighed against the
//!   *full* cost of the study, including its own decision-delay cost, not
//!   just the study's invoice.
//! - **Treating `EVSI > EVPI` as a real finding.** EVSI can never exceed EVPI
//!   by construction; a calculation that produces this is a modeling bug, not
//!   a discovery.
//!
//! ## Sources
//!
//! - Ades AE, Lu G, Claxton K. "Expected value of sample information
//!   calculations in medical decision modeling." Medical Decision Making
//!   2004;24(2):207-27.
//! - Willan AR, Pinto EM. "The value of information and optimal clinical
//!   trial design." Statistics in Medicine 2005;24(12):1791-806.
//! - Strong M, Oakley JE. "When is a model-based value of information
//!   analysis feasible?" Medical Decision Making 2014.
//!
//! Topic doc: health-economics-metrics/topics/expected-value-of-sample-information.md

/// EVSI via the closed-form normal approximation: `EVPI × n / (n + n0)`.
///
/// Valid for a single uncertain parameter under a (roughly) conjugate
/// normal-normal model — a standard shortcut, not exact for every decision
/// model. `n0`, the "prior-equivalent sample size," is derived from the ratio
/// of data variance to prior variance.
///
/// # Arguments
///
/// * `evpi` — expected value of perfect information for the same decision,
///   in currency units.
/// * `sample_size` — proposed study's sample size `n`.
/// * `prior_equivalent_sample_size` — `n0`, the imaginary sample size that
///   would carry the same information as the current prior.
///
/// # Returns
///
/// `Some(evpi * sample_size / (sample_size + prior_equivalent_sample_size))`,
/// or `None` if `sample_size + prior_equivalent_sample_size` is `0.0` (the
/// ratio is undefined).
///
/// # Examples
///
/// ```rust
/// use health_economics::expected_value_of_sample_information::evsi_normal_approximation;
///
/// // EVSI(50) = 1,200,000 × 50 / (50 + 75) = £480,000.
/// let evsi = evsi_normal_approximation(1_200_000.0, 50.0, 75.0).unwrap();
/// assert!((evsi - 480_000.0).abs() < 1e-9);
///
/// // A zero total sample size is undefined.
/// assert!(evsi_normal_approximation(1_200_000.0, 0.0, 0.0).is_none());
/// ```
#[must_use]
pub fn evsi_normal_approximation(
    evpi: f64,
    sample_size: f64,
    prior_equivalent_sample_size: f64,
) -> Option<f64> {
    let denominator = sample_size + prior_equivalent_sample_size;
    if denominator == 0.0 {
        None
    } else {
        Some(evpi * sample_size / denominator)
    }
}

/// Expected net benefit of sampling: `EVSI − Cost(n)`.
///
/// The actual commissioning test for a proposed study — not EVSI alone, since
/// a study with high EVSI but a cost that eats most of it is a weak proposal.
///
/// # Arguments
///
/// * `evsi` — expected value of sample information for the proposed study, in
///   currency units.
/// * `study_cost` — the study's full cost, in the same currency units.
///
/// # Returns
///
/// `evsi − study_cost`; positive means the study is worth funding.
///
/// # Examples
///
/// ```rust
/// use health_economics::expected_value_of_sample_information::expected_net_benefit_of_sampling;
///
/// // EVSI of £480,000 minus a £120,000 pilot cost: ENBS = £360,000.
/// let enbs = expected_net_benefit_of_sampling(480_000.0, 120_000.0);
/// assert!((enbs - 360_000.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn expected_net_benefit_of_sampling(evsi: f64, study_cost: f64) -> f64 {
    evsi - study_cost
}

/// Population EVSI: per-decision EVSI scaled by the number of decisions the
/// study's information would affect.
///
/// # Arguments
///
/// * `per_decision_evsi` — EVSI for one decision instance, in currency units.
/// * `decisions_affected` — number of decisions the study informs (e.g. the
///   number of similar procurement decisions across regional trusts).
///
/// # Returns
///
/// Population EVSI in the same currency units.
///
/// # Examples
///
/// ```rust
/// use health_economics::expected_value_of_sample_information::population_evsi;
///
/// // £480,000 per decision across 3 regional trusts = £1,440,000.
/// let population = population_evsi(480_000.0, 3.0);
/// assert!((population - 1_440_000.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn population_evsi(per_decision_evsi: f64, decisions_affected: f64) -> f64 {
    per_decision_evsi * decisions_affected
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// EVSI(50) = 1,200,000 × 50 / (50 + 75) = 1,200,000 × 0.4 = £480,000.
    #[test]
    fn evsi_normal_approximation_is_480_000() {
        // Worked example: "EVSI(50) = 1,200,000 × 50 / 125 = £480,000".
        let evsi = evsi_normal_approximation(1_200_000.0, 50.0, 75.0).unwrap();
        assert!((evsi - 480_000.0).abs() < TOL, "got {evsi}");
    }

    /// The pilot costs £120,000: ENBS = 480,000 − 120,000 = £360,000.
    #[test]
    fn enbs_is_360_000() {
        // Worked example: "ENBS = EVSI − Cost = 480,000 − 120,000 = £360,000".
        let enbs = expected_net_benefit_of_sampling(480_000.0, 120_000.0);
        assert!((enbs - 360_000.0).abs() < TOL, "got {enbs}");
    }

    /// The procurement decision recurs across 3 regional trusts: £1,440,000.
    #[test]
    fn population_evsi_is_1_440_000() {
        // Worked example: "Population EVSI = 480,000 × 3 = £1,440,000".
        let population = population_evsi(480_000.0, 3.0);
        assert!((population - 1_440_000.0).abs() < TOL, "got {population}");
    }

    // Doc rule: "EVSI can never exceed EVPI" — the normal approximation
    // guarantees this since n / (n + n0) is always in [0, 1].
    #[test]
    fn evsi_never_exceeds_evpi() {
        let evsi = evsi_normal_approximation(1_200_000.0, 50.0, 75.0).unwrap();
        assert!(evsi <= 1_200_000.0);
    }

    // Edge case: a zero total sample size makes the ratio undefined.
    #[test]
    fn zero_total_sample_size_is_undefined() {
        assert!(evsi_normal_approximation(1_200_000.0, 0.0, 0.0).is_none());
    }
}
