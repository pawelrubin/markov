use rand;
use std::fs::File;
use std::io::Write;

const MIN_RUNS: u64 = 1_000;
const ALPHA: f32 = 0.45;
const BETA: f32 = 0.5;

fn random_walk(alpha: f32, beta: f32, epsilon: f32) -> Vec<f32> {
    let mut state = true;
    let mut pos_states = 0;
    let mut distribution: Vec<f32> = vec![0.0];
    let mut last: f32 = 0.0;
    let mut next: f32 = f32::MAX;
    let mut i: u64 = 0;
    while i < MIN_RUNS || (next - last).abs() >= epsilon {
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
        last = next;
        next = pos_states as f32 / i as f32;
        distribution.push(next);
    }
    println!("total iterations = {}", i);
    return distribution;
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
    writeln!(
        File::create("result.txt").unwrap(),
        "{}",
        random_walk(ALPHA, BETA, f32::EPSILON)
            .iter()
            .map(|n| format!("{} {}", n, 1.0 - n))
            .collect::<Vec<String>>()
            .join("\n")
    ).unwrap();

    writeln!(
        File::create("trajectory.txt").unwrap(),
        "{}",
        trajectory(ALPHA, BETA, 200)
            .into_iter()
            .map(|state| format!("{}", if state { 1 } else { -1 }))
            .collect::<Vec<String>>()
            .join("\n")
    ).unwrap();
}
