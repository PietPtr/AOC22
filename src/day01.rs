use std::fs;


fn main() {

    let file = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines: Vec<&str> = file.split("\n").collect();

    let mut calorie_sums: Vec<usize> = Vec::new();

    let mut current_elf_sum = 0;

    for line in &lines {
        match line.parse::<usize>() {
            Err(_) => {
                println!("---");
                calorie_sums.push(current_elf_sum);
                current_elf_sum = 0;
            },
            Ok(value) => {
                current_elf_sum += value;
                println!("{current_elf_sum}");
            },
        }
    }

    calorie_sums.sort();
    calorie_sums.reverse();

    let top3 = calorie_sums[0..3].iter().fold(0, |a, b| a + b);

    println!("Top 3 with most calories have {top3} calories {:?}", &calorie_sums[0..3]);
}
