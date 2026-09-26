//! # Work Productivity and Activity Impairment (WPAI)
//!
//! WPAI is a validated self-report questionnaire (Reilly, Zbrozek, Dasbach,
//! 1993) measuring how much a health problem affects paid work and daily
//! activities, usually over the past 7 days. It splits the loss into
//! *absenteeism* — work time literally missed — and *presenteeism* — reduced
//! productivity while physically at work — and the second is usually the
//! larger, more hidden cost component.
//!
//! ## Formula
//!
//! ```text
//! Absenteeism % = hours_missed_due_to_health / (hours_missed_due_to_health + hours_worked) × 100
//!
//! Presenteeism %  = self-rated 0–10 impairment while working, × 10
//!                   (elicited directly via questionnaire, not derived here)
//!
//! Overall Work Impairment % =
//!     Absenteeism% + (1 − Absenteeism%/100) × Presenteeism%
//!     (combines the two so the total can never exceed 100%)
//!
//! Productivity cost = Overall Work Impairment% / 100 × period_earnings
//! ```
//!
//! The overall-impairment formula is deliberately not a simple sum: adding
//! the two percentages directly could exceed 100%, so presenteeism is applied
//! only to the *remaining* (non-absent) share of work time.
//!
//! ## Why it matters
//!
//! Simple sick-day counts only see absenteeism. A clinician or knowledge
//! worker who never takes a day off but works at 60% capacity through a
//! chronic condition contributes zero to an absence register while still
//! generating a large, real productivity loss — WPAI is designed specifically
//! to surface that invisible cost. Because it is a validated instrument
//! rather than a bespoke survey, its scores are usable in patient-reported
//! outcome evidence packages and cost-of-illness studies without the reviewer
//! needing to re-validate the measure. As a self-report instrument, it is
//! itself a form of PROM, distinguished mainly by its focus on work and
//! activity rather than symptoms or quality of life.
//!
//! ## Example
//!
//! An employee with migraine is scheduled for a 40-hour week but misses 4
//! hours of it, and separately self-rates their productivity impact while
//! working as 3 out of 10 on the WPAI questionnaire (i.e. 30%).
//!
//! ```rust
//! use health_economics::work_productivity_and_activity_impairment::{
//!     absenteeism_percent, overall_work_impairment_percent, productivity_cost,
//! };
//!
//! // 4 hours missed of a 36-hour worked + 4-hour-missed week: 10% absenteeism.
//! let absenteeism = absenteeism_percent(4.0, 36.0).unwrap();
//! assert!((absenteeism - 10.0).abs() < 1e-9);
//!
//! // Presenteeism is given directly (3/10 → 30%), not derived here.
//! let presenteeism = 30.0;
//!
//! // Overall impairment: 10 + 0.9 × 30 = 37%.
//! let overall = overall_work_impairment_percent(absenteeism, presenteeism);
//! assert!((overall - 37.0).abs() < 1e-9);
//!
//! // Over a 5-day week earning £800 (£160/day): 37% × £800 = £296.
//! let cost = productivity_cost(overall, 800.0);
//! assert!((cost - 296.0).abs() < 1e-9);
//! ```
//!
//! ## Software engineering connection
//!
//! This maps directly onto engineering-team health metrics:
//!
//! - **Absenteeism** is sick leave and PTO — visible, already tracked, and
//!   the easy part.
//! - **Presenteeism** is the burned-out or context-switching-overloaded
//!   engineer who is present in every stand-up while operating at reduced
//!   capacity — usually the larger and more hidden cost, and invisible to
//!   headcount or attendance data. It shows up instead as reduced throughput
//!   in the crate's `dora_metrics` and `flow_metrics` modules, or as slower
//!   resolution of the very `technical_debt` whose interest-metaphor
//!   compounds the impairment further.
//! - The engineering lesson is the same as the clinical one: measuring only
//!   absence and calling it "productivity loss" systematically understates
//!   the real cost, because it misses everyone who is present but impaired.
//!
//! ## Pitfalls
//!
//! - **Self-report recall bias.** A 7-day recall window is subject to the
//!   same reporting distortions as any retrospective self-report.
//! - **Treating the 0–10 presenteeism scale as a true physical measurement.**
//!   It is ordinal, elicited by self-rating, not a validated physical
//!   quantity — treating differences on it as strictly linear or interval is
//!   a modeling convenience, not a validated physical fact.
//! - **Pooling scores across WPAI variants.** WPAI has several
//!   condition-specific versions — WPAI:GH (general health), WPAI:SHP
//!   (specific health problem), and disease-specific variants — and scores
//!   from different variants should not be pooled or compared without first
//!   checking they are the same instrument version.
//!
//! ## Sources
//!
//! - Reilly MC, Zbrozek AS, Dasbach EJ. "The validity and reproducibility of
//!   a work productivity and activity impairment instrument."
//!   `PharmacoEconomics` 1993;4(5):353-65.
//! - WPAI instrument documentation, Reilly Associates — the official scoring
//!   reference. <https://www.reillyassociates.net/>
//!
//! Topic doc: health-economics-metrics/topics/work-productivity-and-activity-impairment.md

/// Absenteeism percent: the share of scheduled work time missed due to health.
///
/// # Arguments
///
/// * `hours_missed` — hours of scheduled work missed due to the health
///   problem, over the recall period.
/// * `hours_worked` — hours actually worked over the same recall period.
///
/// # Returns
///
/// `Some(hours_missed / (hours_missed + hours_worked) * 100.0)`, or `None` if
/// `hours_missed + hours_worked` is `0.0` (no scheduled time to measure
/// against).
///
/// # Examples
///
/// ```rust
/// use health_economics::work_productivity_and_activity_impairment::absenteeism_percent;
///
/// // 4 hours missed of a 40-hour week (36 worked, 4 missed): 10%.
/// let pct = absenteeism_percent(4.0, 36.0).unwrap();
/// assert!((pct - 10.0).abs() < 1e-9);
///
/// // No scheduled hours at all is undefined.
/// assert!(absenteeism_percent(0.0, 0.0).is_none());
/// ```
#[must_use]
pub fn absenteeism_percent(hours_missed: f64, hours_worked: f64) -> Option<f64> {
    let denominator = hours_missed + hours_worked;
    if denominator == 0.0 {
        None
    } else {
        Some(hours_missed / denominator * 100.0)
    }
}

/// Overall work impairment percent: absenteeism plus presenteeism applied to
/// the remaining (non-absent) share of work time.
///
/// `Absenteeism% + (1 − Absenteeism%/100) × Presenteeism%`. Combining the two
/// this way — rather than simply adding them — guarantees the total can never
/// exceed 100%.
///
/// # Arguments
///
/// * `absenteeism_percent` — percent of scheduled work time missed (see
///   [`absenteeism_percent`]).
/// * `presenteeism_percent` — self-rated productivity impairment while
///   working, as a percent (elicited directly via questionnaire; this
///   function does not derive it).
///
/// # Returns
///
/// Overall work impairment as a percent (0–100).
///
/// # Examples
///
/// ```rust
/// use health_economics::work_productivity_and_activity_impairment::overall_work_impairment_percent;
///
/// // 10% absenteeism plus 30% presenteeism on the remaining 90%: 10 + 27 = 37%.
/// let overall = overall_work_impairment_percent(10.0, 30.0);
/// assert!((overall - 37.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn overall_work_impairment_percent(absenteeism_percent: f64, presenteeism_percent: f64) -> f64 {
    absenteeism_percent + (1.0 - absenteeism_percent / 100.0) * presenteeism_percent
}

/// Productivity cost: overall work impairment applied to period earnings.
///
/// # Arguments
///
/// * `overall_work_impairment_percent` — overall work impairment as a percent
///   (see [`overall_work_impairment_percent`]).
/// * `period_earnings` — earnings over the same period, in currency units.
///
/// # Returns
///
/// `overall_work_impairment_percent / 100.0 * period_earnings`, in currency
/// units.
///
/// # Examples
///
/// ```rust
/// use health_economics::work_productivity_and_activity_impairment::productivity_cost;
///
/// // 37% impairment over a 5-day week earning £800: £296.
/// let cost = productivity_cost(37.0, 800.0);
/// assert!((cost - 296.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn productivity_cost(overall_work_impairment_percent: f64, period_earnings: f64) -> f64 {
    overall_work_impairment_percent / 100.0 * period_earnings
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// 4 hours missed of a 40-hour week: 10% absenteeism.
    #[test]
    fn absenteeism_percent_is_10() {
        // Worked example: "absenteeism_percent(4.0, 36.0) == Some(10.0)".
        let pct = absenteeism_percent(4.0, 36.0).unwrap();
        assert!((pct - 10.0).abs() < TOL, "got {pct}");
    }

    /// 10% absenteeism plus 30% presenteeism: 10 + 0.9×30 = 37%.
    #[test]
    fn overall_work_impairment_is_37() {
        // Worked example: "overall_work_impairment_percent(10.0, 30.0) == 37.0".
        let overall = overall_work_impairment_percent(10.0, 30.0);
        assert!((overall - 37.0).abs() < TOL, "got {overall}");
    }

    /// 37% impairment over a £800 week: £296.
    #[test]
    fn productivity_cost_is_296() {
        // Worked example: "productivity_cost(37.0, 800.0) == 296.0".
        let cost = productivity_cost(37.0, 800.0);
        assert!((cost - 296.0).abs() < TOL, "got {cost}");
    }

    // Edge case: no scheduled hours at all leaves absenteeism undefined.
    #[test]
    fn zero_scheduled_hours_is_undefined() {
        assert!(absenteeism_percent(0.0, 0.0).is_none());
    }

    // Doc guarantee: overall impairment can never exceed 100%.
    #[test]
    fn overall_impairment_never_exceeds_100_percent() {
        let overall = overall_work_impairment_percent(80.0, 100.0);
        assert!(overall <= 100.0 + TOL, "got {overall}");
    }
}
