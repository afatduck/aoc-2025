pub fn solve(input: String) {

    let mut split = input.split("\n\n");
    let ranges_split = split.next().unwrap().split_ascii_whitespace();
    let ids_split = split.next().unwrap().split_ascii_whitespace();

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for r_str in ranges_split {
        let mut split = r_str.split('-');
        let min: usize = split.next().unwrap().parse().unwrap();
        let max: usize = split.next().unwrap().parse().unwrap();
        ranges.push((min, max));
    }

    ranges.sort_by_key(|f| f.0);

    let mut result = 0u64;
    for id_str in ids_split {
        let id: usize = id_str.parse().unwrap();
        let first_ind = ranges.iter().position(|r| r.0 <= id).unwrap_or(ranges.len());
        for r in &ranges[first_ind..] {
            if r.0 > id {
                break;
            }
            if r.1 >= id {
                result += 1;
                break;
            }
        }
    }

    println!("{result}");

}