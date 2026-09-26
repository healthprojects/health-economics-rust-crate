//! # Cross-Currency ICER Comparison
//!
//! Comparing an [`crate::incremental_cost_effectiveness_ratio`] computed in
//! one country's currency against another country's
//! [`crate::willingness_to_pay_thresholds`] — or pooling cost data collected
//! across a multinational trial — requires an explicit, auditable
//! currency-conversion step. ISPOR's methods guidance for multinational
//! trials recommends converting resource costs using purchasing power
//! parity (PPP) — not market exchange rates — when comparing the real
//! economic value of resources across countries, and reserves market FX
//! rates for what they're actually for: modelling real cross-border cash
//! payment flows. Conflating the two is one of the most common
//! multinational HTA methodology errors, and it can change the converted
//! figure enough to flip an adoption decision.
//!
//! ## Formula
//!
//! ```text
//! icer_in_local_currency = convert(icer_in_source_currency, conversion_factor)
//!
//! conversion_factor should be:
//!   PPP conversion factor  -- for comparing the real economic value of
//!                              resources across countries (ISPOR guidance
//!                              for multinational cost-effectiveness analysis)
//!   market FX rate         -- only for actual cross-border cash payments
//!
//! adopt if icer_in_local_currency < local_threshold
//! ```
//!
//! ## Why it matters
//!
//! The decision rule itself is the ordinary ICER threshold rule — adopt if
//! ICER is less than the threshold — the methodological question is
//! entirely about *which conversion factor* produces the figure that rule
//! is applied to. Willke et al. (*Health Economics*, 1998) is the
//! foundational ISPOR-aligned reference for why PPP, not market FX, is the
//! right conversion basis for multinational cost-effectiveness comparisons.
//!
//! ## Example
//!
//! A drug's ICER from a US trial is $45,000/QALY. A hypothetical importing
//! country sets its own illustrative threshold at £34,000/QALY (a
//! hypothetical country-specific figure for this example only — real
//! thresholds vary by country and change over time, and must always be
//! sourced and dated). Converting with a PPP factor of 0.72 gives
//! £32,400/QALY, which clears the threshold; converting the *same*
//! $45,000/QALY ICER with a market exchange rate of 0.79 instead gives
//! £35,550/QALY, which does not. The conversion method alone flips the
//! adoption decision:
//!
//! ```rust
//! use health_economics::cross_currency_icer_comparison::{adopt_at_threshold, convert};
//! use rusty_money::{iso, ExchangeRate, Money};
//! use rust_decimal::Decimal;
//!
//! let icer_usd = Money::from_major(45_000, iso::USD); // $45,000/QALY
//! let threshold = Money::from_major(34_000, iso::GBP); // hypothetical, illustrative
//!
//! // Purchasing power parity conversion (ISPOR-recommended for this purpose).
//! let rate_ppp = ExchangeRate::new(iso::USD, iso::GBP, Decimal::new(72, 2)).unwrap();
//! let icer_ppp = convert(&rate_ppp, icer_usd).unwrap();
//! assert_eq!(icer_ppp, Money::from_major(32_400, iso::GBP));
//! assert_eq!(adopt_at_threshold(&icer_ppp, &threshold), Ok(true)); // adopt
//!
//! // Market exchange rate conversion (the wrong basis for this comparison).
//! let rate_market = ExchangeRate::new(iso::USD, iso::GBP, Decimal::new(79, 2)).unwrap();
//! let icer_market = convert(&rate_market, icer_usd).unwrap();
//! assert_eq!(icer_market, Money::from_major(35_550, iso::GBP));
//! assert_eq!(adopt_at_threshold(&icer_market, &threshold), Ok(false)); // reject
//! ```
//!
//! The same underlying $45,000/QALY ICER produces an adopt decision under
//! PPP conversion and a reject decision under market-FX conversion — the
//! concrete illustration of why ISPOR guidance treats the choice of
//! conversion factor as methodologically consequential, not a rounding
//! detail.
//!
//! ## Software engineering connection
//!
//! This is identical to i18n/l10n multi-currency pricing correctness in
//! commercial software — a `SaaS` pricing page must never silently compare a
//! `$` amount to a `£` price. The type-level guarantee here — `Money`'s
//! comparison methods return a [`Result`] and refuse to compare mismatched
//! currencies — is a direct software-engineering parallel to the
//! health-economics methodological point: don't compare unconverted
//! figures across currencies.
//!
//! ## Pitfalls
//!
//! - **Silently comparing amounts in different currencies** in ad hoc
//!   spreadsheet HTA work — a bug class a real `Money` type prevents at
//!   the type level, since [`Money::lt`], [`Money::gt`], and [`Money::eq`]
//!   all return a [`Result`] specifically because of this.
//! - **Conflating market exchange rate with PPP** — the single most
//!   common multinational HTA methods error per ISPOR guidance.
//! - **Not dating the exchange rate or PPP index used** — both move over
//!   time, so any cited conversion factor must be dated, exactly like this
//!   crate's other dated-figure conventions (Green Book carbon values,
//!   value of a prevented fatality, and so on).
//!
//! ## Sources
//!
//! - Willke RJ, Glick HA, Polsky D, Schulman K. "Estimating
//!   country-specific cost-effectiveness from multinational clinical
//!   trials." Health Econ. 1998;7(6):481-93.
//! - OECD, Purchasing Power Parities (PPP) data. <https://www.oecd.org>
//!
//! Topic doc: health-economics-metrics/topics/cross-currency-icer-comparison.md

use rusty_money::{iso, ExchangeRate, Money, MoneyError};

/// Converts `amount` into another currency using an explicit, auditable
/// exchange rate.
///
/// The rate should be a purchasing power parity (PPP) conversion factor
/// when the goal is comparing the real economic value of resources across
/// countries (as in multinational cost-effectiveness analysis), and a
/// market exchange rate only when modelling actual cross-border cash
/// payment flows.
///
/// # Arguments
///
/// * `rate` — the exchange rate to convert with; its `from` currency must
///   match `amount`'s currency.
/// * `amount` — the amount to convert.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `amount`'s currency does
/// not match `rate`'s `from` currency, or [`MoneyError::Overflow`] if the
/// conversion arithmetic overflows.
///
/// # Examples
///
/// ```rust
/// use health_economics::cross_currency_icer_comparison::convert;
/// use rusty_money::{iso, ExchangeRate, Money};
/// use rust_decimal::Decimal;
///
/// let icer_usd = Money::from_major(45_000, iso::USD);
/// let rate_ppp = ExchangeRate::new(iso::USD, iso::GBP, Decimal::new(72, 2)).unwrap();
/// let icer_ppp = convert(&rate_ppp, icer_usd).unwrap();
/// assert_eq!(icer_ppp, Money::from_major(32_400, iso::GBP));
/// ```
pub fn convert(
    rate: &ExchangeRate<'static, iso::Currency>,
    amount: Money<'static, iso::Currency>,
) -> Result<Money<'static, iso::Currency>, MoneyError> {
    rate.convert(&amount)
}

/// Reports whether an ICER (already converted into the threshold's
/// currency) clears a willingness-to-pay threshold.
///
/// Returns `Ok(true)` (adopt) if `icer_in_threshold_currency` is strictly
/// less than `threshold`, `Ok(false)` (reject) otherwise.
///
/// # Arguments
///
/// * `icer_in_threshold_currency` — the ICER, already converted into the
///   same currency as `threshold`. Converting an unconverted ICER here is
///   a caller error this function is specifically designed to catch, not
///   silently accept.
/// * `threshold` — the local willingness-to-pay threshold.
///
/// # Errors
///
/// Returns [`MoneyError::CurrencyMismatch`] if `icer_in_threshold_currency`
/// and `threshold` are in different currencies. This is by design: a real
/// `Money` type's comparison methods make "compared a dollar figure to a
/// pound threshold without converting first" a caught error rather than a
/// silent bug.
///
/// # Examples
///
/// ```rust
/// use health_economics::cross_currency_icer_comparison::adopt_at_threshold;
/// use rusty_money::{iso, Money};
///
/// let icer_ppp = Money::from_major(32_400, iso::GBP);
/// let threshold = Money::from_major(34_000, iso::GBP);
/// assert_eq!(adopt_at_threshold(&icer_ppp, &threshold), Ok(true));
/// ```
pub fn adopt_at_threshold(
    icer_in_threshold_currency: &Money<'static, iso::Currency>,
    threshold: &Money<'static, iso::Currency>,
) -> Result<bool, MoneyError> {
    icer_in_threshold_currency.lt(threshold)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    /// Worked example: $45,000/QALY converted at PPP (0.72) is
    /// £32,400/QALY, which clears the £34,000 threshold.
    #[test]
    fn ppp_conversion_clears_the_threshold() {
        let icer_usd = Money::from_major(45_000, iso::USD);
        let threshold = Money::from_major(34_000, iso::GBP);

        let rate_ppp = ExchangeRate::new(iso::USD, iso::GBP, Decimal::new(72, 2)).unwrap();
        let icer_ppp = convert(&rate_ppp, icer_usd).unwrap();

        assert_eq!(icer_ppp, Money::from_major(32_400, iso::GBP));
        assert_eq!(adopt_at_threshold(&icer_ppp, &threshold), Ok(true));
    }

    /// Worked example: the same $45,000/QALY ICER converted at a market
    /// exchange rate (0.79) is £35,550/QALY, which does NOT clear the
    /// £34,000 threshold -- the conversion method alone flips the decision.
    #[test]
    fn market_rate_conversion_misses_the_threshold() {
        let icer_usd = Money::from_major(45_000, iso::USD);
        let threshold = Money::from_major(34_000, iso::GBP);

        let rate_market = ExchangeRate::new(iso::USD, iso::GBP, Decimal::new(79, 2)).unwrap();
        let icer_market = convert(&rate_market, icer_usd).unwrap();

        assert_eq!(icer_market, Money::from_major(35_550, iso::GBP));
        assert_eq!(adopt_at_threshold(&icer_market, &threshold), Ok(false));
    }

    /// Comparing amounts in different currencies is a caught error, not a
    /// silent bug.
    #[test]
    fn comparing_mismatched_currencies_is_an_error() {
        let icer_usd = Money::from_major(45_000, iso::USD);
        let threshold = Money::from_major(34_000, iso::GBP);

        assert!(adopt_at_threshold(&icer_usd, &threshold).is_err());
    }
}
