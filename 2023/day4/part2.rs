pub fn main() {
    let input = include_str!("./input.txt");
    let mut coppys: Vec<u32> = vec![0; input.lines().count() + 50];

    let mut card_id = 0;
    let mut result = input.lines().count() as u32;

    for line in input.lines() {
        card_id += 1;
        let line = line.split(":").last().unwrap();
        let mut split = line.split("|");

        let winning = sting_to_numbers(split.next().unwrap());
        let numbers = sting_to_numbers(split.last().unwrap());

        let mut winns = 0;

        for number in numbers {
            if winning.contains(&number) {
                winns += 1;
            }
        }

        for win in 0..winns {
            coppys[card_id + 1 + win as usize] += coppys[card_id] + 1;
        }
    }

    for v in coppys {
        result += v;
    }

    println!("total winns : {}", result);
}

fn sting_to_numbers(input: &str) -> Vec<u32> {
    let mut output: Vec<u32> = vec![];

    for val in input.split_whitespace() {
        let num = val.parse();
        if num.is_ok() {
            output.push(num.unwrap())
        }
    }
    output
}