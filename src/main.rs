use std::time::Instant;

mod day_04;

fn main() {
    let now = Instant::now();

    day_04::part_a();
    day_04::part_b();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
