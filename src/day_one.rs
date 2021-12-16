pub fn day_one(input: &str) {
    let measurements = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let increases = count_increases(&measurements);

    println!("pt 1: {}", increases);

    let windowed_measurements = create_windowed_measurements(&measurements);
    let windowed_increases = count_increases(&windowed_measurements);

    println!("pt 2: {}", windowed_increases);
}

fn create_windowed_measurements(measurements: &Vec<u32>) -> Vec<u32> {
    let measurement_count = measurements.len();
    let mut windowed_measurements = Vec::new();

    for index in 0..measurement_count {
        let next_step = index + 3;
        if next_step <= measurement_count {
            windowed_measurements.push(measurements[index..next_step].iter().sum());
        }
    }

    windowed_measurements
}

fn count_increases(measurements: &Vec<u32>) -> u32 {
    let mut increases = 0;
    let mut prev: Option<u32> = None;

    for measurement in measurements.iter() {
        prev = match prev {
            Some(val) => {
                if val < *measurement {
                    increases += 1;
                }
                Some(*measurement)
            }
            None => Some(*measurement),
        }
    }

    increases
}
