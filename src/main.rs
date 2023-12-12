use std::time::Instant;

mod day_09;

fn main() {
    let now = Instant::now();

    day_09::part_a();
    day_09::part_b();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
