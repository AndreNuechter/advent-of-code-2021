#[derive(Debug)]
struct FoldingInstruction {
    axis: String,
    value: u16
}

pub fn day_thirteen(input: &str) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let coordinates: Vec<Vec<usize>> = input[0]
        .lines()
        .map(|line| line
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect())
        .collect();
    let width = coordinates.iter().map(|coords| coords[0]).max().unwrap();
    let height = coordinates.iter().map(|coords| coords[1]).max().unwrap();
    let mut transparent_sheet = vec![vec![0; width + 1]; height + 1];

    for coordinate in coordinates.iter() {
        transparent_sheet[coordinate[1]][coordinate[0]] = 1;
    }

    let folding_instructions: Vec<FoldingInstruction> = input[1]
        .lines()
        .map(|line| line
            .replace("fold along ", "")
            .split("=")
            .map(|s| String::from(s))
            .collect::<Vec<String>>())
        .map(|raw_instruction| FoldingInstruction {
            axis: raw_instruction[0].to_string(),
            value: raw_instruction[1].parse().unwrap()
        })
        .collect();

    part_one(&transparent_sheet, &folding_instructions);
    part_two(&transparent_sheet, &folding_instructions);
}

fn part_two(transparent_sheet: &Vec<Vec<usize>>, folding_instructions: &Vec<FoldingInstruction>) {
    println!("pt 2:");

    let mut result = transparent_sheet;
    // NOTE: without this temporary vector, which is never read, we couldnt persist changes between loops
    // as we cant move the return of combine_sheets
    #[allow(unused_assignments)]
    let mut temp = Vec::new();

    for folding_instruction in folding_instructions.iter() {
        let (a, b) = split_transparent_sheet(&result, &folding_instruction);
        temp = combine_sheets(&a, &mirror_sheet(&b, &folding_instruction.axis));
        result = &temp;
    }

    print_sheet(&result);
}

fn part_one(transparent_sheet: &Vec<Vec<usize>>, folding_instructions: &Vec<FoldingInstruction>) {
    println!("pt 1:");

    let (a, b) = split_transparent_sheet(transparent_sheet, &folding_instructions[0]);
    let result = combine_sheets(&a, &mirror_sheet(&b, &folding_instructions[0].axis));

    println!("{:?}", result
        .iter()
        .fold(0 as usize, |count, row| count + row
            .iter()
            .filter(|cell| *cell > &0)
            .collect::<Vec<&usize>>()
            .len())
        );
}

fn split_transparent_sheet(
    transparent_sheet: &Vec<Vec<usize>>, folding_instruction: &FoldingInstruction
) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let value = folding_instruction.value as usize;

    if folding_instruction.axis == "y" {
        (
            transparent_sheet[0..value].to_vec(),
            transparent_sheet[value + 1..].to_vec()
        )
    } else {
        (
            transparent_sheet.iter().map(|row| row[0..value].to_vec()).collect(),
            transparent_sheet.iter().map(|row| row[value + 1..].to_vec()).collect(),
        )
    }
}

fn combine_sheets(sheet_a: &Vec<Vec<usize>>, sheet_b: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    sheet_a
        .iter()
        .enumerate()
        .map(|(y_pos, row)| row
            .iter()
            .enumerate()
            .map(|(x_pos, cell)| if cell + sheet_b[y_pos][x_pos] > 0 { 1 } else { 0 })
            .collect())
        .collect()
}

fn mirror_sheet(sheet: &Vec<Vec<usize>>, axis: &str) -> Vec<Vec<usize>> {
    let mut sheet = sheet.clone();

    if axis == "y" {
        sheet.reverse();
    } else {
        sheet = sheet
            .into_iter()
            .map(|row| {
                let mut row = row.clone();
                row.reverse();
                row
            })
            .collect();
    }

    sheet
}

fn print_sheet(sheet: &Vec<Vec<usize>>) {
    for row in sheet.iter().map(|row| row
        .iter()
        .map(|cell| cell.to_string().replace("0", "."))
        .collect::<String>()) {
        println!("{:?}", row);
    }
}