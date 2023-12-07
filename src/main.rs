use std::time::Instant;

mod day_05;

fn main() {
    let now = Instant::now();

    day_05::part_a();
    day_05::part_b();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
