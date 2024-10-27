use std::time::Instant;

fn fizz_buzz(stop_at: u32) {
    for i in 1..=stop_at {
        if i % 3 == 0 {
            print!("Fizz");
        }

        if i % 5 == 0 {
            print!("Buzz");
        }

        if i % 3 != 0 && i % 5 != 0 {
            print!("{}", i);
        }

        println!();
    }
}

fn run_benchmark(stop_at: u32) -> f64 {
    let start = Instant::now();
    fizz_buzz(stop_at);
    start.elapsed().as_secs_f64()
}

fn main() {
    const STOP_AT: u32 = 1_000_000;
    let seconds = run_benchmark(STOP_AT);
    println!("\nTime taken: {:.6}s", seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_fizz_buzz() {
        const STOP_AT: u32 = 1_000_000;
        let seconds = run_benchmark(STOP_AT);
        println!("\nTest benchmark time: {:.6}s", seconds);
    }
}
