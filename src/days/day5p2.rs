#[allow(dead_code)]
pub fn solve(input: String) {

    let mut split = input.split("\n\n");
    let ranges_split = split.next().unwrap().split_ascii_whitespace();

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for r_str in ranges_split {
        let mut split = r_str.split('-');
        let min: usize = split.next().unwrap().parse().unwrap();
        let max: usize = split.next().unwrap().parse().unwrap();
        ranges.push((min, max));
    }

    ranges.sort_by_key(|f| f.0);

    let mut result = 0usize;
    let mut last_processed = 0;
    for range_ind in 0..ranges.len() {
        let range = ranges[range_ind];
        if range.1 > last_processed {
            let delta = range.1 - std::cmp::max(last_processed, range.0 - 1);
            result += delta;
            last_processed = range.1;
        }
    }

    println!("{result}");

}