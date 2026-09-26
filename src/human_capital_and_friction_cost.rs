//! # Human Capital Approach vs Friction Cost Method
//!
//! These are the two competing methods for valuing lost productivity — from
//! illness, disability, or death — in cost-of-illness and cost-benefit
//! studies. The Human Capital Approach (HCA) values all lost output for the
//! full duration of absence at the wage rate; the Friction Cost Method (FCM)
//! values it only for the shorter period an employer actually needs to
//! restore production. Choosing between them changes an indirect-cost
//! estimate by two times or more.
//!
//! ## Formula
//!
//! ```text
//! Human Capital Approach:
//! HCA_cost = daily_wage × days_lost
//!
//! Friction Cost Method (simplified, capped-at-friction-period form):
//! FCM_cost = daily_wage × min(days_lost, friction_period_days)
//!
//! friction_period_days = country/sector-specific estimate of time-to-restore
//!                         production (historically ~85 days in Dutch iMTA
//!                         costing guidance; varies by country and is
//!                         periodically re-estimated)
//! ```
//!
//! The entire disagreement between the two methods lives in the `min()`: HCA
//! never caps `days_lost`, so cost keeps growing for the whole absence, while
//! FCM caps the counted days at the friction period, however long the actual
//! absence runs.
//!
//! ## Why it matters
//!
//! Indirect (productivity) costs are one of the most contested line items in
//! health economics precisely because the two standard methods disagree so
//! sharply. HCA treats every day of absence as a day of output the economy
//! genuinely loses, valued at the full wage for the full duration — or, for
//! death or permanent disability, for the remaining working life. FCM argues
//! that in an economy with unemployment and labor-market slack, most of a
//! long absence does not actually reduce national output once an employer has
//! trained a replacement or redistributed work; only the "friction period" —
//! the time to restore production to its prior level — represents a real
//! loss. FCM therefore produces systematically lower, more conservative
//! indirect-cost estimates than HCA. This is also why NICE's reference case
//! excludes productivity costs by default, reporting them, when at all, as a
//! separate societal-perspective sensitivity analysis rather than blending
//! them into the reference-case ICER.
//!
//! ## Example
//!
//! An employee is off work for 180 days, earning £150/day.
//!
//! ```rust
//! use health_economics::human_capital_and_friction_cost::{friction_cost, human_capital_cost};
//!
//! // Human Capital Approach: 150 × 180 = £27,000.
//! let hca = human_capital_cost(150.0, 180.0);
//! assert!((hca - 27_000.0).abs() < 1e-9);
//!
//! // Friction Cost Method with an 85-day friction period: 150 × 85 = £12,750.
//! let fcm = friction_cost(150.0, 180.0, 85.0);
//! assert!((fcm - 12_750.0).abs() < 1e-9);
//!
//! // FCM's £12,750 is under half of HCA's £27,000 for the same absence.
//! assert!(fcm < hca / 2.0);
//! ```
//!
//! ## Software engineering connection
//!
//! This maps directly onto how a team values an engineer leaving:
//!
//! - **HCA-style attrition costing**: valuing the loss as the departed
//!   engineer's full salary for however long the role stays vacant. This is
//!   the naive version of most attrition-cost models, and it overstates the
//!   loss for the same reason HCA overstates productivity loss — it assumes
//!   the vacant capacity was fully productive the whole time and nothing else
//!   absorbed the slack. This is the same territory the crate's
//!   `workforce_retention` module quantifies (recruitment, onboarding, and
//!   vacancy-cover costs).
//! - **FCM-style attrition costing**: valuing the loss only for the actual
//!   time-to-backfill-and-ramp a replacement — the engineering "friction
//!   period." This is the more defensible number for a business case, exactly
//!   as FCM is the more conservative choice in a cost-of-illness study.
//! - The underlying discipline is the same one in the crate's
//!   `opportunity_cost` module: value a displaced resource by what is
//!   genuinely lost, not by a headline duration multiplied by a rate.
//!
//! ## Pitfalls
//!
//! - **Mixing HCA and FCM within one analysis, or reporting only one without
//!   disclosing the choice.** The same absence data can produce a 2x+
//!   difference in reported cost depending on method; the choice must be
//!   stated, not buried.
//! - **Using HCA for a societal-perspective case without flagging it as a
//!   sensitivity analysis.** NICE's reference case explicitly excludes
//!   productivity costs; a societal-perspective HCA estimate belongs in a
//!   scenario analysis, not the headline ICER.
//! - **Applying either method to unpaid or non-market work (e.g. caregiving)
//!   without adjustment.** Both methods assume a wage-rate proxy for value,
//!   which does not transfer cleanly to work with no market wage.
//!
//! ## Sources
//!
//! - Koopmanschap MA, Rutten FFH, van Ineveld BM, van Roijen L. "The friction
//!   cost method for measuring indirect costs of disease." Journal of Health
//!   Economics 1995;14(2):171-89.
//! - Drummond MF, Sculpher MJ, Claxton K, Stoddart GL, Torrance GW. "Methods
//!   for the Economic Evaluation of Health Care Programmes." 4th ed. Oxford
//!   University Press — chapter on productivity costs.
//! - NICE health technology evaluations manual (PMG36) — reference-case
//!   perspective and optional societal-perspective guidance.
//!   <https://www.nice.org.uk/process/pmg36>
//!
//! Topic doc: health-economics-metrics/topics/human-capital-and-friction-cost.md

/// Human Capital Approach cost: `daily_wage × days_lost`.
///
/// Values every day of absence at the full wage rate, for the entire
/// duration — no cap. This is the theoretically simpler but usually larger
/// of the two standard indirect-cost estimates.
///
/// # Arguments
///
/// * `daily_wage` — the wage rate per day, in currency units.
/// * `days_lost` — total days of absence (or, for death/permanent
///   disability, remaining working life expressed in days).
///
/// # Returns
///
/// `daily_wage * days_lost`, in currency units.
///
/// # Examples
///
/// ```rust
/// use health_economics::human_capital_and_friction_cost::human_capital_cost;
///
/// // 180 days off work at £150/day: £27,000.
/// let cost = human_capital_cost(150.0, 180.0);
/// assert!((cost - 27_000.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn human_capital_cost(daily_wage: f64, days_lost: f64) -> f64 {
    daily_wage * days_lost
}

/// Friction Cost Method cost: `daily_wage × min(days_lost, friction_period_days)`.
///
/// Caps the counted days at the friction period — the time an employer needs
/// to restore production to its prior level — regardless of how long the
/// actual absence runs. Produces a systematically lower, more conservative
/// estimate than [`human_capital_cost`].
///
/// # Arguments
///
/// * `daily_wage` — the wage rate per day, in currency units.
/// * `days_lost` — total days of absence.
/// * `friction_period_days` — country/sector-specific estimate of
///   time-to-restore production (historically ~85 days in Dutch iMTA costing
///   guidance).
///
/// # Returns
///
/// `daily_wage * days_lost.min(friction_period_days)`, in currency units.
///
/// # Examples
///
/// ```rust
/// use health_economics::human_capital_and_friction_cost::friction_cost;
///
/// // 180 days off work at £150/day, capped at an 85-day friction period: £12,750.
/// let cost = friction_cost(150.0, 180.0, 85.0);
/// assert!((cost - 12_750.0).abs() < 1e-9);
///
/// // A short absence well within the friction period is uncapped.
/// let short = friction_cost(150.0, 10.0, 85.0);
/// assert!((short - 1_500.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn friction_cost(daily_wage: f64, days_lost: f64, friction_period_days: f64) -> f64 {
    daily_wage * days_lost.min(friction_period_days)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// HCA: 150 × 180 = £27,000.
    #[test]
    fn human_capital_cost_is_27_000() {
        // Worked example: "human_capital_cost(150.0, 180.0) == 27_000.0".
        let cost = human_capital_cost(150.0, 180.0);
        assert!((cost - 27_000.0).abs() < TOL, "got {cost}");
    }

    /// FCM with an 85-day friction period: 150 × 85 = £12,750.
    #[test]
    fn friction_cost_is_12_750() {
        // Worked example: "friction_cost(150.0, 180.0, 85.0) == 12_750.0".
        let cost = friction_cost(150.0, 180.0, 85.0);
        assert!((cost - 12_750.0).abs() < TOL, "got {cost}");
    }

    // Doc line: "under half the HCA figure".
    #[test]
    fn fcm_is_under_half_of_hca_for_the_same_absence() {
        let hca = human_capital_cost(150.0, 180.0);
        let fcm = friction_cost(150.0, 180.0, 85.0);
        assert!(fcm < hca / 2.0, "fcm {fcm} not under half of hca {hca}");
    }

    // Edge case: an absence shorter than the friction period is not capped,
    // so FCM and HCA agree.
    #[test]
    fn short_absence_within_friction_period_matches_hca() {
        let hca = human_capital_cost(150.0, 10.0);
        let fcm = friction_cost(150.0, 10.0, 85.0);
        assert!((hca - fcm).abs() < TOL, "hca {hca} fcm {fcm}");
    }
}
