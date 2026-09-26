//! # Population Attributable Fraction (PAF)
//!
//! PAF is the proportion of a disease or outcome's burden in a population
//! that is attributable to a specific risk-factor exposure — the share that
//! would disappear if the exposure were removed entirely. It converts "this
//! risk factor doubles your odds" into a population-level number a
//! commissioner can plan around: how many cases, and how much cost, a given
//! exposure is really worth chasing.
//!
//! ## Formula
//!
//! ```text
//! PAF = prevalence_exposed × (relative_risk − 1) / (1 + prevalence_exposed × (relative_risk − 1))
//!
//! prevalence_exposed = fraction of the population exposed to the risk factor (0–1)
//! relative_risk      = risk of the outcome in exposed vs. unexposed people (e.g. 2.5 = 2.5×)
//!
//! Cases attributable = total_cases × PAF
//! ```
//!
//! ## Why it matters
//!
//! Levin introduced PAF in 1953 to answer a narrow, concrete question: if
//! nobody smoked, how much lung cancer would disappear? The same arithmetic
//! now sizes national prevention planning, because a relative risk alone
//! says nothing about population impact — a risk factor can double the odds
//! of a rare event and barely move total disease burden, or raise a common
//! event's odds only slightly and still account for a huge share of cases.
//! PAF is what turns "risk factor X is dangerous" into "removing risk factor
//! X would prevent this many cases per year."
//!
//! ## Example
//!
//! A risk factor is present in 30% of a population and raises the outcome's
//! risk 2.5-fold.
//!
//! ```rust
//! use health_economics::population_attributable_fraction::{
//!     cases_attributable, paf_from_relative_risk,
//! };
//!
//! // PAF = 0.3 × 1.5 / (1 + 0.3 × 1.5) = 0.45 / 1.45 ≈ 0.310344827586.
//! let paf = paf_from_relative_risk(0.3, 2.5).unwrap();
//! assert!((paf - 0.310_344_827_586_206_9).abs() < 1e-9);
//!
//! // With 1,000 cases/year, roughly 310 are attributable to the exposure.
//! let cases = cases_attributable(1_000.0, paf);
//! assert!((cases - 310.344_827_586_206_9).abs() < 1e-6);
//! ```
//!
//! ## Software engineering connection
//!
//! PAF is the epidemiological version of "how much of our incident volume is
//! attributable to this one root cause?" — the same question teams ask when
//! sizing a specific class of deploy or dependency against total production
//! incidents, rather than treating every incident as equally worth fixing
//! the same way. A root-cause category present in a large share of deploys
//! with only a modest relative risk of causing an incident can outrank a
//! rare, high-relative-risk category for where to spend engineering effort
//! first — exactly the PAF insight, translated.
//!
//! ## Pitfalls
//!
//! - **Summing PAFs across risk factors**: PAFs for multiple factors
//!   affecting the same outcome do not add to 100% — they can exceed it in
//!   total, because factors interact and share causal pathways. Treat each
//!   PAF as "if this factor alone were removed," never as a partition of
//!   total risk.
//! - **Transplanting a relative risk across populations**: a relative risk
//!   estimated in one population (different baseline exposure prevalence,
//!   different confounders) computes a misleading PAF when applied to a
//!   different population's exposure prevalence.
//! - **Confusing PAF with attributable risk in the exposed**: PAF is
//!   population-level and depends on exposure prevalence; attributable risk
//!   in the exposed is individual-level and does not. They answer different
//!   questions.
//!
//! ## Sources
//!
//! - Levin ML. "The occurrence of lung cancer in man." Acta Unio Int Contra
//!   Cancrum. 1953;9(3):531-41.
//! - Rockhill B, Newman B, Weinberg C. "Use and misuse of population
//!   attributable fractions." Am J Public Health. 1998;88(1):15-9.
//!
//! Topic doc: health-economics-metrics/topics/population-attributable-fraction.md

/// PAF from exposure prevalence and relative risk (Levin, 1953).
///
/// The proportion of the outcome's burden attributable to the exposure —
/// the share that would be eliminated if the exposure were removed
/// entirely.
///
/// # Arguments
///
/// * `prevalence_exposed` — fraction of the population exposed to the risk
///   factor (0–1).
/// * `relative_risk` — risk of the outcome in exposed vs. unexposed people
///   (e.g. 2.5 for a 2.5× risk).
///
/// # Returns
///
/// `Some(PAF as a fraction)`, or `None` when the denominator
/// `1 + prevalence_exposed × (relative_risk − 1)` is zero.
///
/// # Examples
///
/// ```rust
/// use health_economics::population_attributable_fraction::paf_from_relative_risk;
///
/// // 30% exposure prevalence, relative risk 2.5 → PAF ≈ 0.3103 (31.0%).
/// let paf = paf_from_relative_risk(0.3, 2.5).unwrap();
/// assert!((paf - 0.310_344_827_586_206_9).abs() < 1e-9);
/// ```
#[must_use]
pub fn paf_from_relative_risk(prevalence_exposed: f64, relative_risk: f64) -> Option<f64> {
    let excess = prevalence_exposed * (relative_risk - 1.0);
    let denominator = 1.0 + excess;
    if denominator == 0.0 {
        None
    } else {
        Some(excess / denominator)
    }
}

/// Cases attributable to the exposure: `total_cases × PAF`.
///
/// # Arguments
///
/// * `total_cases` — total cases of the outcome in the population over the
///   period of interest.
/// * `paf` — the population attributable fraction (from
///   [`paf_from_relative_risk`]).
///
/// # Returns
///
/// The number of cases attributable to the exposure.
///
/// # Examples
///
/// ```rust
/// use health_economics::population_attributable_fraction::cases_attributable;
///
/// // 1,000 cases/year at PAF ≈ 0.3103 → roughly 310 attributable cases.
/// let cases = cases_attributable(1_000.0, 0.310_344_827_586_206_9);
/// assert!((cases - 310.344_827_586_206_9).abs() < 1e-6);
/// ```
#[must_use]
pub fn cases_attributable(total_cases: f64, paf: f64) -> f64 {
    total_cases * paf
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// Worked example: 30% exposure prevalence, RR 2.5 → PAF ≈ 0.310344827586.
    #[test]
    fn paf_worked_example_is_about_0_3103() {
        let paf = paf_from_relative_risk(0.3, 2.5).unwrap();
        assert!((paf - 0.310_344_827_586_206_9).abs() < TOL);
    }

    /// 1,000 cases/year at the worked-example PAF → roughly 310 attributable cases.
    #[test]
    fn cases_attributable_worked_example_is_about_310() {
        let paf = paf_from_relative_risk(0.3, 2.5).unwrap();
        let cases = cases_attributable(1_000.0, paf);
        assert!((cases - 310.344_827_586_206_9).abs() < 1e-6);
    }

    /// No exposure means no attributable burden.
    #[test]
    fn zero_prevalence_gives_zero_paf() {
        let paf = paf_from_relative_risk(0.0, 2.5).unwrap();
        assert!((paf - 0.0).abs() < TOL);
    }

    /// Relative risk of 1 (no association) means no attributable burden.
    #[test]
    fn relative_risk_of_one_gives_zero_paf() {
        let paf = paf_from_relative_risk(0.5, 1.0).unwrap();
        assert!((paf - 0.0).abs() < TOL);
    }

    /// Denominator of zero (a protective exposure that exactly offsets the
    /// baseline) has no defined PAF.
    #[test]
    fn zero_denominator_is_none() {
        // prevalence_exposed × (relative_risk − 1) = −1 ⇒ denominator = 0.
        assert!(paf_from_relative_risk(1.0, 0.0).is_none());
    }
}
