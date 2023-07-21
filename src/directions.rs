use std::fmt::Display;

#[cfg(not(features = "use_flat_top"))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    East = 0,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

#[cfg(not(features = "use_flat_top"))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiagonalDirection {
    NorthEast = 0,
    North,
    NorthWest,
    SouthWest,
    South,
    SouthEast,
}

#[cfg(features = "use_flat_top")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    NorthEast = 0,
    North,
    NorthWest,
    SouthWest,
    South,
    SouthEast,
}

#[cfg(features = "use_flat_top")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiagonalDirection {
    East = 0,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for DiagonalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(not(features = "use_flat_top"))]
impl Default for Direction {
    fn default() -> Self {
        Self::East
    }
}

#[cfg(not(features = "use_flat_top"))]
impl Default for DiagonalDirection {
    fn default() -> Self {
        Self::NorthEast
    }
}

#[cfg(features = "use_flat_top")]
impl Default for Direction {
    fn default() -> Self {
        Self::NorthEast
    }
}

#[cfg(features = "use_flat_top")]
impl Default for DiagonalDirection {
    fn default() -> Self {
        Self::East
    }
}

#[cfg(not(features = "use_flat_top"))]
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::NorthEast => Self::SouthWest,
            Self::NorthWest => Self::SouthEast,
            Self::West => Self::East,
            Self::SouthWest => Self::NorthEast,
            Self::SouthEast => Self::NorthWest,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::East => Self::NorthEast,
            Self::NorthEast => Self::NorthWest,
            Self::NorthWest => Self::West,
            Self::West => Self::SouthWest,
            Self::SouthWest => Self::SouthEast,
            Self::SouthEast => Self::East,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::East => Self::SouthEast,
            Self::NorthEast => Self::East,
            Self::NorthWest => Self::NorthEast,
            Self::West => Self::NorthWest,
            Self::SouthWest => Self::West,
            Self::SouthEast => Self::SouthWest,
        }
    }
}

#[cfg(not(features = "use_flat_top"))]
impl DiagonalDirection {
    pub fn opposite(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::North => Self::South,
            Self::NorthWest => Self::SouthEast,
            Self::SouthWest => Self::NorthEast,
            Self::South => Self::North,
            Self::SouthEast => Self::NorthWest,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::NorthEast => Self::North,
            Self::North => Self::NorthWest,
            Self::NorthWest => Self::SouthWest,
            Self::SouthWest => Self::South,
            Self::South => Self::SouthEast,
            Self::SouthEast => Self::NorthEast,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthEast,
            Self::North => Self::NorthEast,
            Self::NorthWest => Self::North,
            Self::SouthWest => Self::NorthWest,
            Self::South => Self::SouthWest,
            Self::SouthEast => Self::South,
        }
    }
}

#[cfg(features = "use_flat_top")]
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::North => Self::South,
            Self::NorthWest => Self::SouthEast,
            Self::SouthWest => Self::NorthEast,
            Self::South => Self::North,
            Self::SouthEast => Self::NorthWest,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::NorthEast => Self::North,
            Self::North => Self::NorthWest,
            Self::NorthWest => Self::SouthWest,
            Self::SouthWest => Self::South,
            Self::South => Self::SouthEast,
            Self::SouthEast => Self::NorthEast,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::NorthEast => Self::SouthEast,
            Self::North => Self::NorthEast,
            Self::NorthWest => Self::North,
            Self::SouthWest => Self::NorthWest,
            Self::South => Self::SouthWest,
            Self::SouthEast => Self::South,
        }
    }
}

#[cfg(features = "use_flat_top")]
impl DiagonalDirection {
    pub fn opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::NorthEast => Self::SouthWest,
            Self::NorthWest => Self::SouthEast,
            Self::West => Self::East,
            Self::SouthWest => Self::NorthEast,
            Self::SouthEast => Self::NorthWest,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::East => Self::NorthEast,
            Self::NorthEast => Self::NorthWest,
            Self::NorthWest => Self::West,
            Self::West => Self::SouthWest,
            Self::SouthWest => Self::SouthEast,
            Self::SouthEast => Self::East,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::East => Self::SouthEast,
            Self::NorthEast => Self::East,
            Self::NorthWest => Self::NorthEast,
            Self::West => Self::NorthWest,
            Self::SouthWest => Self::West,
            Self::SouthEast => Self::SouthWest,
        }
    }
}

impl IntoIterator for Direction {
    type Item = Direction;
    type IntoIter = DirectionIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionIter {
            direction: Some(self)
        }
    }
}

pub struct DirectionIter {
    direction: Option<Direction>,
}

impl Iterator for DirectionIter {
    type Item = Direction;

    #[cfg(not(features = "use_flat_top"))]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.direction;

        use Direction::*;
        match self.direction {
            None => {},
            Some(East) => {
                self.direction = Some(NorthEast);
            }
            Some(NorthEast) => {
                self.direction = Some(NorthWest);
            }
            Some(NorthWest) => {
                self.direction = Some(West);
            }
            Some(West) => {
                self.direction = Some(SouthWest);
            }
            Some(SouthWest) => {
                self.direction = Some(SouthEast);
            }
            Some(SouthEast) => {
                self.direction = None;
            }
        }

        current
    }

    #[cfg(features = "use_flat_top")]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.direction;

        use Direction::*;
        match self.direction {
            None => {},
            Some(NorthEast) => {
                self.direction = Some(North);
            }
            Some(North) => {
                self.direction = Some(NorthWest);
            }
            Some(NorthWest) => {
                self.direction = Some(SouthWest);
            }
            Some(SouthWest) => {
                self.direction = Some(South);
            }
            Some(South) => {
                self.direction = Some(SouthEast);
            }
            Some(SouthEast) => {
                self.direction = None;
            }
        }

        current
    }
}

impl IntoIterator for DiagonalDirection {
    type Item = DiagonalDirection;
    type IntoIter = DiagonalDirectionIter;

    fn into_iter(self) -> Self::IntoIter {
        DiagonalDirectionIter {
            direction: Some(self)
        }
    }
}

pub struct DiagonalDirectionIter {
    direction: Option<DiagonalDirection>,
}

impl Iterator for DiagonalDirectionIter {
    type Item = DiagonalDirection;

    #[cfg(not(features = "use_flat_top"))]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.direction;

        use DiagonalDirection::*;
        match self.direction {
            None => {},
            Some(NorthEast) => {
                self.direction = Some(North);
            }
            Some(North) => {
                self.direction = Some(NorthWest);
            }
            Some(NorthWest) => {
                self.direction = Some(SouthWest);
            }
            Some(SouthWest) => {
                self.direction = Some(South);
            }
            Some(South) => {
                self.direction = Some(SouthEast);
            }
            Some(SouthEast) => {
                self.direction = None;
            }
        }

        current
    }

    #[cfg(features = "use_flat_top")]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.direction;

        use DiagonalDirection::*;
        match self.direction {
            None => {},
            Some(East) => {
                self.direction = Some(NorthEast);
            }
            Some(NorthEast) => {
                self.direction = Some(NorthWest);
            }
            Some(NorthWest) => {
                self.direction = Some(West);
            }
            Some(West) => {
                self.direction = Some(SouthWest);
            }
            Some(SouthWest) => {
                self.direction = Some(SouthEast);
            }
            Some(SouthEast) => {
                self.direction = None;
            }
        }

        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(features = "use_flat_top"))]
    #[test]
    fn opposite() {
        let check_dir = |d: Direction| assert_eq!(d, d.opposite().opposite());
        check_dir(Direction::East);
        check_dir(Direction::NorthEast);
        check_dir(Direction::NorthWest);
        check_dir(Direction::West);
        check_dir(Direction::SouthWest);
        check_dir(Direction::SouthEast);

        let check_diag = |d: DiagonalDirection| assert_eq!(d, d.opposite().opposite());
        check_diag(DiagonalDirection::NorthEast);
        check_diag(DiagonalDirection::North);
        check_diag(DiagonalDirection::NorthWest);
        check_diag(DiagonalDirection::SouthWest);
        check_diag(DiagonalDirection::South);
        check_diag(DiagonalDirection::SouthEast);
    }

    #[cfg(features = "use_flat_top")]
    #[test]
    fn opposite() {
        let check_dir = |d: Direction| assert_eq!(d, d.opposite().opposite());
        check_dir(Direction::NorthEast);
        check_dir(Direction::North);
        check_dir(Direction::NorthWest);
        check_dir(Direction::SouthWest);
        check_dir(Direction::South);
        check_dir(Direction::SouthEast);

        let check_diag = |d: DiagonalDirection| assert_eq!(d, d.opposite().opposite());
        check_diag(DiagonalDirection::East);
        check_diag(DiagonalDirection::NorthEast);
        check_diag(DiagonalDirection::NorthWest);
        check_diag(DiagonalDirection::West);
        check_diag(DiagonalDirection::SouthWest);
        check_diag(DiagonalDirection::SouthEast);
    }

    #[test]
    fn next() {
        let mut dir = Direction::NorthEast;
        let mut diag = DiagonalDirection::NorthEast;
        for _ in 0..6 {
            dir = dir.next();
            diag = diag.next();
        }

        assert_eq!(dir, Direction::NorthEast);
        assert_eq!(diag, DiagonalDirection::NorthEast);
    }

    #[test]
    fn previous() {
        let mut dir = Direction::NorthEast;
        let mut diag = DiagonalDirection::NorthEast;
        for _ in 0..6 {
            dir = dir.previous();
            diag = diag.previous();
        }

        assert_eq!(dir, Direction::NorthEast);
        assert_eq!(diag, DiagonalDirection::NorthEast);
    }

    #[test]
    fn next_previous() {
        let mut dir = Direction::NorthEast;
        let mut diag = DiagonalDirection::NorthEast;
        for _ in 0..6 {
            dir = dir.next();
            diag = diag.next();

            assert_eq!(dir, dir.next().previous());
            assert_eq!(diag, diag.next().previous());
        }
    }

    #[test]
    fn iterator() {
        let mut dir = Direction::default();

        let mut count = 0;
        for it_dir in Direction::default().into_iter() {
            assert_eq!(it_dir, dir);

            dir = dir.next();
            count += 1;
        }

        assert_eq!(count, 6, "An hexagon must have six directions.");
    }

    #[test]
    fn iterator_diagonal() {
        let mut diag = DiagonalDirection::default();

        let mut count = 0;
        for it_dir in DiagonalDirection::default().into_iter() {
            assert_eq!(it_dir, diag);

            diag = diag.next();
            count += 1;
        }

        assert_eq!(count, 6, "An hexagon must have six diagonal directions.");
    }

}
