use std::fs;
fn main() {
    let values = read_file("day-01/input.txt");

    println!("PART 1: {}", part_one(&values));
    println!("PART 2: {}", part_two(&values));

}

fn read_file(filename: &str) -> Vec<u32> {
    let values: Vec<u32> = fs::read_to_string(filename)
        .expect("File not found")
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    values
}

fn part_one(values: &Vec<u32>) -> usize {
    return values.windows(2).filter(|i| i.first() < i.last()).count()
}

fn part_two(values: &Vec<u32>) -> usize {
    return values.windows(4).filter(|i| i.first() < i.last()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let values: Vec<u32> = read_file("test_input.txt");
        assert_eq!(part_one(&values), 7);
    }

    #[test]
    fn test_part_two() {
        let values: Vec<u32> = read_file("test_input.txt");
        assert_eq!(part_two(&values), 5);
    }
}