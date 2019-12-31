use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::directed::astar;
use std::collections::HashMap;

#[aoc_generator(day20)]
fn dotpoundmap(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut maze = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate().filter(|&(_, ch)| ch != ' ') {
            maze.insert(
                (x as i32, y as i32),
                match ch {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    _ => Tile::Letter(ch),
                },
            );
        }
    }
    maze
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Letter(char),
}

fn portals(maze: &HashMap<(i32, i32), Tile>) -> HashMap<String, Vec<(i32, i32)>> {
    // Find all the letters
    let letters: Vec<((i32, i32), char)> = maze
        .iter()
        .filter_map(|(k, v)| {
            if let Tile::Letter(c) = v {
                Some((*k, *c))
            } else {
                None
            }
        })
        .collect();
    // Then, find the letters in between another letter and a '.'
    let mut portals: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    letters
        .iter()
        .filter_map(|(pos, letter)| {
            if let (Some(Tile::Empty), Some(Tile::Letter(c))) =
                (maze.get(&(pos.0 - 1, pos.1)), maze.get(&(pos.0 + 1, pos.1)))
            {
                Some(([letter, c].iter().cloned().collect(), (pos.0 - 1, pos.1)))
            } else if let (Some(Tile::Empty), Some(Tile::Letter(c))) =
                (maze.get(&(pos.0 + 1, pos.1)), maze.get(&(pos.0 - 1, pos.1)))
            {
                Some(([c, letter].iter().cloned().collect(), (pos.0 + 1, pos.1)))
            } else if let (Some(Tile::Empty), Some(Tile::Letter(c))) =
                (maze.get(&(pos.0, pos.1 - 1)), maze.get(&(pos.0, pos.1 + 1)))
            {
                Some(([letter, c].iter().cloned().collect(), (pos.0, pos.1 - 1)))
            } else if let (Some(Tile::Empty), Some(Tile::Letter(c))) =
                (maze.get(&(pos.0, pos.1 + 1)), maze.get(&(pos.0, pos.1 - 1)))
            {
                Some(([c, letter].iter().cloned().collect(), (pos.0, pos.1 + 1)))
            } else {
                None
            }
        })
        .for_each(|(name, coord)| portals.entry(name).or_default().push(coord));
    portals
}

fn cand_pos(pos: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
}

fn neighbours(
    maze: &HashMap<(i32, i32), Tile>,
    portals: &HashMap<String, Vec<(i32, i32)>>,
    pos: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut neighbours = Vec::new();

    for cand in cand_pos(pos).iter() {
        match maze.get(cand) {
            Some(Tile::Empty) => neighbours.push(*cand),
            Some(Tile::Letter(_)) => {
                // If there is a letter, the current 'pos' must be a portal
                // See if it has an exit (AA, ZZ don't) and if so, add the exit
                // as a neighbour
                if let Some(portal_exit) = portals
                    .iter()
                    .filter_map(|(_name, coords)| {
                        if coords.contains(&pos) {
                            coords.iter().find(|&c| *c != pos)
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    neighbours.push(*portal_exit);
                }
            }
            _ => {}
        }
    }
    neighbours
}

#[aoc(day20, part1)]
fn solver1(maze: &HashMap<(i32, i32), Tile>) -> i64 {
    let portals = portals(&maze);
    let start = portals.get("AA").expect("No start node")[0];
    let finish = portals.get("ZZ").expect("No finish node")[0];

    let path = astar::astar(
        &start,
        |node| {
            neighbours(maze, &portals, *node)
                .iter()
                .cloned()
                .map(|nb| (nb, 1))
                .collect::<Vec<_>>()
        },
        |_| 1,
        |node| *node == finish,
    )
    .expect("No path found");
    path.1
}

fn dimensions(maze: &HashMap<(i32, i32), Tile>) -> (i32, i32) {
    (
        *maze.keys().map(|(x, _y)| x).max().unwrap(),
        *maze.keys().map(|(_x, y)| y).max().unwrap(),
    )
}

fn neighbours_recursed(
    maze: &HashMap<(i32, i32), Tile>,
    portals: &HashMap<String, Vec<(i32, i32)>>,
    pos: (i32, i32, i32),
) -> Vec<(i32, i32, i32)> {
    let mut neighbours = Vec::new();

    for cand in cand_pos((pos.1, pos.2)).iter() {
        match maze.get(cand) {
            Some(Tile::Empty) => neighbours.push((pos.0, cand.0, cand.1)),
            Some(Tile::Letter(_)) => {
                // If there is a letter, the current 'pos' must be a portal
                // See if it has an exit (AA, ZZ don't) and if so, add the exit
                // as a neighbour
                if let Some(portal_exit) = portals
                    .iter()
                    .filter_map(|(_name, coords)| {
                        if coords.contains(&(pos.1, pos.2)) {
                            coords.iter().find(|&c| *c != (pos.1, pos.2))
                        } else {
                            None
                        }
                    })
                    .next()
                {
                    // Are we going deeper? That is, toward the center?
                    let dimensions = dimensions(&maze);
                    let center = (dimensions.0 / 2, dimensions.1 / 2);
                    if (cand.0 > pos.1 && cand.0 < center.0)
                        || (cand.0 < pos.1 && cand.0 > center.0)
                        || (cand.1 > pos.2 && cand.1 < center.1)
                        || (cand.1 < pos.2 && cand.1 > center.1)
                    {
                        // moving inwards
                        neighbours.push((pos.0 + 1, portal_exit.0, portal_exit.1));
                    } else if pos.0 > 0 {
                        // moving outwards
                        neighbours.push((pos.0 - 1, portal_exit.0, portal_exit.1));
                    }
                }
            }
            _ => {}
        }
    }
    neighbours
}

#[aoc(day20, part2)]
fn solver2(maze: &HashMap<(i32, i32), Tile>) -> i64 {
    let portals = portals(&maze);
    let start = portals.get("AA").expect("No start node")[0];
    let finish = portals.get("ZZ").expect("No finish node")[0];

    let path = astar::astar(
        &(0, start.0, start.1),
        |node| {
            neighbours_recursed(maze, &portals, *node)
                .iter()
                .cloned()
                .map(|nb| (nb, 1))
                .collect::<Vec<_>>()
        },
        |_| 1,
        |node| *node == (0, finish.0, finish.1),
    )
    .expect("No path found");
    path.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let maze = dotpoundmap(
            "#                    A               \n\
             #                    A               \n\
             #   #################.#############  \n\
             #   #.#...#...................#.#.#  \n\
             #   #.#.#.###.###.###.#########.#.#  \n\
             #   #.#.#.......#...#.....#.#.#...#  \n\
             #   #.#########.###.#####.#.#.###.#  \n\
             #   #.............#.#.....#.......#  \n\
             #   ###.###########.###.#####.#.#.#  \n\
             #   #.....#        A   C    #.#.#.#  \n\
             #   #######        S   P    #####.#  \n\
             #   #.#...#                 #......VT\n\
             #   #.#.#.#                 #.#####  \n\
             #   #...#.#               YN....#.#  \n\
             #   #.###.#                 #####.#  \n\
             # DI....#.#                 #.....#  \n\
             #   #####.#                 #.###.#  \n\
             # ZZ......#               QG....#..AS\n\
             #   ###.###                 #######  \n\
             # JO..#.#.#                 #.....#  \n\
             #   #.#.#.#                 ###.#.#  \n\
             #   #...#..DI             BU....#..LF\n\
             #   #####.#                 #.#####  \n\
             # YN......#               VT..#....QG\n\
             #   #.###.#                 #.###.#  \n\
             #   #.#...#                 #.....#  \n\
             #   ###.###    J L     J    #.#.###  \n\
             #   #.....#    O F     P    #.#...#  \n\
             #   #.###.#####.#.#####.#####.###.#  \n\
             #   #...#.#.#...#.....#.....#.#...#  \n\
             #   #.#####.###.###.#.#.#########.#  \n\
             #   #...#.#.....#...#.#.#.#.....#.#  \n\
             #   #.###.#####.###.###.#.#.#######  \n\
             #   #.#.........#...#.............#  \n\
             #   #########.###.###.#############  \n\
             #            B   J   C               \n\
             #            U   P   P               ",
        );
        assert_eq!(solver1(&maze), 58);
    }

    #[test]
    fn test_run2() {
        let maze = dotpoundmap(
            "#              Z L X W       C                 \n\
             #              Z P Q B       K                 \n\
             #   ###########.#.#.#.#######.###############  \n\
             #   #...#.......#.#.......#.#.......#.#.#...#  \n\
             #   ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n\
             #   #.#...#.#.#...#.#.#...#...#...#.#.......#  \n\
             #   #.###.#######.###.###.#.###.###.#.#######  \n\
             #   #...#.......#.#...#...#.............#...#  \n\
             #   #.#########.#######.#.#######.#######.###  \n\
             #   #...#.#    F       R I       Z    #.#.#.#  \n\
             #   #.###.#    D       E C       H    #.#.#.#  \n\
             #   #.#...#                           #...#.#  \n\
             #   #.###.#                           #.###.#  \n\
             #   #.#....OA                       WB..#.#..ZH\n\
             #   #.###.#                           #.#.#.#  \n\
             # CJ......#                           #.....#  \n\
             #   #######                           #######  \n\
             #   #.#....CK                         #......IC\n\
             #   #.###.#                           #.###.#  \n\
             #   #.....#                           #...#.#  \n\
             #   ###.###                           #.#.#.#  \n\
             # XF....#.#                         RF..#.#.#  \n\
             #   #####.#                           #######  \n\
             #   #......CJ                       NM..#...#  \n\
             #   ###.#.#                           #.###.#  \n\
             # RE....#.#                           #......RF\n\
             #   ###.###        X   X       L      #.#.#.#  \n\
             #   #.....#        F   Q       P      #.#.#.#  \n\
             #   ###.###########.###.#######.#########.###  \n\
             #   #.....#...#.....#.......#...#.....#.#...#  \n\
             #   #####.#.###.#######.#######.###.###.#.#.#  \n\
             #   #.......#.......#.#.#.#.#...#...#...#.#.#  \n\
             #   #####.###.#####.#.#.#.#.###.###.#.###.###  \n\
             #   #.......#.....#.#...#...............#...#  \n\
             #   #############.#.#.###.###################  \n\
             #                A O F   N                     \n\
             #                A A D   M                     ",
        );
        assert_eq!(solver2(&maze), 396);
    }
}
