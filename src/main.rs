use rand;
use std::fs::File;
use std::io::Write;

const MIN_RUNS: u64 = 1_000_000;
const ALPHA: f32 = 0.45;
const BETA: f32 = 0.5;

fn random_walk(alpha: f32, beta: f32, epsilon: f32) -> Vec<(f32, f32)> {
    let mut state = true;
    let mut pos_states = 0;

    let mut distribution: Vec<(f32, f32)> = vec![(0.0, 0.0)];

    let mut last_pos = 0.0;
    let mut last_neg = 0.0;
    let mut i: u64 = 0;
    loop {
        i += 1;
        if state {
            pos_states += 1;
            if rand::random::<f32>() < alpha {
                state = false;
            }
        } else {
            if rand::random::<f32>() < beta {
                state = true;
            }
        }
        let next_pos = pos_states as f32 / i as f32;
        let next_neg = 1.0 - (pos_states as f32 / i as f32);

        distribution.push((next_pos, next_neg));

        if i >= MIN_RUNS
            && (next_pos - last_pos).abs() < epsilon
            && (next_neg - last_neg).abs() < epsilon
        {
            return distribution;
        }
        last_pos = next_pos;
        last_neg = next_neg;
    }
}

fn trajectory(alpha: f32, beta: f32, reps: u32) -> Vec<bool> {
    let mut state = true;
    (0..reps)
        .into_iter()
        .map(|_| {
            if state && rand::random::<f32>() < alpha {
                state = false;
            } else if rand::random::<f32>() < beta {
                state = true;
            }
            state
        })
        .collect()
}

fn main() {
    let mut output_file = File::create("result.txt").unwrap();
    random_walk(ALPHA, BETA, 1.0e-6)
        .into_iter()
        .for_each(|(p, n)| writeln!(output_file, "{}", format!("{} {}", p, n)).unwrap());

    output_file = File::create("trajectory.txt").unwrap();
    trajectory(ALPHA, BETA, 200).into_iter().for_each(|state| {
        writeln!(output_file, "{}", format!("{}", if state { 1 } else { -1 })).unwrap()
    });
}
