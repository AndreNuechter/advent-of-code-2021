pub fn day_eleven(input: &str) {
    let mut octopuses: Vec<Vec<u8>> = input
        .lines()
        .map(|row| {
            row.split("")
                .filter(|c| c != &"")
                .map(|c| c.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();
    part_one(&mut octopuses.clone());
    part_two(&mut octopuses);
}

fn part_two(octopuses: &mut Vec<Vec<u8>>) {
    println!("pt 2:");
    let octopus_count = octopuses.len() * octopuses[0].len();
    let mut steps = 0;

    loop {
        let (octopuses_after_step, flashed) = step_up(octopuses);
        *octopuses = octopuses_after_step;
        steps += 1;

        if flashed == octopus_count {
            break;
        }
    }

    println!("1st synchronized flash @ step {}", steps);
}

fn part_one(octopuses: &mut Vec<Vec<u8>>) {
    println!("pt 1:");

    let mut flashes = 0;

    for _ in 0..100 {
        let (octopuses_after_step, flashed) = step_up(octopuses);
        *octopuses = octopuses_after_step;
        flashes += flashed;
    }

    println!("# of flashes: {}", flashes);
}

fn step_up(octopuses: &mut Vec<Vec<u8>>) -> (Vec<Vec<u8>>, usize) {
    let mut flashers: Vec<String> = Vec::new();
    let mut octopuses: Vec<Vec<u8>> = octopuses
        .iter()
        .map(|row| row.iter().map(|octopus| octopus + 1).collect())
        .collect();

    let mut next_steppers: Vec<(usize, usize)> = Vec::new();

    for (pos_y, row) in octopuses.iter().enumerate() {
        for (pos_x, octopus) in row.iter().enumerate() {
            if *octopus > 9 {
                flashers.push(format!("{},{}", pos_y, pos_x));
                next_steppers.append(&mut get_neighbours(&octopuses, pos_y, pos_x));
            }
        }
    }

    while let Some((pos_y, pos_x)) = next_steppers.pop() {
        octopuses[pos_y][pos_x] += 1;

        if octopuses[pos_y][pos_x] > 9 {
            let pos_string = format!("{},{}", pos_y, pos_x);

            if !flashers.contains(&pos_string) {
                flashers.push(pos_string);
                next_steppers.append(&mut get_neighbours(&octopuses, pos_y, pos_x));
            }
        }
    }

    flashers.iter().for_each(|flasher| {
        let positions: Vec<usize> = flasher
            .split(",")
            .map(|pos| pos.parse::<usize>().unwrap())
            .collect();
        octopuses[positions[0]][positions[1]] = 0;
    });

    (octopuses, flashers.len())
}

fn get_neighbours(octopuses: &Vec<Vec<u8>>, pos_y: usize, pos_x: usize) -> Vec<(usize, usize)> {
    let row_count = octopuses.len();
    let row_length = octopuses[0].len();
    let mut neighbours = Vec::new();

    // NOTE: going around clockwise, starting top-left
    if pos_y > 0 && pos_x > 0 {
        neighbours.push((pos_y - 1, pos_x - 1));
    }

    if pos_y > 0 {
        neighbours.push((pos_y - 1, pos_x));
    }

    if pos_y > 0 && pos_x < (row_length - 1) {
        neighbours.push((pos_y - 1, pos_x + 1));
    }

    if pos_x < (row_length - 1) {
        neighbours.push((pos_y, pos_x + 1));
    }

    if pos_x < (row_length - 1) && pos_y < (row_count - 1) {
        neighbours.push((pos_y + 1, pos_x + 1));
    }

    if pos_y < (row_count - 1) {
        neighbours.push((pos_y + 1, pos_x));
    }

    if pos_x > 0 && pos_y < (row_count - 1) {
        neighbours.push((pos_y + 1, pos_x - 1));
    }

    if pos_x > 0 {
        neighbours.push((pos_y, pos_x - 1));
    }

    neighbours
}
