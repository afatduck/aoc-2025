#[allow(dead_code)]
pub fn solve(input: String) {

    let ranges = input.split(",");
    let mut result = 0u64;

    for range in ranges {
        let mut split = range.split("-");
        let low: u64 = split.next().unwrap().parse().unwrap();
        let high: u64 = split.next().unwrap().parse().unwrap();

        // Accidentally went solving what was to be the second part because I didn't read the task properly.
        // Turned out pretty convenient.
        'id_loop: for i in low..high+1 {
            let num_str = i.to_string();
            let len = num_str.len();
            if len % 2 != 0 { continue; }
            for k in len/2..len / 2 + 1 {
                if len % k != 0 { continue; }
                let base = &num_str[0..k];
                for j in 1..len / k {
                    let next = &num_str[j*k..(j+1)*k];
                    if base != next {
                        continue 'id_loop;
                    }
                }
                println!("{low}-{high}: {i}");
                result += i;
                continue 'id_loop;
            }
        }
    }

    println!("{result}")
}