//! # Time Trade-Off (TTO) Utility Elicitation
//!
//! TTO (Torrance, 1976) is a standard method for eliciting a health-state
//! utility value directly from a respondent, rather than inventing one. For
//! a state considered *better than death*, the respondent is asked how many
//! years `X` in full health they would consider equivalent to `T` years in
//! the impaired state (`X < T`); utility is `X / T`.
//!
//! For a state considered *worse than death*, the standard formulation
//! breaks down, so an extended TTO applies (Patrick et al., 1994): the
//! respondent trades `a` years of a total `T`-year remaining life for
//! immediate death — preferring `T − a` years in full health followed by
//! death over `T` years in the worse-than-death state. The resulting utility
//! is negative, anchored so that death = 0.
//!
//! ## Formula
//!
//! ```text
//! Standard TTO (state better than death):
//!   utility = time_in_full_health / time_in_impaired_state
//!
//! Extended TTO (state worse than death):
//!   utility = -time_traded_for_death / (total_duration - time_traded_for_death)
//! ```
//!
//! Legend:
//! - `time_in_full_health` — years `X` in full health judged equivalent to
//!   the impaired-state duration.
//! - `time_in_impaired_state` — years `T` spent in the impaired state.
//! - `time_traded_for_death` — years `a` of remaining life the respondent
//!   would trade for immediate death, in the worse-than-death formulation.
//! - `total_duration` — total remaining years `T` being valued.
//!
//! ## Why it matters
//!
//! TTO is the elicitation method underneath the value sets used elsewhere in
//! this crate: [`crate::eq_5d`]'s country-specific value sets, and therefore
//! every [`crate::quality_adjusted_life_year`] calculation that consumes
//! them, are ultimately built from TTO (or a related choice-based) surveys
//! of the general public. A software engineer who treats a utility weight as
//! a given input misses that the number itself required a validated
//! elicitation protocol to produce — see [EQ-5D](../eq-5d/) for how those
//! value sets are used downstream.
//!
//! ## Example
//!
//! A respondent is in an impaired state for `10.0` years and is indifferent
//! with `7.0` years in full health:
//!
//! ```rust
//! use health_economics::time_trade_off_utility::{
//!     time_trade_off_utility, worse_than_death_utility,
//! };
//!
//! // Standard TTO: 7 years full health ~ 10 years impaired → utility 0.7.
//! let utility = time_trade_off_utility(7.0, 10.0).unwrap();
//! assert!((utility - 0.7).abs() < 1e-9);
//!
//! // Worse than death: over a 10-year remaining life, the respondent would
//! // trade 2 years for immediate death (prefers 8 years full health then
//! // death over 10 years in the state) → utility = -2/8 = -0.25.
//! let wtd = worse_than_death_utility(2.0, 10.0).unwrap();
//! assert!((wtd - (-0.25)).abs() < 1e-9);
//! ```
//!
//! ## Software engineering connection
//!
//! The same methodological point [`crate::work_productivity_and_activity_impairment`]'s
//! pitfalls section makes about its own self-rated 0–10 presenteeism scale
//! applies here in reverse: a `DevEx` or engagement survey that asks people to
//! rate something on an unexamined 0–10 scale is skipping the step TTO
//! exists to provide — a validated elicitation method for the weighting
//! scale itself, not just an assumed linear scale. Before building a
//! composite index on top of a self-rated number, ask what elicited it and
//! whether that method was validated, the same question health economists
//! ask of a utility weight before it goes into a QALY.
//!
//! ## Pitfalls
//!
//! - **Individual-value generalization.** TTO values are elicited from a
//!   *sample* of the general public (or patients), not the individual whose
//!   care is being decided — using one respondent's TTO value as if it
//!   generalizes is a sampling error.
//! - **Wrong formulation for the state.** The standard TTO formula assumes
//!   the state is unambiguously better than death; applying it to a state
//!   some respondents would consider worse than death, without switching to
//!   the extended formulation, silently produces a wrong (positive) utility.
//! - **Incomparable durations.** TTO values elicited via different
//!   remaining-life durations `T` for the worse-than-death comparison are
//!   not directly comparable without checking the study design held `T`
//!   constant.
//!
//! ## Sources
//!
//! - Torrance GW. "Social preferences for health states: an empirical
//!   evaluation of three measurement techniques." Socioecon Plan Sci.
//!   1976;10(3):129-36.
//! - Patrick DL, Starks HE, Cain KC, Uhlmann RF, Pearlman RA. "Measuring
//!   preferences for health states worse than death." Med Decis Making.
//!   1994;14(1):9-18.
//!
//! Topic doc: health-economics-metrics/topics/time-trade-off-utility.md

/// Standard TTO utility for a health state considered better than death.
///
/// The respondent judges `time_in_full_health` years in full health
/// equivalent to `time_in_impaired_state` years in the impaired state;
/// utility is the ratio of the two.
///
/// # Arguments
///
/// * `time_in_full_health` — years `X` in full health judged equivalent
///   (worked example: `7.0`).
/// * `time_in_impaired_state` — years `T` spent in the impaired state
///   (worked example: `10.0`).
///
/// # Returns
///
/// `Some(time_in_full_health / time_in_impaired_state)`, or `None` when
/// `time_in_impaired_state` is `0.0` (the ratio is undefined).
///
/// # Examples
///
/// ```rust
/// use health_economics::time_trade_off_utility::time_trade_off_utility;
///
/// // 7 years full health ~ 10 years impaired → utility 0.7.
/// let utility = time_trade_off_utility(7.0, 10.0).unwrap();
/// assert!((utility - 0.7).abs() < 1e-9);
///
/// assert!(time_trade_off_utility(7.0, 0.0).is_none());
/// ```
#[must_use]
pub fn time_trade_off_utility(
    time_in_full_health: f64,
    time_in_impaired_state: f64,
) -> Option<f64> {
    if time_in_impaired_state == 0.0 {
        None
    } else {
        Some(time_in_full_health / time_in_impaired_state)
    }
}

/// Extended TTO utility for a health state considered worse than death.
///
/// The respondent trades `time_traded_for_death` years of a
/// `total_duration`-year remaining life for immediate death, preferring
/// `total_duration - time_traded_for_death` years in full health followed by
/// death over `total_duration` years in the worse-than-death state. The
/// result is negative, anchored so that death = 0.
///
/// # Arguments
///
/// * `time_traded_for_death` — years `a` of remaining life traded for
///   immediate death (worked example: `2.0`).
/// * `total_duration` — total remaining years `T` being valued (worked
///   example: `10.0`).
///
/// # Returns
///
/// `Some(-time_traded_for_death / (total_duration - time_traded_for_death))`,
/// or `None` when `total_duration - time_traded_for_death` is `0.0` (the
/// ratio is undefined).
///
/// # Examples
///
/// ```rust
/// use health_economics::time_trade_off_utility::worse_than_death_utility;
///
/// // Over a 10-year remaining life, 2 years traded for death → -2/8 = -0.25.
/// let utility = worse_than_death_utility(2.0, 10.0).unwrap();
/// assert!((utility - (-0.25)).abs() < 1e-9);
///
/// assert!(worse_than_death_utility(10.0, 10.0).is_none());
/// ```
#[must_use]
pub fn worse_than_death_utility(time_traded_for_death: f64, total_duration: f64) -> Option<f64> {
    let remaining = total_duration - time_traded_for_death;
    if remaining == 0.0 {
        None
    } else {
        Some(-time_traded_for_death / remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// 7 years full health judged equivalent to 10 years impaired → utility 0.7.
    #[test]
    fn seven_over_ten_years_yields_0_7_utility() {
        // Worked example: "time_trade_off_utility(7.0, 10.0) == Some(0.7)".
        let utility = time_trade_off_utility(7.0, 10.0).unwrap();
        assert!((utility - 0.7).abs() < TOL);
    }

    /// Zero years in the impaired state leaves the ratio undefined.
    #[test]
    fn zero_impaired_duration_is_none() {
        assert!(time_trade_off_utility(7.0, 0.0).is_none());
    }

    /// Trading 2 of 10 remaining years for death yields utility -0.25.
    #[test]
    fn two_traded_of_ten_years_yields_negative_0_25_utility() {
        // Worked example: "worse_than_death_utility(2.0, 10.0) == Some(-0.25)".
        let utility = worse_than_death_utility(2.0, 10.0).unwrap();
        assert!((utility - (-0.25)).abs() < TOL);
    }

    /// Trading away the entire remaining duration leaves the ratio undefined.
    #[test]
    fn trading_entire_duration_is_none() {
        assert!(worse_than_death_utility(10.0, 10.0).is_none());
    }
}
