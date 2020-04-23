use itertools::Itertools;

const OPTIMIZE_STEPS: usize = 30;
const LEARN_RATE: f64 = 0.1;
const DIST_WEIGHT: f64 = 0.3;
const OVERLAP_WEIGHT: f64 = 1. - DIST_WEIGHT;
const OVERLAP_DIST: f64 = 6.;

fn dist(a: f64, b: f64) -> f64 {
    let d = (a - b).abs();
    d.min(360. - d)
}

fn overlap_loss(angles: &[f64]) -> f64 {
    angles
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| (OVERLAP_DIST - dist(a, b)).max(0.))
        .sum()
}

fn deviation_loss(angles: &[f64], targets: &[f64]) -> f64 {
    assert!(angles.len() == targets.len());
    DIST_WEIGHT
        * targets
            .iter()
            .zip(angles.iter())
            .map(|(&target, &angle)| (target - angle).powi(2))
            .sum::<f64>()
        / (2 * angles.len()) as f64
}

fn deviation_gradient(angle: f64, target: f64) -> f64 {
    if is_sorted(angle, target) {
        -dist(angle, target)
    } else {
        dist(angle, target)
    }
}

fn is_sorted(a: f64, b: f64) -> bool {
    ((b - a) % 360. - dist(a, b)).abs() < std::f64::EPSILON
}

#[allow(unused)]
fn total_loss(angles: &[f64], targets: &[f64]) -> f64 {
    assert!(angles.len() == targets.len());
    overlap_loss(angles) + deviation_loss(angles, targets)
}

pub fn optimize(angles: &[f64]) -> Vec<f64> {
    let mut candidate = angles.to_vec();
    for _ in 0..OPTIMIZE_STEPS {
        let mut deltas = vec![0.; angles.len()];
        for (i, j) in (0..angles.len()).tuple_combinations() {
            let ang_i = candidate[i];
            let ang_j = candidate[j];
            if dist(ang_i, ang_j) < OVERLAP_DIST {
                if is_sorted(ang_i, ang_j) {
                    deltas[i] -= OVERLAP_WEIGHT;
                    deltas[j] += OVERLAP_WEIGHT;
                } else {
                    deltas[i] += OVERLAP_WEIGHT;
                    deltas[j] -= OVERLAP_WEIGHT;
                }
            }
        }
        let dev_gradient = candidate
            .iter()
            .zip(angles.iter())
            .map(|(&ang, &tgt)| DIST_WEIGHT * deviation_gradient(ang, tgt));
        for (i, g) in dev_gradient.enumerate() {
            deltas[i] += g;
        }
        candidate = candidate
            .iter()
            .zip(deltas)
            .map(|(ang, delta)| ang + LEARN_RATE * delta)
            .collect();
    }
    candidate
}
