pub fn solve(input: String) {
    let mut lines = input.split_ascii_whitespace();

    let first = lines.next().unwrap();

    let width = first.len();
    let mut beams = vec![false; width];
    let start_pos = first.find('S').unwrap();
    beams[start_pos] = true;

    lines.next();

    let mut result = 0u64;
    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        let mut new_beams = vec![false; width];
        for i in 0..width {
            let c = chars.next().unwrap();
            if beams[i] {
                if c == '^' {
                    result += 1;
                    new_beams[i-1] = true;
                    new_beams[i+1] = true;
                    continue;
                }
                new_beams[i] = true;
            }
        }
        beams = new_beams;
        lines.next();
    }

    println!("{result}");
}