use crate::utils::position::Position;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Facing {
    East,
    South,
    West,
    North,
}

impl Facing {
    pub fn apply(&self, position: &Position) -> Position {
        match self {
            Facing::East => Position { x: position.x + 1, y: position.y },
            Facing::West => Position { x: position.x - 1, y: position.y },
            Facing::South => Position { x: position.x, y: position.y + 1 },
            Facing::North => Position { x: position.x, y: position.y - 1 },
        }
    }

    pub fn all() -> Vec<Facing> {
        vec![Facing::North, Facing::East, Facing::South, Facing::West]
    }

    pub fn adjacents(&self) -> Vec<Facing> {
        Facing::all()
            .into_iter()
            .filter(|f| *f != *self)
            .collect::<Vec<Facing>>()
    }

    pub fn opposite(&self) -> Facing {
        match self {
            Facing::East => Facing::West,
            Facing::South => Facing::North,
            Facing::West => Facing::East,
            Facing::North => Facing::South,
        }
    }
}