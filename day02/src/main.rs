use regex::Regex;

#[derive(Debug)]
struct Guess {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    guesses: Vec<Guess>,
}

impl Guess {
    fn from_str(s: &str) -> Self {
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;

        for cubes in s.split(", ") {
            let parts: Vec<&str> = cubes.split(" ").collect();
            let count = parts[0];
            let color = parts[1];

            match color {
                "red" => {
                    r = u32::from_str_radix(count, 10).unwrap();
                }
                "green" => {
                    g = u32::from_str_radix(count, 10).unwrap();
                }
                "blue" => {
                    b = u32::from_str_radix(count, 10).unwrap();
                }
                _ => {}
            }
        }

        Guess {
            red: r,
            green: g,
            blue: b,
        }
    }

    fn is_possible(&self) -> bool {
        const RED_MAX: u32 = 12;
        const GREEN_MAX: u32 = 13;
        const BLUE_MAX: u32 = 14;

        self.red <= RED_MAX && self.green <= GREEN_MAX && self.blue <= BLUE_MAX
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn from_str(line: &str) -> Self {
        let game_regex = Regex::new(r"Game (\d+)").unwrap();

        let parts: Vec<&str> = line.split(": ").into_iter().collect();
        let guesses_str = parts[1];
        let guesses: Vec<Guess> = guesses_str
            .split("; ")
            .map(|s| Guess::from_str(s))
            .collect();

        let m = game_regex.captures(parts[0]).unwrap();
        let game_id = u32::from_str_radix(&m[1], 10).unwrap();

        Game {
            id: game_id,
            guesses: guesses,
        }
    }

    fn is_possible(&self) -> bool {
        (&self.guesses).into_iter().all(|g| g.is_possible())
    }

    fn min_cubes(&self) -> Guess {
        let r_min = (&self.guesses).into_iter().map(|g| g.red).max();
        let g_min = (&self.guesses).into_iter().map(|g| g.green).max();
        let b_min = (&self.guesses).into_iter().map(|g| g.blue).max();

        Guess {
            red: r_min.unwrap(),
            green: g_min.unwrap(),
            blue: b_min.unwrap(),
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut id_sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in file.split("\n") {
        if line == "" {
            continue;
        }

        let game = Game::from_str(line);

        if game.is_possible() {
            id_sum += game.id;
        }

        power_sum += game.min_cubes().power();
    }

    println!("{}", id_sum);
    println!("{}", power_sum);
}
