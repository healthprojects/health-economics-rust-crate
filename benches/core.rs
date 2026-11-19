//! Dependency-free performance smoke benchmarks for representative workloads.

use std::hint::black_box;
use std::time::{Duration, Instant};

use health_economics::markov_cohort_simulation::simulate_cohort;
use health_economics::net_monetary_benefit::net_monetary_benefit;
use health_economics::probabilistic_sensitivity_analysis::{
    MigrationCase, simulate_migration_net_benefits,
};

fn measure(iterations: u64, mut operation: impl FnMut()) -> Duration {
    let started = Instant::now();
    for _ in 0..iterations {
        operation();
    }
    started.elapsed()
}

fn main() {
    let scalar_iterations = 10_000_000;
    let scalar = measure(scalar_iterations, || {
        black_box(net_monetary_benefit(
            black_box(30.0),
            black_box(400_000.0),
            black_box(20_000.0),
        ));
    });

    let transition = [vec![0.9, 0.1], vec![0.0, 1.0]];
    let markov_iterations = 100_000;
    let markov = measure(markov_iterations, || {
        black_box(simulate_cohort(
            black_box(&[1.0, 0.0]),
            black_box(&transition),
            black_box(&[1_000.0, 0.0]),
            black_box(&[0.8, 0.0]),
            black_box(1.0),
            black_box(40),
            black_box(0.035),
        ));
    });

    let migration = MigrationCase {
        cost_mean: 800_000.0,
        cost_sd: 200_000.0,
        benefit_mean: 350_000.0,
        benefit_sd: 150_000.0,
        duration_low: 3.0,
        duration_high: 6.0,
    };
    let psa_iterations = 100;
    let psa = measure(psa_iterations, || {
        black_box(simulate_migration_net_benefits(
            black_box(&migration),
            black_box(10_000),
            black_box(42),
        ));
    });

    println!(
        "scalar NMB: {:.2} ns/call",
        scalar.as_secs_f64() * 1e9 / 10_000_000.0
    );
    println!(
        "40-cycle two-state Markov: {:.2} ns/run",
        markov.as_secs_f64() * 1e9 / 100_000.0
    );
    println!(
        "10,000-draw migration PSA: {:.2} ms/run",
        psa.as_secs_f64() * 1e3 / 100.0
    );
}
