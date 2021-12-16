pub fn day_three(input: &str) {
    let report = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|p| *p != "")
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    part_one(&report);
    part_two(report);
}

fn part_one(report: &Vec<Vec<u32>>) {
    let entry_count = report.len() as u32;
    let element_count = report[0].len();
    let inverted_report: Vec<u32> = (0..element_count)
        .map(|index| report.iter().map(|entry| entry[index]).sum::<u32>())
        .collect();
    let gamma = get_rating(&inverted_report, &entry_count, "1", "0");
    let epsilon = get_rating(&inverted_report, &entry_count, "0", "1");

    println!(
        "pt 1: {}",
        binary_string_to_base10(&gamma) * binary_string_to_base10(&epsilon)
    );
}

fn get_rating(
    inverted_report: &Vec<u32>,
    entry_count: &u32,
    val_one: &str,
    val_two: &str,
) -> String {
    inverted_report
        .iter()
        .map(|sum| {
            if *sum > entry_count / 2 {
                val_one
            } else {
                val_two
            }
        })
        .collect()
}

fn part_two(report: Vec<Vec<u32>>) {
    let oxygen_rating = binary_string_to_base10(&get_tricky_rating(report.clone(), "1", "0"));
    let co2_rating = binary_string_to_base10(&get_tricky_rating(report.clone(), "0", "1"));

    println!(
        "pt 2: {} x {} = {}",
        oxygen_rating,
        co2_rating,
        oxygen_rating * co2_rating
    );
}

fn get_tricky_rating(mut report: Vec<Vec<u32>>, val_one: &str, val_two: &str) -> String {
    let mut index = 0;
    while report.len() > 1 {
        let entry_count = report.len() as f64;
        let inverted_column_sum: u32 = report.iter().map(|row| row[index]).sum();
        let token = if inverted_column_sum as f64 >= (entry_count / 2.0)
            || inverted_column_sum == 1 && entry_count == 2.0
        {
            val_one
        } else {
            val_two
        };
        report = report
            .into_iter()
            .filter(|row| row[index].to_string() == token)
            .collect();
        index += 1;
    }

    report[0].iter().map(|v| v.to_string()).collect()
}

fn binary_string_to_base10(string: &str) -> isize {
    isize::from_str_radix(&string, 2).unwrap()
}
