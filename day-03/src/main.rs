use std::io::{self, BufRead};
mod command;
// mod point_iter;
mod segment;

use command::Command;
// use point_iter::{Point, PointIter};
use segment::{Segment, SegmentIter};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().expect("Expected two lines of stdin").unwrap();
    let second_line = lines.next().expect("Expected two lines of stdin").unwrap();

    let first: Vec<Command> = first_line.split(",").map(|s| s.parse().unwrap()).collect();
    let second: Vec<Command> = second_line.split(",").map(|s| s.parse().unwrap()).collect();

    let first_iter: Vec<Segment> = SegmentIter::new(first.into_iter()).collect();
    let second_iter: Vec<Segment> = SegmentIter::new(second.into_iter()).collect();

    let mut min_manhatten_distance = None;
    for first_segment in &first_iter[..] {
        for second_segment in &second_iter[..] {
            if let Some((x, y)) = first_segment.intersection(second_segment) {
                let manhatten_distance = x + y;
                if let Some(min) = min_manhatten_distance {
                    if manhatten_distance < min {
                        min_manhatten_distance = Some(manhatten_distance)
                    }
                } else {
                    min_manhatten_distance = Some(manhatten_distance)
                }
            }
        }
    }

    if let Some(min) = min_manhatten_distance {
        println!("{}", min);
    }

    let mut min_path = None;
    let mut first_path_magnitude = 0;
    for first_segment in &first_iter[..] {
        let mut second_path_magnitude = 0;
        for second_segment in &second_iter[..] {
            if let Some(pt) = first_segment.intersection(second_segment) {
                let total_distance = first_segment.magnitude_to_point(&pt)
                    + second_segment.magnitude_to_point(&pt)
                    + first_path_magnitude
                    + second_path_magnitude;

                if let Some(min) = min_path {
                    if total_distance < min {
                        min_path = Some(total_distance)
                    }
                } else {
                    min_path = Some(total_distance)
                }
            }
            second_path_magnitude += second_segment.magnitude();
        }
        first_path_magnitude += first_segment.magnitude();
    }

    if let Some(min) = min_path {
        println!("{}", min);
    }
}
