pub fn solve(input: String) {

    let mut numbers = Vec::<Vec<char>>::new();
    // false for + true for *
    let mut operations = Vec::<bool>::new();

    let mut col_widths: Vec<usize> = Vec::new();

    let mut lines: Vec<&str> = input.split('\n').collect();

    let opeartions_line = lines.pop().unwrap();
    {
        let mut w = 0usize;
        let mut chars = opeartions_line.chars();
        operations.push(chars.next().unwrap() == '*');
        for c in chars {
            if c != ' ' {
                operations.push(c == '*');
                col_widths.push(w);
                w = 0;
                continue;
            }
            w += 1; 
        }
        col_widths.push(w + 1);
    }

    for line in lines {
        let mut acc = 0usize;
        for w in &col_widths {
            let slice: String = line[acc..acc+w].into();
            numbers.push(
                slice.chars().collect()
            );
            acc += w + 1;
        }
    }

    let cols = operations.len();
    let rows = numbers.len() / cols;

    let mut result = 0u64;
    for i in (0..cols).rev() {
        let multiply = operations[i];
        let mut acc = if multiply { 1u64 } else { 0u64 };

        for k in (0..col_widths[i]).rev() {
            let mut n_chars = Vec::<char>::new();
            for j in 0..rows {
                let num = &numbers[cols * j + i];
                // println!("{num:?} {k}");
                let digit = num[k];
                n_chars.push(digit);
            }
            let n_str: String = n_chars.iter().collect();
            let n_res = n_str.replace(" ", "").parse::<u64>();

            match n_res {
                Ok(n) => {
                    match multiply {
                        true => {
                            acc *= n;
                        },
                        false => {
                            acc += n;
                        }
                    }
                },
                Err(_) => {}
            }
            
        }
        result += acc;
    }

    println!("{result}");

}