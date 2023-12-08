use std::time::Instant;

mod day_08;

fn main() {
    let now = Instant::now();

    day_08::part_a();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
