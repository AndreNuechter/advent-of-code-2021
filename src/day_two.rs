pub fn day_two(input: &str) {
    let commands = input
        .lines()
        .map(|c| {
            let mut split = c.split_whitespace();
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(&str, i32)>>();
    part_one(&commands);
    part_two(&commands);
}

fn part_one(commands: &Vec<(&str, i32)>) {
    let (horizontal_moves, vertical_moves): (Vec<(&str, i32)>, Vec<(&str, i32)>) =
        commands.iter().partition(|(cmd, _)| *cmd == "forward");
    let horizontal_moves: i32 = horizontal_moves.iter().map(|(_, val)| val).sum();
    let vertical_moves: i32 = vertical_moves
        .iter()
        .map(|(cmd, val)| if *cmd == "up" { val * -1 } else { *val })
        .sum();

    println!(
        "pt 1: {} x {} = {}",
        horizontal_moves,
        vertical_moves,
        horizontal_moves * vertical_moves
    );
}

fn part_two(commands: &Vec<(&str, i32)>) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (cmd, val) in commands.iter() {
        match *cmd {
            "down" => aim = down(*val, aim),
            "up" => aim = up(*val, aim),
            "forward" => {
                let (h, d) = forward(*val, horizontal_position, depth, aim);
                horizontal_position = h;
                depth = d;
            }
            _ => print!("nonono"),
        }
    }

    println!(
        "pt 1: {} x {} = {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );
}

fn down(val: i32, aim: i32) -> i32 {
    aim + val
}

fn up(val: i32, aim: i32) -> i32 {
    aim - val
}

fn forward(val: i32, horizontal_position: i32, depth: i32, aim: i32) -> (i32, i32) {
    (horizontal_position + val, depth + val * aim)
}
