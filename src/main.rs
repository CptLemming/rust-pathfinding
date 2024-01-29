use std::collections::{HashMap, BinaryHeap};
use std::hash::Hash;

use colored::{Colorize, ColoredString};

fn main() {
    println!("Hello, world!");
    let world = create_game();

    render_game(&world);

    let start = find_node(&world, Tile::Start);
    let end = find_node(&world, Tile::End);

    println!("Start : {start:?}");
    println!("End   : {end:?}");

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    heap.push(Node(0, start.clone()));

    let mut visited: HashMap<Vector, Option<Vector>> = HashMap::default();
    visited.insert(start.clone(), None);

    let mut cost: HashMap<Vector, u8> = HashMap::default();
    cost.insert(start.clone(), 0);

    while let Some(node) = heap.pop() {
        if node.1 == end {
            println!("END");
            break;
        }

        // println!("Node : {node:?} of {}", heap.len());

        for next in get_edges(&world, &node.1) {
            let new_node_cost = cost.get(&node.1).map(|c|c.clone()).unwrap_or(0) + next.cost(&world);
            let existing_node_cost = cost.get(&next).map(|c|c.clone()).unwrap_or(0);
            // println!("Next : {next:?} = {new_node_cost} < {existing_node_cost}");

            if !cost.contains_key(&next) || new_node_cost < existing_node_cost {
                cost.insert(next.clone(), new_node_cost);
                heap.push(Node(new_node_cost + goal_heuristic(&end, &next), next.clone()));
                visited.insert(next.clone(), Some(node.1.clone()));
            }
        }

        // render_game(&combine_path(&world, &visited.keys().map(|k|k.clone()).collect::<Vec<Vector>>()));
    }

    // println!("Post loop -> {visited:?}");

    let mut current = end;
    let mut path: Vec<Vector> = vec![];
    while current != start {
        path.push(current.clone());

        match visited.get(&current) {
            Some(Some(next)) => {
                current = next.clone();
            }
            _ => {
                break;
            }
        }
    }
    path.push(start.clone());
    path.reverse();

    render_game(&combine_path(&world, &path));
    // render_game(&combine_path(&world, &visited.keys().map(|k|k.clone()).collect::<Vec<Vector>>()));

    println!("Done -> visited = {}, path = {}", visited.len(), path.len());
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Vector(u8, u8);

impl Vector {
    pub fn cost(&self, world: &GameWorld) -> u8 {
        match world[self.0 as usize][self.1 as usize].into() {
            Tile::Floor => 1,
            Tile::Water => 5,
            Tile::Start | Tile::End => 0,
            _ => 10,
        }
    }
}

impl From<(u8, u8)> for Vector {
    fn from(value: (u8, u8)) -> Self {
        return Vector(value.0, value.1);
    }
}
impl From<(usize, usize)> for Vector {
    fn from(value: (usize, usize)) -> Self {
        return Vector(value.0 as u8, value.1 as u8);
    }
}
impl From<(i32, i32)> for Vector {
    fn from(value: (i32, i32)) -> Self {
        return Vector(value.0 as u8, value.1 as u8);
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node(u8, Vector);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.0.cmp(&other.1.0).then_with(|| self.1.1.cmp(&other.1.1)))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       Some(self.cmp(other))
    }
}

enum Tile {
    Floor = 0,
    Wall = 1,
    Water = 2,
    Start = 3,
    End = 4,
    Path = 5,
}

impl Tile {
    pub fn as_color(self) -> ColoredString {
        match self {
            Tile::Floor => " ".bold(),
            Tile::Wall => "#".white(),
            Tile::Water => "~".blue(),
            Tile::Start => "S".red(),
            Tile::End => "E".red().bold(),
            Tile::Path => "x".green()
        }
    }
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            0 => Tile::Floor,
            1 => Tile::Wall,
            2 => Tile::Water,
            3 => Tile::Start,
            4 => Tile::End,
            5 => Tile::Path,
            _ => {
                panic!("Not an enum");
            }
        }
    }
}

impl Into<u8> for Tile {
    fn into(self) -> u8 {
        match self {
            Tile::Floor => 0,
            Tile::Wall => 1,
            Tile::Water => 2,
            Tile::Start => 3,
            Tile::End => 4,
            Tile::Path => 5,
        }
    }
}

type GameWorld = Vec<Vec<u8>>;

fn create_game() -> GameWorld {
    return vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 2, 2, 0, 0, 4, 1, 0, 1],
        vec![1, 0, 2, 2, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 2, 2, 0, 1, 1, 2, 2, 1],
        vec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
        vec![1, 0, 0, 0, 1, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1, 1, 0, 0, 0, 1],
        vec![1, 3, 1, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];
}

fn render_game(world: &GameWorld) {
    println!("");

    for row in world {
        // println!("{row:?}");
        for col in row {
            let tile: Tile = col.clone().into();
            print!("{} ", tile.as_color());
        }
        println!("");
    }
    println!("");
}

fn find_node(world: &GameWorld, search: Tile) -> Vector {
    let search = search.into();
    let mut res: Vector = (0, 0).into();

    'outer: for i in 0..world.len() {
        for j in 0..world[i].len() {
            if world[i][j] == search {
                res = (i, j).into();
                break 'outer;
            }
        }
    }

    return res;
}

fn get_edges(world: &GameWorld, node: &Vector) -> Vec<Vector> {
    let mut edges = vec![];

    if world.len() < 1 || world[0].len() < 1 {
        return edges;
    }

    // North
    if node.0 > 0 {
        add_neighbor(&world, &mut edges, (node.0 - 1, node.1).into());
    }

    // East
    if (node.1 as usize) < (world[node.0 as usize].len() - 1) {
        add_neighbor(&world, &mut edges, (node.0, node.1 + 1).into());
    }

    // South
    if (node.0 as usize) < (world.len() - 1) {
        add_neighbor(&world, &mut edges, (node.0 + 1, node.1).into());
    }

    // West
    if node.1 > 0 {
        add_neighbor(&world, &mut edges, (node.0, node.1 - 1).into());
    }

    return edges;
}

fn add_neighbor(world: &GameWorld, edges: &mut Vec<Vector>, node: Vector) {
    if world[node.0 as usize][node.1 as usize] != Tile::Wall.into() {
        edges.push(node);
    }
}

fn combine_path(world: &GameWorld, path: &Vec<Vector>) -> GameWorld {
    let mut new_world = world.clone();

    for p in path {
        if new_world[p.0 as usize][p.1 as usize] == Tile::Floor.into() {
            new_world[p.0 as usize][p.1 as usize] = Tile::Path.into();
        } else if new_world[p.0 as usize][p.1 as usize] == Tile::Water.into() {
            new_world[p.0 as usize][p.1 as usize] = Tile::Path.into();
        }
    }

    return new_world;
}

fn goal_heuristic(end: &Vector, node: &Vector) -> u8 {
    return manhattan_distance(end, node);
}

fn manhattan_distance(end: &Vector, node: &Vector) -> u8 {
    return end.0.abs_diff(node.0) + end.1.abs_diff(node.1);
}
