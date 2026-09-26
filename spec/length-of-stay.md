# Spec: Length of Stay (LOS)

- **Module**: [`src/length_of_stay.rs`](../src/length_of_stay.rs)
- **Status**: implemented
- **Upstream topic**: `health-economics-metrics/topics/length-of-stay.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `length_of_stay_days(admission_day: f64, discharge_day: f64) -> f64`

- Formula: `discharge_day − admission_day`
- Total function: never returns `None`.
- Worked example: `length_of_stay_days(10.0, 16.0) == 6.0`

### `average_length_of_stay(occupied_bed_days: f64, discharges: f64) -> Option<f64>`

- Formula: `occupied_bed_days / discharges`
- Returns `None` iff `discharges == 0.0`
- Worked example: `average_length_of_stay(240.0, 40.0) == Some(6.0)`

### `mean_length_of_stay(spells: &[f64]) -> Option<f64>`

- Formula: arithmetic mean of `spells`
- Returns `None` iff `spells` is empty
- Worked example: `mean_length_of_stay(&[2.0, 3.0, 3.0, 4.0, 5.0, 6.0, 61.0]) == Some(12.0)`

### `median_length_of_stay(spells: &[f64]) -> Option<f64>`

- Formula: sorts a copy of `spells` (via `f64::total_cmp`); for an odd count
  returns the middle order statistic, for an even count returns
  `f64::midpoint` of the two central order statistics.
- Returns `None` iff `spells` is empty
- Worked example: `median_length_of_stay(&[2.0, 3.0, 3.0, 4.0, 5.0, 6.0, 61.0]) == Some(4.0)`

### `beds_occupied(admissions_per_day: f64, average_los_days: f64) -> f64`

- Formula: `admissions_per_day × average_los_days` (Little's Law)
- Total function: never returns `None`.
- Worked example: `beds_occupied(40.0, 6.0) == 240.0`

### `beds_freed(admissions_per_day: f64, los_before_days: f64, los_after_days: f64) -> f64`

- Formula: `beds_occupied(admissions_per_day, los_before_days) − beds_occupied(admissions_per_day, los_after_days)`
- Total function: never returns `None`. Negative when LOS rises.
- Worked example: `beds_freed(40.0, 6.0, 5.6) == 16.0`

### `annual_bed_days_freed(beds_freed: f64) -> f64`

- Formula: `beds_freed × 365.0`
- Total function: never returns `None`.
- Worked example: `annual_bed_days_freed(16.0) == 5_840.0`

## Invariants

- `mean_length_of_stay` and `median_length_of_stay` diverge under a
  right-skewed long-stay tail; the module's rustdoc directs callers to
  report both rather than either alone (its own worked example gives mean
  `12.0` vs median `4.0` for the same spell set).
- `beds_freed(rate, before, after) == beds_occupied(rate, before) − beds_occupied(rate, after)`
  by direct composition of `beds_occupied` (Little's Law applied twice).
