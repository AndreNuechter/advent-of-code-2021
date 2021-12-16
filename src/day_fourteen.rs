use std::collections::HashMap;

pub fn day_fourteen(input: &str) {
    let (polymer_template, insertion_rules) = input.split_once("\n\n").unwrap();
    let insertion_rules: HashMap<_, _> = insertion_rules
        .split("\n")
        .map(|row| row.split_once(" -> ").unwrap())
        .collect();

    part_one(polymer_template, &insertion_rules);
    part_two(polymer_template, &insertion_rules);
}

fn part_two(polymer_template: &str, insertion_rules: &HashMap<&str, &str>) {
    println!("pt 2:");
    // the idea is that by counting pairs, we bound the memory usage,
    // which would grow exponentially when done as for pt 1.
    // so instead of just growing a String, we use a HashMap of counts of pairs.
    // the key insight is that each step of polymerase on a pair (a, b), adds one new char x and
    // thereby creates two new pairs((a, x), (x, b)), while destroying the initial one (a, b).
    // and in a cycle this happens as many times as pair (a, b) has been counted thus far

    let mut pairs: HashMap<[char; 2], i64> = HashMap::new();
    let mut letter_counts: HashMap<char, i64> = HashMap::new();

    for offset in 0..polymer_template.len() - 1 {
        let mut chars = polymer_template[offset..offset + 2].chars();
        *pairs
            .entry([chars.next().unwrap(), chars.next().unwrap()])
            .or_insert(0) += 1;
    }

    for character in polymer_template.chars() {
        *letter_counts.entry(character).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut changes = Vec::new();

        for ([left, right], current_count) in &pairs {
            let key_str = &format!("{}{}", left, right)[0..];

            if *current_count == 0 {
                continue;
            } else {
                let new_char = insertion_rules
                    .get(key_str)
                    .unwrap()
                    .chars()
                    .nth(0)
                    .unwrap();

                *letter_counts.entry(new_char).or_insert(0) += current_count;
                changes.push(((*left, *right), -*current_count));
                changes.push(((*left, new_char), *current_count));
                changes.push(((new_char, *right), *current_count));
            }
        }

        for ((left, right), value) in changes.iter() {
            *pairs.entry([*left, *right]).or_insert(0) += value;
        }
    }

    let (min, max) = (
        letter_counts.values().min().unwrap(),
        letter_counts.values().max().unwrap(),
    );

    println!("{:?}", max - min);
}

fn part_one(polymer_template: &str, insertion_rules: &HashMap<&str, &str>) {
    println!("pt 1:");

    let mut polymer = String::from(polymer_template);

    for _ in 0..10 {
        polymer = polymerase(&polymer, &insertion_rules);
    }

    let letter_counts = count_letters(&polymer);
    let (min, max) = (
        letter_counts.values().min().unwrap(),
        letter_counts.values().max().unwrap(),
    );

    println!("{:?}", max - min);
}

fn count_letters(string: &str) -> HashMap<&str, u64> {
    string
        .split("")
        .filter(|letter| letter != &"")
        .fold(HashMap::new(), |mut result, letter| {
            *result.entry(letter).or_insert(0) += 1;
            result
        })
}

fn polymerase(polymer_template: &str, insertion_rules: &HashMap<&str, &str>) -> String {
    let mut result = String::from(polymer_template);
    let mut insertions = Vec::new();

    for offset in 0..polymer_template.len() - 1 {
        insertions.push((
            offset + 1,
            insertion_rules
                .get(&polymer_template[offset..offset + 2])
                .unwrap(),
        ))
    }

    for (position, value) in insertions.iter().rev() {
        result.insert_str(*position, value);
    }

    result
}
