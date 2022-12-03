use std::fs;

fn part1(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    return result;
}

fn part2(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut result: Vec<u32> = input
        .split("\n\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    result.sort_by(|a, b| b.cmp(a));
    let result: u32 = result[0..3].to_vec().iter().sum();
    return result;
}

fn main() {
    let file_path: &str = "input1.txt";
    let result = part1(file_path);
    println!("Part 1 Result: {}", result);
    let result = part2(file_path);
    println!("Part 2 Result: {}", result);
}
