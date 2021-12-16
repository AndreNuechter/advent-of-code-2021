use std::fmt;

#[derive(Clone)]
struct Lanternfish {
    days_to_reproduce: u8,
}

pub fn day_six(input: &str) {
    let fish: Vec<Lanternfish> = input
        .split(",")
        .map(|num_string| Lanternfish::from(num_string.parse::<u8>().unwrap()))
        .collect();
    part_one(&mut fish.clone(), 80);
    part_two(&fish, 256);
}

fn part_two(fish: &Vec<Lanternfish>, days: u32) {
    let cycle_length = 7;
    let first_cycle_delay = 2;
    let mut stages: Vec<u64> = vec![0; cycle_length + first_cycle_delay];

    for fish_n in fish.iter() {
        stages[fish_n.days_to_reproduce as usize] += 1;
    }

    for _ in 0..days {
        // fish in stages[0] reproduce, so they get removed and added to the end, to cover new ones
        // and added to stages[6] to reset their cycle
        let new_fish = stages.remove(0);
        stages[cycle_length - 1] += new_fish;
        stages.push(new_fish);
    }

    println!("pt 2: {}", stages.iter().sum::<u64>());
}

fn part_one(fish: &mut Vec<Lanternfish>, days: u32) {
    let mut days = days;

    while days > 0 {
        days -= 1;

        let mut new_fish = Vec::new();

        for fish_n in fish.iter_mut() {
            fish_n.days_to_reproduce = if fish_n.days_to_reproduce == 0 {
                new_fish.push(Lanternfish::new());
                6
            } else {
                fish_n.days_to_reproduce - 1
            };
        }

        fish.append(&mut new_fish);
    }

    println!("pt 1: {}", fish.len());
}

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish {
            days_to_reproduce: 8,
        }
    }
}

impl From<u8> for Lanternfish {
    fn from(days_to_reproduce: u8) -> Self {
        Lanternfish { days_to_reproduce }
    }
}

impl fmt::Debug for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|days_to_reproduce: {}|", self.days_to_reproduce)
    }
}
