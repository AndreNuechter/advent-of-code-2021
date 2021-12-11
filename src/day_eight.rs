use std::fmt;

pub fn day_eight(input: &str) {
    let entries: Vec<Entry> = input.lines().map(|entry_str| {
        let mut split = entry_str.split(" | ");
        let examples: Vec<&str> = split.next().unwrap().split(" ").collect();
        let wiring_guide = figure_out_wiring(&examples);
        let output_value: [SevenSegmentDisplay; 4] = split
            .next()
            .unwrap()
            .split(" ")
            .map(|string| descramble_value(string, &wiring_guide))
            .collect::<Vec<SevenSegmentDisplay>>()
            .try_into()
            .unwrap();
        Entry { output_value }
    }).collect();

    part_one(&entries);
    part_two(&entries);
}

fn part_two(entries: &Vec<Entry>) {
    println!("pt 2");

    let result: u32 = entries
        .iter()
        .map(|entry| {
            entry.output_value
                .iter()
                .map(|seven_segment_display| seven_segment_display.numerical_value().to_string())
                .collect::<String>()})
        .map(|string_value| {
            println!("{}", string_value);
            string_value.parse::<u32>().unwrap()
        })
        .sum();

    println!("sum of outputs: {}", result);
}

fn part_one(entries: &Vec<Entry>) {
    println!("pt 1");

    let unique_lenghts = [2, 4, 3, 7];
    let values = entries
        .iter()
        .fold(
            Vec::new(),
            |mut e, entry| {
                e.extend_from_slice(&entry.output_value
                    .iter()
                    .map(|seven_segment_display| seven_segment_display.segment_count())
                    .filter(|segment_count| unique_lenghts.contains(segment_count))
                    .collect::<Vec<u8>>());
                e
            }
        );
    println!("number of unique length values: {:?}", values.len());
}

fn descramble_value(segment_str: &str, wiring_guide: &[&str; 7]) -> SevenSegmentDisplay {
    let mut value = [false; 7];

    for scrambled_letter in segment_str.split("").filter(|l| l != &"") {
        value[wiring_guide
        .iter()
        .position(|descrambled_letter| &scrambled_letter == descrambled_letter)
        .unwrap() as usize] = true;
    }

    SevenSegmentDisplay { value }
}

fn figure_out_wiring<'a>(examples: &'a Vec<&str>) -> [&'a str; 7] {
    let mut wiring_guide = [""; 7];

    // println!("if an example has length 2, it's 1");
    assert!(examples
        .iter()
        .filter(|example| example.len() == 2)
        .collect::<Vec<&&str>>()
        .len() == 1
    );
    let one = examples
        .iter()
        .find(|example| example.len() == 2)
        .unwrap();
    // println!("one {}", one);
    // println!("if an example has length 3, it's 7");
    assert!(examples
        .iter()
        .filter(|example| example.len() == 3)
        .collect::<Vec<&&str>>()
        .len() == 1
    );
    let seven = examples
        .iter()
        .find(|example| example.len() == 3)
        .unwrap();
    // println!("seven {}", seven);
    // println!("the letter used in 7 but not in 1 is a/0");
    assert!(seven
        .split("")
        .filter(|letter| letter != &"")
        .filter(|letter| !one.contains(letter))
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[0] = seven
        .split("")
        .filter(|letter| letter != &"")
        .find(|letter| !one.contains(letter))
        .unwrap();
    // println!("result: {:?}", result);
    // println!("if an example has length 4, it's 4");
    assert!(examples
        .iter()
        .filter(|example| example.len() == 4)
        .collect::<Vec<&&str>>()
        .len() == 1
    );
    let four = examples
        .iter()
        .find(|example| example.len() == 4)
        .unwrap();
    // println!("four {}", four);
    // println!("if an example has length 5, it's 2, 3 or 5");
    assert!(examples
        .iter()
        .filter(|example| example.len() == 5)
        .collect::<Vec<&&str>>()
        .len() == 3
    );
    let five_segment_numbers: Vec<&&str> = examples
        .iter()
        .filter(|example| example.len() == 5)
        .collect();
    // println!("3 is the only five segment number that uses both parts of 1");
    assert!(five_segment_numbers
        .iter()
        .filter(|example| one
            .split("")
            .filter(|letter| letter != &"")
            .all(|letter| example.contains(letter))
        ).collect::<Vec<&&&str>>()
        .len() == 1
    );
    let three = five_segment_numbers
        .iter()
        .find(|example| one
            .split("")
            .filter(|letter| letter != &"")
            .all(|letter| example.contains(letter))
        ).unwrap();
    // println!("three {}", three);
    // println!("the segment 4 shares with 3, but not with 1, is d/3");
    assert!(four
        .split("")
        .filter(|letter| letter != &"")
        .filter(|letter| three.contains(letter) && !one.contains(letter))
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[3] = four
        .split("")
        .filter(|letter| letter != &"")
        .find(|letter| three.contains(letter) && !one.contains(letter))
        .unwrap();
    // println!("result: {:?}", result);
    // println!("the segment 4 shares with neither 3, nor 1, is b/1");
    assert!(four
        .split("")
        .filter(|letter| letter != &"")
        .filter(|letter| !one.contains(letter) && &wiring_guide[3] != letter)
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[1] = four
        .split("")
        .filter(|letter| letter != &"")
        .find(|letter| !one.contains(letter) && &wiring_guide[3] != letter)
        .unwrap();
    // println!("result: {:?}", result);
    // println!("the only five segment number using b/1 is 5");
    assert!(five_segment_numbers
        .iter()
        .filter(|example| example.contains(wiring_guide[1]))
        .collect::<Vec<&&&str>>()
        .len() == 1
    );
    let five = five_segment_numbers
        .iter()
        .find(|example| example.contains(wiring_guide[1]))
        .unwrap();
    // println!("five {}", five);
    // println!("the only segment 5 shares with 1 is f/5");
    assert!(five
        .split("")
        .filter(|letter| letter != &"")
        .filter(|letter| one.contains(letter))
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[5] = five
        .split("")
        .filter(|letter| letter != &"")
        .find(|letter| one.contains(letter))
        .unwrap();
    // println!("result: {:?}", result);
    // println!("so we know the other segment used in 1 is c/2");
    assert!(one
        .split("")
        .filter(|letter| letter != &"")
        .filter(|letter| letter != &wiring_guide[5])
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[2] = one
        .split("")
        .filter(|letter| letter != &"")
        .find(|letter| letter != &wiring_guide[5])
        .unwrap();
    // println!("result: {:?}", result);
    // println!("the thus far undisclosed segment of 3, not shared with 1, is g/6");
    assert!(three
        .split("")
        .filter(|letter| letter != &"")
        .filter(|letter| !one.contains(letter) && ![wiring_guide[0], wiring_guide[3]].contains(letter))
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[6] = three
        .split("")
        .filter(|letter| letter != &"")
        .find(|letter| !one.contains(letter) && ![wiring_guide[0], wiring_guide[3]].contains(letter))
        .unwrap();
    // println!("result: {:?}", result);
    // println!("now there's only one possibility left");
    assert!("a,b,c,d,e,f,g"
        .split(",")
        .filter(|letter| !wiring_guide.contains(letter))
        .collect::<Vec<&str>>()
        .len() == 1
    );
    wiring_guide[4] = "a,b,c,d,e,f,g"
        .split(",")
        .find(|letter| !wiring_guide.contains(letter))
        .unwrap();
    println!("result: {:?}", wiring_guide);

    wiring_guide
}

#[derive(Copy, Clone, PartialEq)]
struct SevenSegmentDisplay {
    value: [bool; 7]
}

impl From<u8> for SevenSegmentDisplay {
    fn from(number: u8) -> Self {
        let value = match number {
            1 => [false, false, true, false, false, true, false],
            2 => [true, false, true, true, true, false, true],
            3 => [true, false, true, true, false, true, true],
            4 => [false, true, true, true, false, true, false],
            5 => [true, true, false, true, false, true, true],
            6 => [true, true, false, true, true, true, true],
            7 => [true, false, true, false, false, true, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            _ => [true, true, true, false, true, true, true]
        };
        SevenSegmentDisplay{ value }
    }
}

impl From<&str> for SevenSegmentDisplay {
    fn from(string: &str) -> Self {
        let mut value = [false; 7];

        fn letter_to_index(letter: &str) -> usize {
            match letter {
                "a" => 0,
                "b" => 1,
                "c" => 2,
                "d" => 3,
                "e" => 4,
                "f" => 5,
                "g" => 6,
                _ => 99
            }
        }
    
        for letter in string.split("").filter(|letter| *letter != "") {
            value[letter_to_index(letter)] = true;
        }
    
        SevenSegmentDisplay { value }
    }
}

impl SevenSegmentDisplay {
    fn segment_count(&self) -> u8 {
        self.value.iter().filter(|segment| **segment).collect::<Vec<&bool>>().len() as u8
    }

    fn numerical_value(&self) -> u8 {
        match self.value {
            [false, false, true, false, false, true, false] => 1,
            [true, false, true, true, true, false, true] => 2,
            [true, false, true, true, false, true, true] => 3,
            [false, true, true, true, false, true, false] => 4,
            [true, true, false, true, true, true, true] => 6,
            [true, true, false, true, false, true, true] => 5,
            [true, false, true, false, false, true, false] => 7,
            [true, true, true, true, true, true, true] => 8,
            [true, true, true, true, false, true, true] => 9,
            [true, true, true, false, true, true, true] => 0,
            _ => {
                println!("invalid value {:?}", self.value);
                0
            }
        }
    }
}

impl fmt::Display for SevenSegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.value;
        let seg_one = horizontal_segment(val[0]);
        let segs_two_and_three = vertical_segments(val[1], val[2]);
        let seg_four = horizontal_segment(val[3]);
        let segs_five_and_six = vertical_segments(val[4], val[5]);
        let seg_seven = horizontal_segment(val[6]);
        write!(
            f,
            "\n {} \n{} {} \n{} {} \n", seg_one, segs_two_and_three, seg_four, segs_five_and_six, seg_seven
        )
    }
}

fn horizontal_segment(val: bool) -> String {
    String::from(if val { "####" } else { "    " })
}

fn vertical_segment(val: bool) -> String {
    String::from(if val { "#" } else { " " })
}

fn vertical_segments(val_1: bool, val_2: bool) -> String {
    let seg_1 = vertical_segment(val_1);
    let seg_2 = vertical_segment(val_2);
    (0..2).map(|_| format!("{}    {}\n", seg_1, seg_2)).collect()
}

impl fmt::Debug for SevenSegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ value: {:?} }}",
            self.value
        )
    }
}

struct Entry {
    output_value: [SevenSegmentDisplay; 4]
}