use common::load;
use std::collections::VecDeque;
use itertools::Itertools;
use itertools::iproduct;

type Position = (i32, i32, i32);
type Volume = (Position, Position);

fn main() {
    println!("=== Day 22, part {} ===", if cfg!(feature = "part2") { "2" } else { "1" });

    // Load the map
    let lines = load::lines().unwrap();
    let mut bricks = parse_bricks(&lines);

    // Sort the bricks by z
    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));
    let extents = find_extents(&bricks);

    // Let the bricks fall and sort again afterwards
    drop(&mut bricks, extents);
    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    // For each brick, find which bricks it supports
    let brick_supports: Vec<Vec<usize>> = supports(&bricks);

    // For each brick, find the bricks it is supported by
    let brick_supported_by: Vec<Vec<usize>> = supported_by(&brick_supports);

    let result = if cfg!(feature = "part2") {
        part2(&brick_supports, &brick_supported_by)
    } else {
        part1(&brick_supports, &brick_supported_by)
    };
    println!("Result: {}", result);
}

fn part1(supports: &[Vec<usize>], supported_by: &[Vec<usize>]) -> usize {
    let disintegratable: Vec<usize> = supports
        .iter()
        .enumerate()
        .filter_map(|(i, support_list)| {
            if support_list.is_empty() || support_list.iter().all(|&j| supported_by[j].len() > 1) {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    disintegratable.len()
}

fn part2(supports: &[Vec<usize>], supported_by: &[Vec<usize>]) -> usize {
    (0..supports.len())
        .map(|brick| chain_reaction(brick, supports, supported_by))
        .sum()
}

fn chain_reaction(first_brick: usize, supports: &[Vec<usize>], initial_supported_by: &[Vec<usize>]) -> usize {
    let mut supported_by = initial_supported_by.to_owned();
    let mut disintegation_queue: VecDeque<usize> = VecDeque::from(vec![first_brick]);
    let mut count = 0;
    while let Some(brick) = disintegation_queue.pop_front() {
        count += 1;
        for &supported in &supports[brick] {
            supported_by[supported].retain(|&b| b != brick);
            if supported_by[supported].is_empty() {
                disintegation_queue.push_back(supported);
            }
        }
    }

    count - 1 // Don't count the original brick
}

fn parse_bricks(lines: &[String]) -> Vec<Volume> {
    lines.iter().map(|line| {
        let corners: Vec<&str> = line.split("~").collect();
        let c0: (i32, i32, i32) = corners[0].split(",")
            .map(|s| s.parse().unwrap())
            .collect_tuple::<(_,_,_)>().unwrap();
        let c1: (i32, i32, i32) = corners[1].split(",")
            .map(|s| s.parse().unwrap())
            .collect_tuple::<(_,_,_)>().unwrap();
        (c0, c1)
    }).collect()
}

fn find_extents(bricks: &[Volume]) -> Volume {
    let extents: Volume =
        bricks
            .iter()
            .fold(((i32::MAX, i32::MAX, i32::MAX), (0, 0, 0)), |acc, brick| {
                let min = (acc.0 .0.min(brick.0 .0), acc.0 .1.min(brick.0 .1), acc.0 .2.min(brick.0 .2));
                let max = (acc.1 .0.max(brick.1 .0), acc.1 .1.max(brick.1 .1), acc.1 .2.max(brick.1 .2));
                (min, max)
            });
    extents
}

fn drop(bricks: &mut [Volume], extents: Volume) {
    let mut heights: Vec<Vec<i32>> = vec![vec![1; extents.1 .0 as usize + 1]; extents.1 .1 as usize + 1];
    for brick in bricks {
        let distance = brick.0 .2 - highest_z_under(brick, &heights);
        if distance > 0 {
            brick.0 .2 -= distance;
            brick.1 .2 -= distance;
        }
        pile(brick, &mut heights);
    }
}

fn pile(brick: &Volume, heights: &mut [Vec<i32>]) {
    for x in brick.0 .0..=brick.1 .0 {
        for y in brick.0 .1..=brick.1 .1 {
            heights[y as usize][x as usize] = brick.1 .2 + 1;
        }
    }
}

fn highest_z_under(brick: &Volume, heights: &[Vec<i32>]) -> i32 {
    let min_x = brick.0 .0;
    let max_x = brick.1 .0;
    let min_y = brick.0 .1;
    let max_y = brick.1 .1;
    iproduct!(min_x..=max_x, min_y..=max_y)
        .map(|(x, y)| heights[y as usize][x as usize])
        .max()
        .unwrap()
}

fn supports(bricks: &[Volume]) -> Vec<Vec<usize>> {
    let mut supporting: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];

    for (i, brick_i) in bricks.iter().enumerate() {
        let i_max_z = brick_i.1 .2;

        for (j, brick_j) in bricks.iter().enumerate().skip(i + 1) {
            let j_min_z = brick_j.0 .2;

            // If brick_j is too high then stop looking because the remaining bricks will also be too high
            if j_min_z > i_max_z + 1 {
                break;
            }

            // If brick_j is just above brick_i and the bricks overlap, then brick_i supports brick_j.
            if j_min_z == i_max_z + 1 && overlaps_xy(brick_i, brick_j) {
                supporting[i].push(j);
            }
        }
    }

    supporting
}

fn overlaps_xy(brick0: &Volume, brick1: &Volume) -> bool {
    let min0 = brick0.0;
    let max0 = brick0.1;
    let min1 = brick1.0;
    let max1 = brick1.1;
    max0.0 >= min1.0 && min0.0 <= max1.0 && max0.1 >= min1.1 && min0.1 <= max1.1
}

fn supported_by(supports: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut brick_supported_by: Vec<Vec<usize>> = vec![Vec::new(); supports.len()];
    for (i, support_list) in supports.iter().enumerate() {
        for &j in support_list {
            brick_supported_by[j].push(i);
        }
    }
    brick_supported_by
}
