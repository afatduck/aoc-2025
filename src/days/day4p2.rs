use crate::utils::matrix::Matrix;

#[allow(dead_code)]
pub fn solve(input: String) {

    let m = input.find('\n').unwrap();
    let n = input.len() / m;

    let mut sheet = Matrix::from_default(false, n, m);

    let binding = input.replace("\n", "");
    let mut chars = binding.chars();

    for i in 0..n {
        for j in 0..m {
            let c = chars.next().unwrap();
            if c == '@' {
                let _ = sheet.set(i, j, true);
            }
        }
    }

    let mut result = 0u64;
    let mut changed = true;
    while changed {
        changed = false;
        let mut new_sheet = sheet.clone();
        for i in 0..n {
            for j in 0..m {
                let mut adj = 0u8;

                if !sheet.get(i, j).unwrap() { continue; }

                for dx in 0..3 {
                    for dy in 0..3 {
                        if dx == 1 && dy == 1 || i + dx == 0 || j + dy == 0 { continue; }
                        match sheet.get(i + dx - 1, j + dy - 1) {
                            Ok(roll) => {
                                if *roll { 
                                    adj += 1;
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }

                if adj < 4 {
                    result += 1;
                    changed = true;
                    let _ = new_sheet.set(i, j, false);
                }

            }
        }
        sheet = new_sheet;
    } 

    println!("{result:?}")

}