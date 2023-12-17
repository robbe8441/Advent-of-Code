pub fn main() {
    let mut result = 0;
    let input = include_str!("./input.txt");
    
    for line in input.lines() {
        let line = line.split(":").last().unwrap();
        let mut split = line.split("|");

        let winning = sting_to_numbers(split.next().unwrap());
        let numbers = sting_to_numbers(split.last().unwrap());

        let mut winns = 0;

        for number in numbers {
            if winning.contains(&number) {
                if winns == 0 {winns += 1;} 
                else {winns *= 2;}
            }
        }
        result += winns;
    }
    println!("total winns : {}", result);
}


fn sting_to_numbers(input : &str) -> Vec<u32> {
    let mut output : Vec<u32> = vec![];

    for val in input.split_whitespace() {
        let num = val.parse();
        if num.is_ok() {
            output.push(num.unwrap())
        }
    }
    output
}