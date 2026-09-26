//! # Concentration Index
//!
//! The Concentration Index (Wagstaff, Paci, van Doorslaer, 1991) is a
//! standard measure of socioeconomic-related inequality in a health
//! variable, ranging from -1 to 1: negative means the health variable is
//! concentrated among the socioeconomically disadvantaged, positive means
//! it is concentrated among the better-off, and zero means no consistent
//! socioeconomic gradient.
//!
//! ## Formula
//!
//! ```text
//! CI = (2 / mean(health_values)) × Cov(health_values, socioeconomic_ranks)
//!
//! Cov(X, Y) = mean(X × Y) − mean(X) × mean(Y)   (population covariance)
//!
//! socioeconomic_ranks: each person's fractional rank in the socioeconomic
//! distribution, in [0, 1] (0 = most disadvantaged, 1 = most advantaged;
//! for grouped/banded data, conventionally the midpoint rank of each group)
//! ```
//!
//! This is the "convenient covariance formula" (O'Donnell, van Doorslaer,
//! Wagstaff, Lindelow, World Bank 2008) — the standard practitioner shortcut
//! for computing the Concentration Index directly from paired observations,
//! without first drawing the concentration curve.
//!
//! ## Why it matters
//!
//! A programme can look effective on aggregate and still deliver its
//! benefit almost entirely to people who were already better off — the
//! Concentration Index turns that suspicion into a single, comparable
//! number rather than a chart someone has to squint at. Because it is
//! computed the same way for any health variable against any socioeconomic
//! ranking, it lets a health system compare the *distributional* fairness
//! of completely different interventions on the same scale, and track
//! whether a specific programme's inequality is widening or narrowing over
//! time.
//!
//! ## Example
//!
//! A self-reported good-health score (1 = worst, 4 = best) observed across
//! four equal-sized socioeconomic quartiles, each represented by its
//! quartile midpoint rank:
//!
//! ```rust
//! use health_economics::concentration_index::concentration_index;
//!
//! let health_values = vec![1.0, 2.0, 3.0, 4.0];
//! let socioeconomic_ranks = vec![0.125, 0.375, 0.625, 0.875];
//!
//! // mean(health) = 2.5
//! // mean(health x rank) = mean([0.125, 0.75, 1.875, 3.5]) = 1.5625
//! // mean(rank) = 0.5
//! // Cov = 1.5625 - 2.5 x 0.5 = 0.3125
//! // CI = 2 x 0.3125 / 2.5 = 0.25
//! let ci = concentration_index(&health_values, &socioeconomic_ranks).unwrap();
//! assert!((ci - 0.25).abs() < 1e-9);
//! ```
//!
//! A positive `0.25` means this health score is concentrated among the
//! socioeconomically advantaged group.
//!
//! ## Software engineering connection
//!
//! This is the same covariance-based inequality measurement used in
//! economics generally (the Gini coefficient's cousin), and it maps onto
//! measuring whether a software product's benefits are concentrated among
//! already-advantaged user segments rather than spread equitably — a direct
//! extension of this crate's [`crate::reach_and_equity`] module (RE-AIM's
//! "reach" dimension) into a formal statistical measure rather than a
//! described gap. Where `reach_and_equity` reports impact per stratum,
//! `concentration_index` compresses the whole distribution into one
//! signed number, suitable for a single tracked KPI across releases.
//!
//! ## Pitfalls
//!
//! - **Sign convention drift**: the sign depends on how both the health
//!   variable and the rank are defined — flipping either flips the sign, so
//!   the convention used must always be stated explicitly alongside any
//!   reported value.
//! - **Boundary ranks instead of midpoint ranks**: grouped or banded
//!   socioeconomic data (e.g. quintiles) requires using each group's
//!   fractional rank at its *midpoint*, not its boundary, or the index is
//!   biased.
//! - **Reading "near zero" as "no inequality"**: a Concentration Index near
//!   zero means "no consistent socioeconomic gradient," not "no inequality"
//!   in an absolute sense — offsetting inequalities in different directions
//!   can cancel out.
//!
//! ## Sources
//!
//! - Wagstaff A, Paci P, van Doorslaer E. "On the measurement of
//!   inequalities in health." Soc Sci Med. 1991;33(5):545-57.
//! - O'Donnell O, van Doorslaer E, Wagstaff A, Lindelow M. "Analyzing Health
//!   Equity Using Household Survey Data." World Bank. 2008 (the standard
//!   practitioner handbook — source of the convenient covariance formula
//!   used here).
//!
//! Topic doc: health-economics-metrics/topics/concentration-index.md

/// Concentration Index: `(2 / mean(health_values)) × Cov(health_values, socioeconomic_ranks)`.
///
/// # Arguments
///
/// * `health_values` — the health variable observed for each person (e.g. a
///   self-reported health score, a cost, or a utilisation count).
/// * `socioeconomic_ranks` — each person's fractional rank in the
///   socioeconomic distribution, in `[0, 1]` (0 = most disadvantaged, 1 =
///   most advantaged), in the same order as `health_values`.
///
/// # Returns
///
/// `Some(concentration_index)`, or `None` if the two slices have different
/// lengths, either is empty, or `mean(health_values) == 0.0` (the index is
/// undefined when the mean is zero).
///
/// # Examples
///
/// ```rust
/// use health_economics::concentration_index::concentration_index;
///
/// let health_values = vec![1.0, 2.0, 3.0, 4.0];
/// let socioeconomic_ranks = vec![0.125, 0.375, 0.625, 0.875];
/// let ci = concentration_index(&health_values, &socioeconomic_ranks).unwrap();
/// assert!((ci - 0.25).abs() < 1e-9);
///
/// // Mismatched lengths are undefined.
/// assert_eq!(concentration_index(&[1.0, 2.0], &[0.5]), None);
/// ```
#[must_use]
pub fn concentration_index(health_values: &[f64], socioeconomic_ranks: &[f64]) -> Option<f64> {
    if health_values.len() != socioeconomic_ranks.len() || health_values.is_empty() {
        return None;
    }

    // Sample sizes in this domain are small enough (survey/cohort counts)
    // that the f64 cast never loses precision in practice.
    #[allow(clippy::cast_precision_loss)]
    let n = health_values.len() as f64;

    let mean_health: f64 = health_values.iter().sum::<f64>() / n;
    if mean_health == 0.0 {
        return None;
    }

    let mean_rank: f64 = socioeconomic_ranks.iter().sum::<f64>() / n;
    let mean_cross: f64 = health_values
        .iter()
        .zip(socioeconomic_ranks.iter())
        .map(|(h, r)| h * r)
        .sum::<f64>()
        / n;
    let covariance = mean_cross - mean_health * mean_rank;

    Some(2.0 * covariance / mean_health)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    // Worked example: health_values = [1,2,3,4], quartile midpoint ranks
    // [0.125, 0.375, 0.625, 0.875] -> CI = 0.25.
    #[test]
    fn worked_example_concentration_index_is_0_25() {
        let health_values = vec![1.0, 2.0, 3.0, 4.0];
        let socioeconomic_ranks = vec![0.125, 0.375, 0.625, 0.875];
        let ci = concentration_index(&health_values, &socioeconomic_ranks).unwrap();
        assert!((ci - 0.25).abs() < TOL, "got {ci}");
    }

    // Doc math intermediate: mean(health x rank) = 1.5625.
    #[test]
    fn worked_example_mean_cross_product_is_1_5625() {
        let health_values = [1.0, 2.0, 3.0, 4.0];
        let socioeconomic_ranks = [0.125, 0.375, 0.625, 0.875];
        let mean_cross: f64 = health_values
            .iter()
            .zip(socioeconomic_ranks.iter())
            .map(|(h, r)| h * r)
            .sum::<f64>()
            / 4.0;
        assert!((mean_cross - 1.5625).abs() < TOL, "got {mean_cross}");
    }

    // A perfectly flat health variable across ranks has zero covariance,
    // so CI = 0 (no socioeconomic gradient), as long as the mean isn't zero.
    #[test]
    fn flat_health_variable_has_zero_concentration_index() {
        let health_values = vec![2.0, 2.0, 2.0, 2.0];
        let socioeconomic_ranks = vec![0.125, 0.375, 0.625, 0.875];
        let ci = concentration_index(&health_values, &socioeconomic_ranks).unwrap();
        assert!((ci - 0.0).abs() < TOL, "got {ci}");
    }

    // Reversing which end of the health scale is "good" flips the sign
    // convention, per the documented pitfall.
    #[test]
    fn reversed_health_scale_flips_sign() {
        let health_values = vec![1.0, 2.0, 3.0, 4.0];
        let reversed_health_values = vec![4.0, 3.0, 2.0, 1.0];
        let socioeconomic_ranks = vec![0.125, 0.375, 0.625, 0.875];
        let ci = concentration_index(&health_values, &socioeconomic_ranks).unwrap();
        let ci_reversed =
            concentration_index(&reversed_health_values, &socioeconomic_ranks).unwrap();
        assert!((ci + ci_reversed).abs() < TOL, "got {ci} and {ci_reversed}");
    }

    // Edge case: mismatched lengths are undefined.
    #[test]
    fn mismatched_lengths_return_none() {
        assert!(concentration_index(&[1.0, 2.0], &[0.5]).is_none());
    }

    // Edge case: empty inputs are undefined.
    #[test]
    fn empty_inputs_return_none() {
        assert!(concentration_index(&[], &[]).is_none());
    }

    // Edge case: a zero mean health value leaves the index undefined.
    #[test]
    fn zero_mean_health_returns_none() {
        let health_values = vec![-1.0, 1.0];
        let socioeconomic_ranks = vec![0.25, 0.75];
        assert!(concentration_index(&health_values, &socioeconomic_ranks).is_none());
    }
}
