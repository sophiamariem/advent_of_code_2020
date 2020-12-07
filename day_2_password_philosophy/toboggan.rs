use std::fs;

fn main() {
    let passwords = input();

    part_one(&passwords);
    part_two(&passwords);
}

fn input() -> Vec<Password> {
 let input = fs::read_to_string("./input.txt").unwrap();
 let passwords: Vec<Password> =
    input
        .lines()
        .map(|val| val.trim())
        .map(|raw| -> Password {
            let processed = raw.replace(':', "");
            let mut components = processed.split(" ");

            let range: Vec<usize> = components
                .next()
                .unwrap()
                .split("-")
                .map(|x| x.parse().unwrap())
                .collect();
            let character = components.next().unwrap().chars().next().unwrap();
            let password = components.next().unwrap().to_string();

            Password {
                min: range[0],
                max: range[1],
                character: character,
                password: password,
            }
        })
        .collect();

    return passwords;
}

fn part_one(passwords: &Vec<Password>) {
    let count = passwords.iter().filter(|password| password.is_valid_part_one()).count();
    println!("Valid count part one: {}", count);
}

fn part_two(passwords: &Vec<Password>) {
    let count = passwords.iter().filter(|password| password.is_valid_part_two()).count();
    println!("Valid count part two: {}", count);
}

struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl Password {
    fn is_valid_part_one(&self) -> bool {
        let count = self.password.matches(self.character).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_part_two(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        let first_pos = self.min - 1;
        let second_pos = self.max - 1;

        if chars[first_pos] == self.character && chars[second_pos] == self.character {
            return false;
        }
        return chars[first_pos] == self.character || chars[second_pos] == self.character;
    }
}

