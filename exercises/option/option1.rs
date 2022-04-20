// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [
        Some(((0 * 1235) + 2) / (4 * 16)),
        Some(((1 * 1235) + 2) / (4 * 16)),
        Some(((2 * 1235) + 2) / (4 * 16)),
        Some(((3 * 1235) + 2) / (4 * 16)),
        Some(((4 * 1235) + 2) / (4 * 16)),
    ];
}
