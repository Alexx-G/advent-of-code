use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

use failure::{Error, format_err};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn expand_wire_segment(starting_point: &Point, direction: &str, length: usize) -> Result<(Point, Vec<Point>), Error> {
    let mut points: Vec<Point> = Vec::with_capacity(length);
    let mut current_point = Point {
        x: starting_point.x,
        y: starting_point.y,
    };
    for _ in 0..length {
        let point = match direction {
            "L" => Ok(Point { x: current_point.x - 1, y: current_point.y }),
            "R" => Ok(Point { x: current_point.x + 1, y: current_point.y }),
            "U" => Ok(Point { x: current_point.x, y: current_point.y + 1 }),
            "D" => Ok(Point { x: current_point.x, y: current_point.y - 1 }),
            _ => Err(format_err!("Unknown direction")),
        }?;
        current_point.x = point.x;
        current_point.y = point.y;
        points.push(point);
    }
    Ok((current_point, points))
}

fn get_wire_path(wire_path: &String) -> Result<Vec<Point>, Error> {
    let mut current_point = Point {
        x: 0,
        y: 0,
    };
    let mut wire_coords = Vec::new();
    for x in wire_path.trim().split(",") {
        let (direction, length) = x.split_at(1);
        let length = length.parse::<usize>()?;
        let wire_segment = expand_wire_segment(&current_point, direction, length)?;
        current_point = wire_segment.0;
        wire_coords.extend(wire_segment.1);
    }
    Ok(wire_coords)
}

fn get_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn main() -> Result<(), Error> {
    let path = "data/day3.txt";
    let input = File::open(path)?;
    let mut buffered = BufReader::new(input);

    let mut input = String::new();
    buffered.read_line(&mut input)?;
    let wire1_path = get_wire_path(&input)?;
    let wire1: HashSet<&Point> = wire1_path.iter().collect();

    let mut input = String::new();
    buffered.read_line(&mut input)?;
    let wire2_path = get_wire_path(&input)?;
    let wire2: HashSet<&Point> = wire2_path.iter().collect();

    let origin = Point { x: 0, y: 0};
    let min_distance = wire1.intersection(&wire2)
        .map(|a| get_distance(a, &origin))
        .min()
        .ok_or(format_err!("Didn't find the min distance. Perhaps there's no intersection."));
    println!("Intersection with min distance = {}", min_distance?);

    let shortest_path = wire1.intersection(&wire2)
        .map(|intersection| {
            let wire1_steps = wire1_path.iter().position(|p| &p == intersection).unwrap();
            let wire2_steps = wire2_path.iter().position(|p| &p == intersection).unwrap();
            wire1_steps + wire2_steps + 2
        })
        .min()
        .ok_or(format_err!("Expected to find at least 1 intersection"));
    println!("Path with to the shortests intersection has {} steps", shortest_path?);
    Ok(())
}