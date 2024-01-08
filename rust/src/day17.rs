use anyhow::Result;
use aoc::print_answers;
use std::collections::{BinaryHeap, HashMap};

// TODO: Test the algorithm with sets

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

const DIRECTION: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }

    fn rotate(&mut self) {
        *self = DIRECTION[((*self as isize + 1) % 4) as usize];
    }

    fn next_coor(&mut self, point: &Point) -> (isize, isize) {
        let x = point.x as isize;
        let y = point.y as isize;
        self.rotate();
        match *self {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Right => (x, y + 1),
            Direction::Left => (x, y - 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    point: Point,
    count: usize,
    direction: Direction,
}

impl Node {
    fn new(x: usize, y: usize, count: usize, direction: Direction) -> Self {
        Self {
            point: Point { x, y },
            count,
            direction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    node: Node,
}

impl State {
    fn new(cost: usize, node: Node) -> Self {
        Self { cost, node }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn next(node: &Node, width: usize, height: usize) -> Vec<Node> {
    let mut points = Vec::new();
    let mut dir = node.direction.opposite();

    for _ in 0..3 {
        let (x, y) = dir.next_coor(&node.point);
        if x >= 0 && (x as usize) < height && y >= 0 && (y as usize) < width {
            if node.direction != dir {
                points.push(Node::new(x as usize, y as usize, 1, dir));
            } else if node.count < 3 {
                points.push(Node::new(
                    x as usize,
                    y as usize,
                    node.count + 1,
                    node.direction,
                ));
            }
        }
    }

    points
}

fn next2(node: &Node, width: usize, height: usize) -> Vec<Node> {
    let mut points = Vec::new();
    let mut dir = node.direction.opposite();

    for _ in 0..3 {
        let (x, y) = dir.next_coor(&node.point);
        if x >= 0 && (x as usize) < height && y >= 0 && (y as usize) < width {
            if node.direction != dir && node.count >= 4 {
                points.push(Node::new(x as usize, y as usize, 1, dir));
            } else if node.direction == dir && node.count < 10 {
                points.push(Node::new(
                    x as usize,
                    y as usize,
                    node.count + 1,
                    node.direction,
                ));
            }
        }
    }

    points
}

fn part_1(lines: &[String]) -> usize {
    let grid = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut distances: HashMap<Node, usize> = HashMap::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let end = Point::new(grid.len() - 1, grid[0].len() - 1);

    distances.insert(Node::new(0, 0, 0, Direction::Down), 0);
    distances.insert(Node::new(0, 0, 0, Direction::Right), 0);
    frontier.push(State::new(0, Node::new(0, 0, 0, Direction::Down)));
    frontier.push(State::new(0, Node::new(0, 0, 0, Direction::Right)));

    while let Some(State { cost, node }) = frontier.pop() {
        if node.point == end {
            return cost;
        }

        for node in next(&node, grid[0].len(), grid.len()) {
            let new_cost = cost + grid[node.point.x][node.point.y];
            if let Some(&curr) = distances.get(&node) {
                if new_cost >= curr {
                    continue;
                }
            }

            distances.insert(node, new_cost);
            frontier.push(State::new(new_cost, node));
        }
    }

    0
}

fn part_2(lines: &[String]) -> usize {
    let grid = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut distances: HashMap<Node, usize> = HashMap::new();
    let mut frontier: BinaryHeap<State> = BinaryHeap::new();
    let end = Point::new(grid.len() - 1, grid[0].len() - 1);

    distances.insert(Node::new(0, 0, 0, Direction::Down), 0);
    distances.insert(Node::new(0, 0, 0, Direction::Right), 0);
    frontier.push(State::new(0, Node::new(0, 0, 0, Direction::Down)));
    frontier.push(State::new(0, Node::new(0, 0, 0, Direction::Right)));

    while let Some(State { cost, node }) = frontier.pop() {
        if node.point == end && node.count >= 4 {
            return cost;
        }

        for node in next2(&node, grid[0].len(), grid.len()) {
            let new_cost = cost + grid[node.point.x][node.point.y];
            if let Some(&curr) = distances.get(&node) {
                if new_cost >= curr {
                    continue;
                }
            }

            distances.insert(node, new_cost);
            frontier.push(State::new(new_cost, node));
        }
    }

    0
}

fn main() -> Result<()> {
    // part 1: 1138
    // part 2: 1312
    print_answers(17, &part_1, &part_2);
    Ok(())
}
