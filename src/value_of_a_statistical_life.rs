//! # Value of a Statistical Life (VSL)
//!
//! The value of a statistical life (VSL) — called the "value of a prevented
//! fatality" (VPF) in UK usage — is the amount a *population* is
//! collectively willing to pay to reduce the risk of one statistical death,
//! derived from wage-risk trade-off studies (how much extra pay workers
//! demand for riskier jobs) and stated-preference surveys. It is not the
//! price of any identified individual's life; it is a population-risk
//! construct used in regulatory cost-benefit analysis (transport safety,
//! environmental regulation), and it comes from a different theoretical
//! tradition than `willingness_to_pay_thresholds`' QALY × threshold
//! valuation — labour-market revealed preference rather than
//! health-budget-constrained methodology.
//!
//! ## Formula
//!
//! ```text
//! Deaths averted = population × risk_reduction_per_person
//!   (risk_reduction_per_person is a probability, e.g. 0.000001 = 1-in-a-million
//!    reduction in annual mortality risk)
//!
//! Monetized mortality benefit = deaths_averted × value_of_prevented_fatality
//! ```
//!
//! ## Why it matters
//!
//! VSL/VPF is the standard tool for monetizing mortality-risk reductions in
//! regulatory cost-benefit analysis. HM Treasury's Green Book publishes a
//! Value of a Prevented Fatality figure derived from UK labour-market and
//! survey evidence, and the Department for Transport uses it directly in
//! road-safety appraisal. Using a VSL/VPF figure alongside a separate
//! QALY-based net monetary benefit calculation in the same case, without
//! reconciling the two frameworks, risks double counting the value of the
//! same averted deaths.
//!
//! ## Example
//!
//! A region of 800,000 people benefits from a road-safety digital
//! dispatch/triage intervention that reduces each person's annual mortality
//! risk by 1 in a million.
//!
//! ```rust
//! use health_economics::value_of_a_statistical_life::{
//!     deaths_averted_from_risk_reduction, monetized_mortality_benefit,
//! };
//!
//! // 800,000 people × a 1-in-a-million annual risk reduction = 0.8 deaths averted.
//! let deaths_averted = deaths_averted_from_risk_reduction(800_000.0, 0.000_001);
//! assert!((deaths_averted - 0.8).abs() < 1e-9);
//!
//! // UK Value of a Prevented Fatality, 2023/24 prices: £2,180,000.
//! // 0.8 × £2,180,000 = £1,744,000/year of monetized mortality benefit.
//! let benefit = monetized_mortality_benefit(deaths_averted, 2_180_000.0);
//! assert!((benefit - 1_744_000.0).abs() < 1e-9);
//! ```
//!
//! ## Software engineering connection
//!
//! Safety-critical software teams — medical device firmware, autonomous
//! vehicle software — face exactly this pricing problem when building the
//! cost-benefit case for a safety investment: how do you price "prevent one
//! catastrophic failure" when the failure is rare, severe, and spread across
//! a large population of users? VSL/VPF is a decades-old, publicly
//! documented real-world precedent for putting a number on a rare, severe,
//! population-level risk reduction — the same shape of argument as pricing
//! an SRE investment against a rare catastrophic outage, just with a
//! mortality outcome instead of a downtime outcome.
//!
//! ## Pitfalls
//!
//! - **Treating VSL as "the price of an identified life"**: it isn't. VSL/VPF
//!   is a population statistical construct derived from risk-reduction
//!   trade-offs across many people, not a valuation of any specific person's
//!   life or death.
//! - **Double counting against a QALY-based net monetary benefit**: using a
//!   VSL/VPF figure and a separate QALY-based NMB calculation in the same
//!   case, without reconciling them, double counts the same averted deaths.
//!   Pick one framework per case.
//! - **Transplanting a VSL estimate across contexts without adjustment**: a
//!   VSL derived from one country's labour market, or from working-age
//!   wage-risk data, applied unadjusted to a different income context or a
//!   different population (children, retirees) is a long-standing, genuinely
//!   contested methodological issue, not a solved one.
//!
//! ## Sources
//!
//! - HM Treasury, The Green Book: Central Government Guidance on Appraisal
//!   and Evaluation — Value of a Prevented Fatality supplementary guidance
//!   (2023/24 prices; Green Book values are updated annually).
//!   <https://www.gov.uk/government/publications/the-green-book-appraisal-and-evaluation-in-central-government>
//! - US EPA, "Mortality Risk Valuation" (the US VSL tradition, cited for
//!   contrast with the UK VPF figure above).
//!   <https://www.epa.gov/environmental-economics/mortality-risk-valuation>
//! - Viscusi WK, Aldy JE. "The Value of a Statistical Life: A Critical Review
//!   of Market Estimates Throughout the World." J Risk Uncertain.
//!   2003;27(1):5-76.
//!
//! Topic doc: health-economics-metrics/topics/value-of-a-statistical-life.md

/// Expected deaths averted by a population-wide mortality risk reduction.
///
/// `population × risk_reduction_per_person`. `risk_reduction_per_person` is
/// a probability (e.g. `0.000_001` for a 1-in-a-million reduction in annual
/// mortality risk), so the result is a fractional "statistical" death count,
/// not a whole number of identified people.
///
/// # Arguments
///
/// * `population` — number of people exposed to the risk reduction.
/// * `risk_reduction_per_person` — reduction in annual mortality
///   probability per person (e.g. `0.000_001` = 1 in a million).
///
/// # Returns
///
/// Expected deaths averted across the population per year.
///
/// # Examples
///
/// ```rust
/// use health_economics::value_of_a_statistical_life::deaths_averted_from_risk_reduction;
///
/// // 800,000 people, 1-in-a-million annual risk reduction: 0.8 deaths averted.
/// let deaths_averted = deaths_averted_from_risk_reduction(800_000.0, 0.000_001);
/// assert!((deaths_averted - 0.8).abs() < 1e-9);
/// ```
#[must_use]
pub fn deaths_averted_from_risk_reduction(population: f64, risk_reduction_per_person: f64) -> f64 {
    population * risk_reduction_per_person
}

/// Monetized mortality benefit: deaths averted × the value of a prevented
/// fatality (VSL/VPF).
///
/// # Arguments
///
/// * `deaths_averted` — expected statistical deaths averted (may be
///   fractional).
/// * `value_of_prevented_fatality` — the VSL/VPF figure in use (e.g. HM
///   Treasury/DfT's £2,180,000 at 2023/24 prices — the Green Book updates
///   this annually, re-verify before citing in a live analysis).
///
/// # Returns
///
/// Monetized mortality benefit, in the same currency as
/// `value_of_prevented_fatality`.
///
/// # Examples
///
/// ```rust
/// use health_economics::value_of_a_statistical_life::monetized_mortality_benefit;
///
/// // 0.8 deaths averted × £2,180,000 VPF = £1,744,000/year.
/// let benefit = monetized_mortality_benefit(0.8, 2_180_000.0);
/// assert!((benefit - 1_744_000.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn monetized_mortality_benefit(deaths_averted: f64, value_of_prevented_fatality: f64) -> f64 {
    deaths_averted * value_of_prevented_fatality
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// 800,000 people × a 1-in-a-million annual risk reduction = 0.8 deaths
    /// averted.
    #[test]
    fn eight_hundred_thousand_people_at_one_in_a_million_gives_0_8_deaths_averted() {
        let deaths_averted = deaths_averted_from_risk_reduction(800_000.0, 0.000_001);
        assert!((deaths_averted - 0.8).abs() < TOL);
    }

    /// 0.8 deaths averted × the UK's £2,180,000 Value of a Prevented
    /// Fatality (2023/24 prices) = £1,744,000/year.
    #[test]
    fn zero_point_eight_deaths_averted_at_uk_vpf_gives_1_744_000() {
        let benefit = monetized_mortality_benefit(0.8, 2_180_000.0);
        assert!((benefit - 1_744_000.0).abs() < TOL);
    }

    /// The full chain: population and risk reduction to monetized benefit.
    #[test]
    fn full_chain_from_population_to_monetized_benefit() {
        let deaths_averted = deaths_averted_from_risk_reduction(800_000.0, 0.000_001);
        let benefit = monetized_mortality_benefit(deaths_averted, 2_180_000.0);
        assert!((benefit - 1_744_000.0).abs() < TOL);
    }
}
