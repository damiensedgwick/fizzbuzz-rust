fn main() {
    let stop_at = 1_000_000;

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
