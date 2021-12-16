use std::{cmp, fmt};

struct Point {
    x: i64,
    y: i64,
}

struct Segment {
    start: Point,
    end: Point,
}

pub fn day_five(input: &str) {
    let segments: Vec<Segment> = input.lines().map(|line| Segment::from(line)).collect();
    part_one(&segments);
    part_two(&segments);
}

fn part_two(segments: &Vec<Segment>) {
    let segments = segments.iter().map(|e| e).collect();
    let diagram = create_diagram(&segments);
    println!("pt 2: {}", count_volatile_tiles(&diagram));
}

fn part_one(segments: &Vec<Segment>) {
    let straight_segments: Vec<&Segment> = segments
        .iter()
        .filter(|seg| seg.start.x == seg.end.x || seg.start.y == seg.end.y)
        .collect();
    let diagram = create_diagram(&straight_segments);
    println!("pt 1: {}", count_volatile_tiles(&diagram));
}

fn create_diagram(segments: &Vec<&Segment>) -> Vec<i64> {
    let width = segments
        .iter()
        .map(|seg| cmp::max(seg.start.x, seg.end.x))
        .max()
        .unwrap()
        + 1;
    let height = segments
        .iter()
        .map(|seg| cmp::max(seg.start.y, seg.end.y))
        .max()
        .unwrap()
        + 1;
    let mut diagram = vec![0; (width * height) as usize];

    for segment in segments.iter() {
        for point in segment.get_points().iter() {
            diagram[(point.x + point.y * width) as usize] += 1;
        }
    }

    diagram
}

fn count_volatile_tiles(diagram: &Vec<i64>) -> usize {
    diagram
        .iter()
        .filter(|cell| **cell > 1)
        .collect::<Vec<&i64>>()
        .len()
}

fn get_delta(start: i64, end: i64) -> i64 {
    if start < end {
        1
    } else if start > end {
        -1
    } else {
        0
    }
}

fn get_min_and_max(component_a: i64, component_b: i64) -> (i64, i64) {
    (
        cmp::min(component_a, component_b),
        cmp::max(component_a, component_b),
    )
}

fn get_start_and_end(component_a: i64, component_b: i64) -> (i64, i64) {
    let (min, max) = get_min_and_max(component_a, component_b);
    (min, max + 1)
}

impl Segment {
    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        if self.start.x == self.end.x {
            let x = self.start.x;
            let (start, end) = get_start_and_end(self.start.y, self.end.y);
            for y in start..end {
                points.push(Point { x, y });
            }
        }

        if self.start.y == self.end.y {
            let y = self.start.y;
            let (start, end) = get_start_and_end(self.start.x, self.end.x);
            for x in start..end {
                points.push(Point { x, y });
            }
        }

        if self.start.x != self.end.x && self.start.y != self.end.y {
            let (min_x, max_x) = get_min_and_max(self.start.x, self.end.x);
            let (min_y, max_y) = get_min_and_max(self.start.y, self.end.y);
            let x_delta = get_delta(self.start.x, self.end.x);
            let y_delta = get_delta(self.start.y, self.end.y);
            let point_count = cmp::max(max_x - min_x, max_y - min_y) + 1;

            for index in 0..point_count {
                points.push(Point {
                    x: self.start.x + x_delta * index,
                    y: self.start.y + y_delta * index,
                });
            }
        }

        points
    }
}

impl From<&str> for Segment {
    fn from(segment_string: &str) -> Self {
        let split: Vec<i64> = segment_string
            .replace(" -> ", ",")
            .split(",")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        Segment {
            start: Point {
                x: split[0],
                y: split[1],
            },
            end: Point {
                x: split[2],
                y: split[3],
            },
        }
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|x1: {}, y1: {} -> x2: {}, y2: {}|",
            self.start.x, self.start.y, self.end.x, self.end.y
        )
    }
}
