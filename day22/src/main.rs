use common::load;

fn main() {
    println!("Day 22, part {}", if cfg!(feature="part2") { "2" } else { "1" });

    // Load the map
    let lines = load::lines().unwrap();
    let mut bricks = parse_bricks(&lines);

    // Sort the bricks by z
    bricks.sort_by(|a, b| a.0.2.cmp(&b.0.2));
    let extents = find_extents(&bricks);

    // Let the bricks fall and sort again afterwards
    drop(&mut bricks, extents);
    bricks.sort_by(|a, b| a.0.2.cmp(&b.0.2));

    // For each brick, find which bricks it supports
    let brick_supports:Vec<Vec<usize>> = supports(&bricks);

    // For each brick, find the bricks it is supported by
    let brick_supported_by:Vec<Vec<usize>> = supported_by(&brick_supports);

    // Find all bricks that are not the only support for any brick
    let disintegratable: Vec<usize> = find_disintegratable_bricks(&brick_supports, &brick_supported_by);
    println!("Number of disintegratable bricks: {}", disintegratable.len());

}

fn parse_bricks(lines: &Vec<String>) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
    let mut bricks: Vec<((i32, i32, i32), (i32, i32, i32))> = Vec::new();
    for line in lines {
        let corners: Vec<&str> = line.split("~").collect();
        let c0:Vec<i32> = corners[0].split(",").map(|s| s.parse().unwrap()).collect();
        let c1:Vec<i32> = corners[1].split(",").map(|s| s.parse().unwrap()).collect();
        bricks.push(((c0[0], c0[1], c0[2]), (c1[0], c1[1], c1[2])));
    }
    bricks
}

fn find_extents(bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>) -> ((i32, i32, i32), (i32, i32, i32)) {
    let extents: ((i32, i32, i32), (i32, i32, i32)) = bricks.into_iter().fold(((std::i32::MAX, std::i32::MAX, std::i32::MAX), (0, 0, 0)), |acc, brick| {
        let min = (acc.0.0.min(brick.0.0), acc.0.1.min(brick.0.1), acc.0.2.min(brick.0.2));
        let max = (acc.1.0.max(brick.1.0), acc.1.1.max(brick.1.1), acc.1.2.max(brick.1.2));
        (min, max)
    });
    extents
}

fn drop(bricks: &mut Vec<((i32, i32, i32), (i32, i32, i32))>, extents: ((i32, i32, i32), (i32, i32, i32))) {
    let mut heights:Vec<Vec<i32>> = vec![vec![1; extents.1.0 as usize + 1]; extents.1.1 as usize + 1];
    for brick in bricks {
        let distance = brick.0.2 - highest_z_under(&brick, &heights);
        if distance > 0 {
            brick.0.2 -= distance;
            brick.1.2 -= distance;
        }
        pile(&brick, &mut heights);
    }
}

fn pile(brick: &((i32, i32, i32), (i32, i32, i32)), heights: &mut Vec<Vec<i32>>) {
    for x in brick.0.0..=brick.1.0 {
        for y in brick.0.1..=brick.1.1 {
            heights[y as usize][x as usize] = brick.1.2 + 1;
        }
    }
}

fn highest_z_under(brick: &((i32, i32, i32), (i32, i32, i32)), heights: &Vec<Vec<i32>>) -> i32 {
    let min_x = brick.0.0;
    let max_x = brick.1.0;
    let min_y = brick.0.1;
    let max_y = brick.1.1;
    (min_x..=max_x)
        .flat_map(|x| { (min_y..=max_y).map(move |y| heights[y as usize][x as usize]) })
        .max()
        .unwrap()
}

fn supports(bricks: &Vec<((i32, i32, i32), (i32, i32, i32))>) -> Vec<Vec<usize>> {
    let mut supporting: Vec<Vec<usize>> = vec![Vec::new(); bricks.len()];

    for (i, brick_i) in bricks.iter().enumerate() {
        let i_max_z = brick_i.1.2;

        for (j, brick_j) in bricks.iter().enumerate().skip(i + 1) {
            let j_min_z = brick_j.0.2;

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

fn overlaps_xy(brick0: &((i32, i32, i32), (i32, i32, i32)), brick1: &((i32, i32, i32), (i32, i32, i32))) -> bool {
    brick0.1.0 >= brick1.0.0 && brick0.0.0 <= brick1.1.0 && brick0.1.1 >= brick1.0.1 && brick0.0.1 <= brick1.1.1
}

fn supported_by(supports: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut brick_supported_by:Vec<Vec<usize>> = vec![Vec::new(); supports.len()];
    for (i, support_list) in supports.iter().enumerate() {
        for &j in support_list {
            brick_supported_by[j].push(i);
        }
    }
    brick_supported_by
}

fn find_disintegratable_bricks(supports: &Vec<Vec<usize>>, supported_by: &Vec<Vec<usize>>) -> Vec<usize> {
    let disintegratable: Vec<usize> = supports.iter().enumerate().filter_map(|(i, support_list)| {
        if support_list.len() == 0 || support_list.iter().all(|&j| supported_by[j].len() > 1) {
            Some(i)
        } else {
            None
        }
    }).collect();
    disintegratable
}
