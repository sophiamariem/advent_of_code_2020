use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("err"))
        .collect();

   	fix_expenses_two(&numbers);
   	fix_expenses_three(&numbers);
}

fn fix_expenses_two(numbers: &Vec<usize>) -> usize {
    let mut complements: HashSet<usize> = HashSet::new();
    for i in 0..numbers.len() {
        let current = numbers[i];
        let complement = 2020 - current;
        if complements.contains(&complement) {
        	let result = current * complement;
            println!("Two number product: {}", result);
            return result;
        }
        complements.insert(current);
    }
    panic!("panic - two!");
}

fn fix_expenses_three(numbers: &Vec<usize>) -> usize {
 
    for i in 0..numbers.len() {
        let current = numbers[i];
        let other = 2020 - current;

        let mut complements: HashSet<usize> = HashSet::new();
        for j in i+1..numbers.len() {
         	let num = numbers[j];
         	let complement = other.wrapping_sub(num);
        	if complements.contains(&complement) {
        		let result = current * num * complement;
            	println!("Three number product: {}", result);
            	return result;
        	}
        	complements.insert(num);
        }
    }
    panic!("panic - three!");
}
