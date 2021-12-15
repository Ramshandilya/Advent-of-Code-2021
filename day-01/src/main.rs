fn main() {
    let values: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    println!("PART 1: {}", part_one(&values));
    println!("PART 2: {}", part_two(&values));

}

fn part_one(values: &Vec<u32>) -> usize {
    return values.windows(2).filter(|i| i.first() < i.last()).count()
}

fn part_two(values: &Vec<u32>) -> usize {
    return values.windows(4).filter(|i| i.first() < i.last()).count()
}