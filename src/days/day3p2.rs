#[allow(dead_code)]
pub fn solve(input: String) {

    let banks = input.split_ascii_whitespace();

    let mut result: u64 = 0;

    for bank in banks {
        let batteries = bank.chars().collect::<Vec<char>>();

        let mut final_comb: Vec<char> = Vec::new();
        let mut largest = '1';
        let mut largest_index = 0;

        for k in 0..12usize {
            for i in largest_index..batteries.len() - 11 + k {
                let battery = batteries[i];
                if battery > largest {
                    largest = battery;
                    largest_index = i + 1;
                }
            }
            final_comb.push(largest);
            largest = '1';
        }

        let best = final_comb.iter().collect::<String>();
        result += best.parse::<u64>().unwrap();
    }

    println!("{result}");

}