use std::fs;
use std::fmt::Debug;
use std::collections::HashSet;

#[derive(PartialEq, Clone, Copy, Debug, Eq, Hash)] 
enum Item {
    Name(char)
}

#[derive(PartialEq, Clone, Debug)] 
struct Rucksack {
    left: HashSet<Item>,
    right: HashSet<Item>
}

fn priority(item: Item) -> u32 {
    let number = match item {
        Item::Name(chr) => chr as u32
    };

    match number {
        n if n >= 65 && n <= 90 => n - 38,
        n if n >= 97 && n <= 122 => n - 96,
        _ => todo!()
    }
}

fn read_rucksack(line: &str) -> Rucksack {
    let length = line.len();
    let (first_half, second_half) = line.split_at(length / 2);
    Rucksack {
        left: first_half.chars().map(|c| Item::Name(c)).collect(),
        right: second_half.chars().map(|c| Item::Name(c)).collect(),
    }

}

fn main() {
    let file = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines = file.split("\n");

    pretty_test(vec!['a','z','A','Z'].iter().map(|c| Item::Name(*c)).collect(), priority);

    let total_priority: u32 = lines.fold(0, |acc, line| {
        let rucksack = read_rucksack(line);
        let mut intersection = rucksack.left.intersection(&rucksack.right);
        let common_item: Item = *intersection.next().unwrap();
        dbg!(priority(common_item));


        acc + priority(common_item)
    });

    println!("Summed priorities: {}", total_priority);
}

fn pretty_test<Input: Debug, Output: Debug, Function>(elems: Vec<Input>, f: Function) -> () 
    where Function: Fn(Input) -> Output {
    for elem in elems {
        {
            print!("f({:?}) => ", elem);
        }
        {
            println!("{:?}", f(elem));
        }
    }
}