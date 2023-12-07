use std::time::Instant;

mod day_07;

fn main() {
    let now = Instant::now();

    day_07::part_a();
    day_07::part_b();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
