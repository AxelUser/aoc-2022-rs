fn main() {
    let mut input = parse(include_str!("in"));
    input.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", input[0]);
    println!("Part 2: {}", input.iter().take(3).sum::<i32>());
}

fn parse(input: &str) -> Vec<i32> {
    let mut items = vec![0];
    
    for line in input.lines() {
        if line == "" {
            items.push(0)
        } else {
            let sum = items.last_mut().expect("failed to get last value");
            *sum += line.parse::<i32>().expect("failed to parse")
        }
        
    }

    return items;
}
