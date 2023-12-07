use std::time::Instant;

mod day_06;

fn main() {
    let now = Instant::now();

    day_06::part_a();
    day_06::part_b();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
