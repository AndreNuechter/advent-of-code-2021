use std::collections::HashMap;

struct ClosingParen {
    opening_paren: String,
    error_score: u32
}

pub fn day_ten(input: &str) {
    let chunks: Vec<Vec<&str>> = input
        .lines()
        .map(|chunk| chunk
            .split("")
            .filter(|p| p != &"")
            .collect::<Vec<&str>>())
        .collect();
    let incomplete_chunks = part_one(&chunks);
    part_two(&incomplete_chunks);
}

fn part_two(incomplete_chunks: &Vec<Vec<&str>>) {
    println!("pt 2:");

    let parens = HashMap::from([
        ("(", 1),
        ("[", 2),
        ("{", 3),
        ("<", 4),
    ]);
    let middle_index = (incomplete_chunks.len() - 1) / 2;
    let mut completion_scores: Vec<u64> = incomplete_chunks
        .iter()
        .map(|chunk| {
            chunk
                .iter()
                .rev()
                .fold(0, |score, paren| score * 5 + parens.get(paren).unwrap())
        })
        .collect();

    completion_scores.sort();
    println!("completion score: {}", completion_scores[middle_index]);
}

fn part_one<'a>(chunks: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    println!("pt 1:");

    let parens = HashMap::from([
        (")", ClosingParen { opening_paren: String::from("("), error_score: 3 }),
        ("]", ClosingParen { opening_paren: String::from("["), error_score: 57 }),
        ("}", ClosingParen { opening_paren: String::from("{"), error_score: 1197 }),
        (">", ClosingParen { opening_paren: String::from("<"), error_score: 25137 }),
    ]);
    let mut score = 0;
    let mut incomplete_chunks: Vec<Vec<&str>> = Vec::new();

    for chunk in chunks.iter() {
        let mut corrupted = false;
        let mut opened: Vec<&str> = Vec::new();

        for paren in chunk.iter() {
            if parens.get(paren).is_none() {
                opened.push(paren);
            } else {
                let left = opened.pop();
                let config = parens.get(paren).unwrap();

                if left.is_none() || config.opening_paren != left.unwrap() {
                    score += config.error_score;
                    corrupted = true;
                    break;
                }
            }
        }

        if !corrupted {
            incomplete_chunks.push(opened);
        }
    }

    println!("error score: {}", score);

    incomplete_chunks
}