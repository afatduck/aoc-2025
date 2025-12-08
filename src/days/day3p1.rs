pub fn solve(input: String) {

    let banks = input.split_ascii_whitespace();

    let mut result: u64 = 0;

    for bank in banks {
        let batteries = bank.chars().collect::<Vec<char>>();
        let mut largest = '1';
        let mut largest_index = 0;
        for i in 0..batteries.len() - 1 {
            let battery = batteries[i];
            if battery > largest {
                largest = battery;
                largest_index = i;
            }
        }

        let mut sec_largest = '1';

        for i in largest_index + 1..batteries.len() {
            let battery = batteries[i];
            if battery > sec_largest {
                sec_largest = battery;
            }
        }

        let best = [largest, sec_largest].iter().collect::<String>();
        result += best.parse::<u64>().unwrap();
    }

    println!("{result}");

}