use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
--- Day 1: Report Repair ---

--- Part Two ---

The Elves in accounting are thankful for your help; one of them even offers you a starfish coin
they had left over from a past vacation. They offer you a second one if you can find three numbers
in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?

 */
fn main() {
    let mut candidates: HashMap<u32, u32> = HashMap::new();

    if let Ok(lines) = read_lines("./resources/input") {
        for line in lines {
            if let Ok(amount_as_string) = line {
                let value: u32 = match amount_as_string.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                candidates.insert(value, 2020 - value);
            }
        }

        'outer: for candidate in candidates.iter() {
            //println!("{:#?}", candidate);

            let rest = candidate.1;

            for c in candidates.iter() {
                if let Some(possible_val) = rest.checked_sub(*c.0) {
                    if candidates.contains_key(&possible_val) {
                        println!("{} -- {} -- {}", candidate.0, c.0, possible_val);
                        println!("{}", candidate.0 * c.0 * possible_val);
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}