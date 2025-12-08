pub fn solve(input: String) {

    let rotations = input.split_ascii_whitespace();

    let mut dial = 50i16;
    let mut result = 0;

    for rotation_str in rotations {
        let mut rotation = rotation_str[1..].parse::<i16>().unwrap() % 100;
        if rotation_str.chars().next().unwrap() == 'L' {
            rotation = -rotation;
        }
        dial += rotation;
        if dial < 0 {
            dial += 100;
        }
        if dial > 99 {
            dial -= 100;
        }
        if dial == 0 {
            result += 1;
        }
    }

    println!("{result}")
}