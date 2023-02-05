use core::panic;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Slot,
    Wall,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            ' ' => Self::Empty,
            '#' => Self::Wall,
            '.' => Self::Slot,
            _ => panic!("Unknown type of tile."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn u_turn(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,

        }
    }
}

trait Maze {
    fn get_tile(&self, x: isize, y: isize) -> Tile;
    fn next_pos(&self, pos: &Pos) -> Pos;
}

#[derive(Debug, Clone)]
struct Pos {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Pos {
    fn execute(&mut self, command: &Command, maze: &dyn Maze) {
        match command {
            Command::TurnLeft => self.dir = self.dir.turn_left(),
            Command::TurnRight => self.dir = self.dir.turn_right(),
            Command::MoveOn(len) => {
                for _ in 0..*len {
                    let next_pos = maze.next_pos(&self);
                    if maze.get_tile(next_pos.x, next_pos.y) == Tile::Wall {
                        break;
                    }
                    self.x = next_pos.x;
                    self.y = next_pos.y;
                    self.dir = next_pos.dir;
                }
            }
        }
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
    ymax: isize,
    hdims: Vec<(isize, isize)>,
    vdims: Vec<(isize, isize)>,
}

impl Board {
    fn parse(input: &str) -> Self {
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().map(|c| Tile::from(c)).collect::<Vec<Tile>>())
            .collect();
        let xmax: isize = tiles.iter().map(|row| row.len()).max().unwrap() as isize;
        let ymax: isize = tiles.len() as isize;
        // Retrieve min/max position of each row
        let hdims: Vec<(isize, isize)> = tiles
            .iter()
            .map(|row| {
                let mut filled = row
                    .iter()
                    .enumerate()
                    .filter(|(_, tile)| **tile != Tile::Empty);
                let first = filled.nth(0).unwrap().0 as isize;
                let last = filled.last().unwrap().0 as isize;
                (first, last)
            })
            .collect();
        // Retrieve min/max position of each col
        let vdims: Vec<(isize, isize)> = (0..xmax)
            .map(|x| {
                let mut filled = tiles
                    .iter()
                    .map(|row| match row.get(x as usize) {
                        Some(tile) => *tile,
                        None => Tile::Empty,
                    })
                    .enumerate()
                    .filter(|(_, tile)| *tile != Tile::Empty);
                let first = filled.nth(0).unwrap().0 as isize;
                let last = filled.last().unwrap().0 as isize;
                (first, last)
            })
            .collect();
        Self {
            tiles,
            ymax,
            hdims,
            vdims,
        }
    }
}

impl Maze for Board {
    fn get_tile(&self, x: isize, y: isize) -> Tile {
        self.tiles[y as usize][x as usize]
    }

    fn next_pos(&self, pos: &Pos) -> Pos {
        let (x, y) = match pos.dir {
            Direction::Up if pos.y == self.vdims[pos.x as usize].0 => {
                (pos.x, self.vdims[pos.x as usize].1)
            }
            Direction::Up => (pos.x, pos.y - 1),
            Direction::Down if pos.y == self.vdims[pos.x as usize].1 => {
                (pos.x, self.vdims[pos.x as usize].0)
            }
            Direction::Down => (pos.x, pos.y + 1),
            Direction::Right if pos.x == self.hdims[pos.y as usize].1 => {
                (self.hdims[pos.y as usize].0, pos.y)
            }
            Direction::Right => (pos.x + 1, pos.y),
            Direction::Left if pos.x == self.hdims[pos.y as usize].0 => {
                (self.hdims[pos.y as usize].1, pos.y)
            }
            Direction::Left => (pos.x - 1, pos.y),
        };
        Pos {
            x,
            y,
            dir: pos.dir.clone(),
        }
    }
}

#[derive(Debug)]
enum Command {
    TurnRight,
    TurnLeft,
    MoveOn(isize),
}

impl Command {
    fn parse(token: &str) -> Command {
        if let Ok(number) = token.parse::<isize>() {
            Command::MoveOn(number)
        } else if token == "R" {
            Command::TurnRight
        } else if token == "L" {
            Command::TurnLeft
        } else {
            panic!("Wrong command")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CornerId {
    FTL,
    FTR,
    FBL,
    FBR,
    BTL,
    BTR,
    BBL,
    BBR,
}

#[derive(Debug, Clone)]
struct Corner {
    id: CornerId,
    x: isize,
    y: isize,
}

#[derive(Clone, Debug)]
struct Face {
    id: FaceId,
    dir: Direction,
    x: isize,
    y: isize,
}

impl Face {
    fn corners(&self) -> HashMap<CornerId, Corner> {
        let coords = [
            (self.x + 49, self.y),
            (self.x + 49, self.y + 49),
            (self.x, self.y + 49),
            (self.x, self.y),
        ];
        let start = self.dir as usize;
        let ids = match self.id {
            FaceId::Front => [CornerId::FTL, CornerId::FTR, CornerId::FBR, CornerId::FBL],
            FaceId::Top => [CornerId::BTL, CornerId::BTR, CornerId::FTR, CornerId::FTL],
            FaceId::Back => [CornerId::BBL, CornerId::BBR, CornerId::BTR, CornerId::BTL],
            FaceId::Bottom => [CornerId::FBL, CornerId::FBR, CornerId::BBR, CornerId::BBL],
            FaceId::Left => [CornerId::BTL, CornerId::FTL, CornerId::FBL, CornerId::BBL],
            FaceId::Right => [CornerId::FTR, CornerId::BTR, CornerId::BBR, CornerId::FBR],
        };
        let mut corners = Vec::new();
        for i in 0..4 {
            corners.push(Corner {
                id: ids[i].clone(),
                x: coords[(i + start) % 4].0,
                y: coords[(i + start) % 4].1,
            });
        }
        corners
            .iter()
            .map(|corner| (corner.id, corner.to_owned()))
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum FaceId {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

struct Cube {
    board: Board,
    faces: HashMap<FaceId, Face>,
}

impl Cube {
    fn new(board: Board) -> Self {
        // Find 'front'
        let x = board.tiles[0]
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile != Tile::Empty)
            .nth(0)
            .unwrap()
            .0 as isize;
        let front = Face {
            id: FaceId::Front,
            dir: Direction::Up,
            x,
            y: 0,
        };
        // Build list of square in the map
        let mut regions: HashMap<(isize, isize), bool> = HashMap::new();
        for y in (0..board.ymax).step_by(50) {
            for x in (0..board.hdims[y as usize].1).step_by(50) {
                match board.tiles[y as usize][x as usize] {
                    Tile::Empty => {}
                    Tile::Slot | Tile::Wall => {
                        regions.insert((x, y), false);
                    }
                }
            }
        }
        // Associate square to Faces
        let mut faces = HashMap::from([(front.id.clone(), front.clone())]);
        Self::map_faces(&mut regions, &front, &mut faces);
        Self { board, faces }
    }

    fn map_faces(
        regions: &mut HashMap<(isize, isize), bool>,
        face: &Face,
        faces: &mut HashMap<FaceId, Face>,
    ) {
        for dir in [Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
            // Compute new position
            let (x, y) = match dir {
                Direction::Down => (face.x, face.y + 50),
                Direction::Up => (face.x, face.y - 50),
                Direction::Left => (face.x - 50, face.y),
                Direction::Right => (face.x + 50, face.y),
            };
            // Is it a region that hasn't been visited yet?
            if x < 0 || y < 0 {
                continue;
            }
            let x = x;
            let y = y;
            match regions.get(&(x, y)) {
                None => continue,
                Some(visited) if *visited => continue,
                _ => (),
            }
            regions.insert((x, y), true);
            // Identify the face.
            let (neighbour_id, neighbour_dir) =
                Self::neighbour(face.id.clone(), face.dir.clone(), dir);
            // Store mapped face
            let neighbour = Face {
                id: neighbour_id.clone(),
                dir: neighbour_dir.clone(),
                x,
                y,
            };
            faces.insert(neighbour_id.clone(), neighbour.clone());
            // Recurse.
            Self::map_faces(regions, &neighbour, faces);
        }
    }

    fn neighbour(faceid: FaceId, orientation: Direction, direction: Direction) -> (FaceId, Direction) {
        match orientation {
            Direction::Right => {
                let (nfaceid, norientation) =
                    Self::neighbour(faceid, orientation.turn_left(), direction.turn_left());
                (nfaceid, norientation.turn_right())
            }
            Direction::Left => {
                let (nfaceid, norientation) =
                    Self::neighbour(faceid, orientation.turn_right(), direction.turn_right());
                (nfaceid, norientation.turn_left())
            }
            Direction::Down => {
                let (nfaceid, norientation) = Self::neighbour(
                    faceid,
                    orientation.u_turn(),
                    direction.u_turn(),
                );
                (nfaceid, norientation.u_turn())
            }
            Direction::Up => match (faceid, direction) {
                (FaceId::Front, Direction::Up) => (FaceId::Top, Direction::Up),
                (FaceId::Front, Direction::Right) => (FaceId::Right, Direction::Up),
                (FaceId::Front, Direction::Down) => (FaceId::Bottom, Direction::Up),
                (FaceId::Front, Direction::Left) => (FaceId::Left, Direction::Up),
                (FaceId::Bottom, Direction::Up) => (FaceId::Front, Direction::Up),
                (FaceId::Bottom, Direction::Right) => (FaceId::Right, Direction::Right),
                (FaceId::Bottom, Direction::Down) => (FaceId::Back, Direction::Up),
                (FaceId::Bottom, Direction::Left) => (FaceId::Left, Direction::Left),
                (FaceId::Back, Direction::Up) => (FaceId::Bottom, Direction::Up),
                (FaceId::Back, Direction::Right) => (FaceId::Right, Direction::Down),
                (FaceId::Back, Direction::Down) => (FaceId::Top, Direction::Up),
                (FaceId::Back, Direction::Left) => (FaceId::Left, Direction::Down),
                (FaceId::Top, Direction::Up) => (FaceId::Back, Direction::Up),
                (FaceId::Top, Direction::Right) => (FaceId::Right, Direction::Left),
                (FaceId::Top, Direction::Down) => (FaceId::Front, Direction::Up),
                (FaceId::Top, Direction::Left) => (FaceId::Left, Direction::Right),
                (FaceId::Left, Direction::Up) => (FaceId::Top, Direction::Left),
                (FaceId::Left, Direction::Right) => (FaceId::Front, Direction::Up),
                (FaceId::Left, Direction::Down) => (FaceId::Bottom, Direction::Right),
                (FaceId::Left, Direction::Left) => (FaceId::Back, Direction::Down),
                (FaceId::Right, Direction::Up) => (FaceId::Top, Direction::Right),
                (FaceId::Right, Direction::Right) => (FaceId::Back, Direction::Down),
                (FaceId::Right, Direction::Down) => (FaceId::Bottom, Direction::Left),
                (FaceId::Right, Direction::Left) => (FaceId::Front, Direction::Up),
            },
        }
    }

    fn get_face(&self, x: isize, y: isize) -> Face {
        for face in self.faces.values() {
            if face.x <= x && x < face.x + 50 && face.y <= y && y < face.y + 50 {
                return face.clone();
            }
        }
        panic!("I should never get to that point.");
    }
}

impl Maze for Cube {
    fn get_tile(&self, x: isize, y: isize) -> Tile {
        self.board.get_tile(x, y)
    }

    fn next_pos(&self, pos: &Pos) -> Pos {
        let x = pos.x;
        let y = pos.y;
        let (x_, y_) = match pos.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        };

        let mut x__ = x_;
        let mut y__ = y_;
        let mut dir__ = pos.dir;

        // if the position is not anymore on a till, compute new face/position
        if x_ < self.board.hdims[pos.y as usize].0
            || x_ > self.board.hdims[pos.y as usize].1
            || y_ < self.board.vdims[pos.x as usize].0
            || y_ > self.board.vdims[pos.x as usize].1
        {
            // Retrieve origin face
            let org_face = self.get_face(pos.x, pos.y);
            // Retrieve destination face
            let (dest_face_id, _) =
                Self::neighbour(org_face.id.clone(), org_face.dir, pos.dir);
            let dest_face = &self.faces[&dest_face_id];
            // Find the two common corners of the two faces
            let org_corners = org_face.corners();
            let dest_corners = dest_face.corners();
            let org_corners_ids: HashSet<&CornerId> = org_corners.keys().collect();
            let dest_corners_ids: HashSet<&CornerId> = dest_corners.keys().collect();
            let common_corners_ids: Vec<&&CornerId> =
                org_corners_ids.intersection(&dest_corners_ids).collect();
            // Determine the matrix of rotation between the two faces
            let org_vec = (
                org_corners[common_corners_ids[1]].x - org_corners[common_corners_ids[0]].x,
                org_corners[common_corners_ids[1]].y - org_corners[common_corners_ids[0]].y,
            );
            let dest_vec = (
                dest_corners[common_corners_ids[1]].x - dest_corners[common_corners_ids[0]].x,
                dest_corners[common_corners_ids[1]].y - dest_corners[common_corners_ids[0]].y,
            );
            let determinant = org_vec.0 * dest_vec.1 - org_vec.1 * dest_vec.0;
            let scalar = org_vec.0 * dest_vec.0 + org_vec.1 * dest_vec.1;
            let mat = if determinant == 0 {
                if scalar >= 0 {
                    (1, 0, 0, 1)
                } else {
                    dir__ = dir__.u_turn();
                    (-1, 0, 0, -1)
                }
            } else if determinant > 0 {
                dir__ = dir__.turn_right();
                (0, -1, 1, 0)
            } else {
                dir__ = dir__.turn_left();
                (0, 1, -1, 0)
            };
            // Determine vector of translation between the two faces
            let mut org_matching_till = org_corners[common_corners_ids[0]].clone();
            match pos.dir {
                Direction::Up => org_matching_till.y -= 1,
                Direction::Down => org_matching_till.y += 1,
                Direction::Left => org_matching_till.x -= 1,
                Direction::Right => org_matching_till.x += 1,
            }
            let dest_matching_till = dest_corners[common_corners_ids[0]].clone();
            let dx =
                dest_matching_till.x - mat.0 * org_matching_till.x - mat.1 * org_matching_till.y;
            let dy =
                dest_matching_till.y - mat.2 * org_matching_till.x - mat.3 * org_matching_till.y;
            // Compute new position
            x__ = mat.0 * x_ + mat.1 * y_ + dx;
            y__ = mat.2 * x_ + mat.3 * y_ + dy;
        };
        Pos {
            x: x__,
            y: y__,
            dir: dir__,
        }
    }
}

fn main() {
    // Read input
    let input = read_to_string("./data/input.txt").unwrap();
    let board = Board::parse(&input);
    let commands: Vec<Command> = input
        .lines()
        .last()
        .unwrap()
        .chars()
        .group_by(|c| c.is_ascii_digit())
        .into_iter()
        .map(|(_, group)| Command::parse(&group.collect::<String>()))
        .collect();
    let start = Pos {
        x: board.tiles[0]
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile != Tile::Empty)
            .nth(0)
            .unwrap()
            .0 as isize,
        y: 0,
        dir: Direction::Right,
    };

    // Part 1
    let mut pos = start.clone();
    for command in &commands {
        pos.execute(&command, &board);
    }
    let res = 1000 * (pos.y + 1) + 4 * (pos.x + 1) + pos.dir as isize;
    println!("Part 1: {:?}", res);

    // Part 2
    let mut pos = start.clone();
    let cube = Cube::new(board);
    for command in &commands {
        pos.execute(&command, &cube);
    }
    let res = 1000 * (pos.y + 1) + 4 * (pos.x + 1) + pos.dir as isize;
    println!("Part 2: {:?}", res);
}
