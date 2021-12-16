struct ThreeDPoint {
    x: u16,
    y: u16,
    z: u16,
}

pub fn day_nine(input: &str) {
    let rows: Vec<Vec<ThreeDPoint>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.split("")
                .filter(|num| num != &"")
                .enumerate()
                .map(|(x, num)| ThreeDPoint {
                    x: x as u16,
                    y: y as u16,
                    z: num.parse::<u16>().unwrap(),
                })
                .collect::<Vec<ThreeDPoint>>()
        })
        .collect();
    let lowpoints = get_lowpoints(&rows);

    part_one(&lowpoints);
    part_two(&lowpoints, &rows);
}

fn part_two(lowpoints: &Vec<&ThreeDPoint>, data_rows: &Vec<Vec<ThreeDPoint>>) {
    println!("pt 2");

    let mut basin_sizes: Vec<u32> = lowpoints
        .iter()
        .map(|lowpoint| get_basinsize(&lowpoint, &data_rows))
        .collect();

    basin_sizes.sort_by(|a, b| b.cmp(a));

    println!("{:?}", basin_sizes[0..3].iter().product::<u32>());
}

fn part_one(lowpoints: &Vec<&ThreeDPoint>) {
    println!("pt 1");
    println!(
        "{:?}",
        lowpoints.iter().map(|lowpoint| lowpoint.z + 1).sum::<u16>()
    );
}

fn get_basinsize(lowpoint: &ThreeDPoint, data_rows: &Vec<Vec<ThreeDPoint>>) -> u32 {
    let row_count = data_rows.len();
    let row_length = data_rows[0].len();
    let mut size = 0;
    let mut visited: Vec<String> = Vec::new();
    let mut queue = vec![ThreeDPoint {
        x: lowpoint.x,
        y: lowpoint.y,
        z: lowpoint.z,
    }];

    while queue.len() > 0 {
        let current = queue.remove(0);

        if current.z == 9 {
            continue;
        }

        let curr_id = format!("{},{}", current.x, current.y);

        if visited.contains(&curr_id) {
            continue;
        }

        size += 1;
        visited.push(curr_id);
        queue.append(&mut get_neighbours(
            &data_rows, row_count, row_length, &current,
        ));
    }

    size
}

fn get_lowpoints(data_rows: &Vec<Vec<ThreeDPoint>>) -> Vec<&ThreeDPoint> {
    let row_count = data_rows.len();
    let row_length = data_rows[0].len();
    let mut lowpoints: Vec<&ThreeDPoint> = Vec::new();

    for row in data_rows.iter() {
        for cell in row.iter() {
            if get_neighbours(&data_rows, row_count, row_length, &cell)
                .iter()
                .all(|n| cell.z < n.z)
            {
                lowpoints.push(cell);
            }
        }
    }

    lowpoints
}

fn get_neighbours(
    data_rows: &&Vec<Vec<ThreeDPoint>>,
    row_count: usize,
    row_length: usize,
    cell: &ThreeDPoint,
) -> Vec<ThreeDPoint> {
    let mut neighbours = Vec::new();

    if cell.y > 0 {
        neighbours.push(ThreeDPoint {
            x: cell.x,
            y: cell.y - 1,
            z: data_rows[cell.y as usize - 1][cell.x as usize].z,
        });
    }

    if cell.y < (row_count - 1) as u16 {
        neighbours.push(ThreeDPoint {
            x: cell.x,
            y: cell.y + 1,
            z: data_rows[cell.y as usize + 1][cell.x as usize].z,
        });
    }

    if cell.x > 0 {
        neighbours.push(ThreeDPoint {
            x: (cell.x - 1) as u16,
            y: cell.y as u16,
            z: data_rows[cell.y as usize][cell.x as usize - 1].z,
        });
    }

    if cell.x < (row_length - 1) as u16 {
        neighbours.push(ThreeDPoint {
            x: cell.x + 1,
            y: cell.y,
            z: data_rows[cell.y as usize][cell.x as usize + 1].z,
        });
    }

    neighbours
}
