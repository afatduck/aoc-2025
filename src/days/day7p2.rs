pub fn solve(input: String) {
    let mut lines = input.split_ascii_whitespace();

    let first = lines.next().unwrap();

    let width = first.len();
    let mut beams = vec![0u64; width];
    let start_pos = first.find('S').unwrap();
    beams[start_pos] = 1;

    lines.next();

    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        let mut new_beams = vec![0u64; width];
        for i in 0..width {
            let c = chars.next().unwrap();
            let beam_count = beams[i];
            if beam_count > 0  {
                if c == '^' {
                    new_beams[i-1] += beam_count;
                    new_beams[i+1] += beam_count;
                    continue;
                }
                new_beams[i] += beam_count;
            }
        }
        beams = new_beams;
        lines.next();
    }

    let result: u64 = beams.iter().sum();
    println!("{result}");

}