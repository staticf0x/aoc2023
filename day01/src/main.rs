use std::dbg;

fn replace_digits(line: &str) -> String {
    let digits_str = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut new_line = String::new();

    for i in 0..line.chars().count() {
        let c = line.chars().nth(i).unwrap();

        let mut replaced: bool = false;

        for (j, digit) in digits_str.iter().enumerate() {
            if line
                .chars()
                .skip(i)
                .take(digit.len())
                .map(|c| c.to_string())
                .collect::<String>()
                == *digit
            {
                new_line.push_str(format!("{}", j + 1).as_str());
                replaced = true;
                break;
            }
        }

        if !replaced {
            new_line.push(c);
        }
    }

    new_line
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut total: u32 = 0;

    for line in file.split("\n") {
        let new_line = replace_digits(line);

        let digits: Vec<char> = new_line.chars().filter(|c| c.is_digit(10)).collect();

        if digits.len() >= 1 {
            let number: u32 = u32::from_str_radix(
                format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).as_str(),
                10,
            )
            .unwrap();

            total += number;
        }
    }

    println!("{}", total);
}
