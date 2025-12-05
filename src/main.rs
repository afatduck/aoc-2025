mod days;
pub mod utils;

fn main() {
    let input_result = std::fs::read_to_string("input.txt");

    match input_result {
        Ok(input) => {
            days::day4p2::solve(input);
        },
        Err(_) => {
            println!("Error reading file...")
        }
    }

}
