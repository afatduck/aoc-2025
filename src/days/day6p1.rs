#[allow(dead_code)]
pub fn solve(input: String) {

    let mut numbers = Vec::<u64>::new();
    // false for + true for *
    let mut opertaions = Vec::<bool>::new(); 
    for s in input.split_ascii_whitespace() {
        match s {
            "+" => {
                opertaions.push(false);
            },
            "*" => {
                opertaions.push(true);
            },
            _ => {
                numbers.push(s.parse().unwrap());
            }
        }
    }

    let cols = opertaions.len();
    let rows = numbers.len() / cols;
    let mut result = 0u64;

    for i in 0..cols {
        let multiply = opertaions[i];
        let mut acc = if multiply { 1u64 } else { 0u64 };
        for j in 0..rows {
            let num = numbers[j*cols + i];
            match multiply {
                true => {
                    acc *= num;
                }
                false => {
                    acc += num;
                }
            }
        }
        result += acc;
    } 

    println!("{result}");

}