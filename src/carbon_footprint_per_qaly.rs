//! # Carbon Footprint per QALY
//!
//! Carbon per QALY is an efficiency ratio — an intervention's carbon
//! emissions (or emissions avoided) divided by the QALYs it delivers —
//! directly analogous to cost per QALY, letting an intervention's carbon
//! efficiency be assessed alongside its cost efficiency. A "carbon-adjusted
//! net monetary benefit" goes a step further, monetizing the carbon impact
//! using the UK Green Book's official non-traded carbon values and netting
//! it against the standard net monetary benefit.
//!
//! ## Formula
//!
//! ```text
//! Carbon per QALY = total_emissions_tonnes_co2e / total_qalys
//!   (a negative value means net emissions AVOIDED per QALY gained — a
//!    double win: better health and lower carbon)
//!
//! Monetized carbon impact = emissions_tonnes_co2e × carbon_value_per_tonne
//!   (negative emissions × positive value = a negative cost, i.e. a benefit)
//!
//! Carbon-adjusted NMB = net_monetary_benefit − monetized_carbon_impact
//! ```
//!
//! ## Why it matters
//!
//! NICE and NHS England now expect environmental impact to be considered
//! alongside cost and QALYs. The NHS has a public net-zero commitment: net
//! zero for its direct emissions by 2040, and net zero for its full
//! supply-chain footprint by 2045. NICE's health technology evaluations
//! manual (PMG36) references environmental sustainability as an emerging
//! consideration in technology appraisal. Carbon is becoming a fourth
//! pillar of the value case, alongside cost, QALYs, and dominance on the
//! efficiency frontier — not a replacement for any of them.
//!
//! ## Example
//!
//! A telehealth service replaces in-person visits, avoiding 5,000 car
//! journeys/year at roughly 8kg `CO2e` each — 40 tonnes of `CO2e` avoided,
//! represented as a negative emissions figure — and it delivers 25 QALYs/year.
//!
//! ```rust
//! use health_economics::carbon_footprint_per_qaly::{
//!     carbon_adjusted_net_monetary_benefit, carbon_per_qaly, monetized_carbon_impact,
//! };
//!
//! // -40.0 tonnes (avoided) / 25.0 QALYs = -1.6 tonnes CO2e avoided per QALY gained.
//! let per_qaly = carbon_per_qaly(-40.0, 25.0).unwrap();
//! assert!((per_qaly - (-1.6)).abs() < 1e-9);
//!
//! // Green Book non-traded carbon value, illustrative 2023 figure: £269/tonne.
//! // -40.0 × £269 = -£10,760 (a £10,760 benefit).
//! let monetized = monetized_carbon_impact(-40.0, 269.0);
//! assert!((monetized - (-10_760.0)).abs() < 1e-9);
//!
//! // Standalone NMB of £500,000, adjusted for the carbon benefit: £510,760.
//! let adjusted = carbon_adjusted_net_monetary_benefit(500_000.0, -40.0, 269.0);
//! assert!((adjusted - 510_760.0).abs() < 1e-9);
//! ```
//!
//! The carbon saving adds to the case rather than detracting from it — the
//! double win the negative-emissions framing is meant to surface.
//!
//! ## Software engineering connection
//!
//! This is a live, current intersection with AI/cloud economics: the
//! compute carbon footprint of training and running an AI model is now a
//! real line item in NHS procurement, since NHS supplier contracts above
//! certain thresholds require a Carbon Reduction Plan. `cloud_unit_economics`
//! already tracks cost per unit of compute output; carbon per QALY is the
//! natural template for a future "carbon cost per inference" metric
//! extending that module and `inference_unit_economics` into the
//! environmental dimension, though that metric doesn't exist yet.
//!
//! ## Pitfalls
//!
//! - **Scope-boundary gaming**: counting only direct (Scope 1) emissions and
//!   excluding supply-chain (Scope 3) emissions, which are usually the
//!   majority for a digital health product's actual footprint.
//! - **Using a stale carbon value**: the Green Book updates its non-traded
//!   carbon values annually, so any cited £/tonne figure must be dated, not
//!   quoted as a fixed constant.
//! - **Treating "carbon efficient" as a substitute for "cost effective"**: a
//!   low-carbon, low-value intervention is still a poor use of NHS
//!   resources. Carbon is a fourth pillar alongside cost and QALYs, not a
//!   replacement for either.
//!
//! ## Sources
//!
//! - NHS England, "Delivering a Net Zero National Health Service" (2020,
//!   updated 2022).
//!   <https://www.england.nhs.uk/greenernhs/publication/delivering-a-net-zero-national-health-service/>
//! - HM Treasury, The Green Book: Carbon Values supplementary guidance
//!   (updated annually; non-traded central value ≈ £269/tCO2e, 2023 — date
//!   any citation).
//!   <https://www.gov.uk/government/publications/the-green-book-appraisal-and-evaluation-in-central-government>
//! - NICE health technology evaluations: the manual (PMG36).
//!   <https://www.nice.org.uk/process/pmg36>
//!
//! Topic doc: health-economics-metrics/topics/carbon-footprint-per-qaly.md

/// Carbon per QALY: emissions divided by health gain.
///
/// A negative result means net emissions are *avoided* per QALY gained — a
/// double win of better health and lower carbon.
///
/// # Arguments
///
/// * `total_emissions_tonnes_co2e` — total emissions in tonnes of CO2
///   equivalent (negative for net emissions avoided).
/// * `total_qalys` — total QALYs delivered by the intervention.
///
/// # Returns
///
/// `Some(carbon_per_qaly)` in tonnes `CO2e` per QALY; `None` when
/// `total_qalys` is zero (the ratio is undefined).
///
/// # Examples
///
/// ```rust
/// use health_economics::carbon_footprint_per_qaly::carbon_per_qaly;
///
/// // -40.0 tonnes avoided / 25.0 QALYs = -1.6 tonnes CO2e avoided per QALY.
/// let per_qaly = carbon_per_qaly(-40.0, 25.0).unwrap();
/// assert!((per_qaly - (-1.6)).abs() < 1e-9);
///
/// // Zero QALYs: the ratio is undefined.
/// assert!(carbon_per_qaly(-40.0, 0.0).is_none());
/// ```
#[must_use]
pub fn carbon_per_qaly(total_emissions_tonnes_co2e: f64, total_qalys: f64) -> Option<f64> {
    if total_qalys == 0.0 {
        None
    } else {
        Some(total_emissions_tonnes_co2e / total_qalys)
    }
}

/// Monetized carbon impact: emissions × the Green Book non-traded carbon
/// value.
///
/// Negative emissions (avoided) multiplied by a positive carbon value gives
/// a negative "cost", i.e. a benefit.
///
/// # Arguments
///
/// * `emissions_tonnes_co2e` — emissions in tonnes `CO2e` (negative for net
///   emissions avoided).
/// * `carbon_value_per_tonne` — the Green Book non-traded carbon value in
///   use (illustrative 2023 non-traded central value ≈ £269/tonne — the
///   Green Book updates carbon values annually, re-verify before citing in a
///   live analysis).
///
/// # Returns
///
/// Monetized carbon impact, in the same currency as `carbon_value_per_tonne`
/// (negative means a net benefit).
///
/// # Examples
///
/// ```rust
/// use health_economics::carbon_footprint_per_qaly::monetized_carbon_impact;
///
/// // -40.0 tonnes avoided × £269/tonne = -£10,760 (a £10,760 benefit).
/// let monetized = monetized_carbon_impact(-40.0, 269.0);
/// assert!((monetized - (-10_760.0)).abs() < 1e-9);
/// ```
#[must_use]
pub fn monetized_carbon_impact(emissions_tonnes_co2e: f64, carbon_value_per_tonne: f64) -> f64 {
    emissions_tonnes_co2e * carbon_value_per_tonne
}

/// Carbon-adjusted net monetary benefit: the standard NMB, netted against
/// the monetized carbon impact.
///
/// `net_monetary_benefit − (emissions_tonnes_co2e × carbon_value_per_tonne)`.
/// When emissions are negative (avoided), subtracting a negative monetized
/// impact *increases* the adjusted benefit.
///
/// # Arguments
///
/// * `net_monetary_benefit` — the intervention's standalone net monetary
///   benefit, before any carbon adjustment.
/// * `emissions_tonnes_co2e` — emissions in tonnes `CO2e` (negative for net
///   emissions avoided).
/// * `carbon_value_per_tonne` — the Green Book non-traded carbon value in
///   use.
///
/// # Returns
///
/// Carbon-adjusted net monetary benefit, in the same currency as
/// `net_monetary_benefit`.
///
/// # Examples
///
/// ```rust
/// use health_economics::carbon_footprint_per_qaly::carbon_adjusted_net_monetary_benefit;
///
/// // £500,000 NMB, adjusted for -40.0 tonnes avoided at £269/tonne:
/// // 500,000 - (-10,760) = £510,760.
/// let adjusted = carbon_adjusted_net_monetary_benefit(500_000.0, -40.0, 269.0);
/// assert!((adjusted - 510_760.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn carbon_adjusted_net_monetary_benefit(
    net_monetary_benefit: f64,
    emissions_tonnes_co2e: f64,
    carbon_value_per_tonne: f64,
) -> f64 {
    net_monetary_benefit - (emissions_tonnes_co2e * carbon_value_per_tonne)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f64 = 1e-9;

    /// -40.0 tonnes avoided / 25.0 QALYs = -1.6 tonnes `CO2e` avoided per QALY.
    #[test]
    fn telehealth_carbon_per_qaly_is_negative_1_6() {
        let per_qaly = carbon_per_qaly(-40.0, 25.0).unwrap();
        assert!((per_qaly - (-1.6)).abs() < TOL);
    }

    /// Zero total QALYs makes the ratio undefined.
    #[test]
    fn zero_qalys_has_no_carbon_per_qaly() {
        assert!(carbon_per_qaly(-40.0, 0.0).is_none());
    }

    /// -40.0 tonnes avoided × £269/tonne = -£10,760 (a £10,760 benefit).
    #[test]
    fn telehealth_monetized_carbon_impact_is_negative_10_760() {
        let monetized = monetized_carbon_impact(-40.0, 269.0);
        assert!((monetized - (-10_760.0)).abs() < TOL);
    }

    /// £500,000 NMB adjusted for the avoided carbon is £510,760.
    #[test]
    fn telehealth_carbon_adjusted_nmb_is_510_760() {
        let adjusted = carbon_adjusted_net_monetary_benefit(500_000.0, -40.0, 269.0);
        assert!((adjusted - 510_760.0).abs() < TOL);
    }
}
