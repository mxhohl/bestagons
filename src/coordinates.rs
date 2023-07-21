use std::{ops::{Add, Sub, Mul, AddAssign, Index, IndexMut, MulAssign, SubAssign}, cmp::max, vec};

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use crate::directions::{Direction, DiagonalDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    data: [i32; 3]
}

impl Coordinates {

    pub fn zero() -> Self {
        Self {
            data: [0, 0, 0]
        }
    }

    pub fn from_qr(q: i32, r: i32) -> Self {
        Self {
            data: [q, r, -q - r],
        }
    }

    pub fn from_qs(q: i32, s: i32) -> Self {
        Self {
            data: [q, -q - s, s],
        }
    }

    pub fn from_rs(r: i32, s: i32) -> Self {
        Self {
            data: [-r - s, r, s],
        }
    }

    pub fn from_floating(q: f64, r: f64, s: f64) -> Self {
        let mut rounded_q = q.round();
        let mut rounded_r = r.round();
        let mut rounded_s = s.round();

        let dq = (rounded_q - q).abs();
        let dr = (rounded_r - r).abs();
        let ds = (rounded_s - s).abs();

        if dq > dr && dq > ds {
            rounded_q = -r - s;
        } else if dr > ds {
            rounded_r = -q - s;
        } else {
            rounded_s = -q - r;
        }

        Self { data: [
            rounded_q as i32,
            rounded_r as i32,
            rounded_s as i32
        ]}
    }

    #[cfg(not(features = "use_flat_top"))]
    pub fn from_direction(direction: Direction) -> Self {
        match direction {
            Direction::East => Self::from_qr(1, 0),
            Direction::NorthEast => Self::from_qr(1, -1),
            Direction::NorthWest => Self::from_qr(0, -1),
            Direction::West => Self::from_qr(-1, 0),
            Direction::SouthWest => Self::from_qr(-1, 1),
            Direction::SouthEast => Self::from_qr(0, 1),
        }
    }

    #[cfg(not(features = "use_flat_top"))]
    pub fn from_diagonal_direction(direction: DiagonalDirection) -> Self {
        match direction {
            DiagonalDirection::NorthEast => Self::from_qr(1, -1),
            DiagonalDirection::North => Self::from_qr(0, -1),
            DiagonalDirection::NorthWest => Self::from_qr(-1, 0),
            DiagonalDirection::SouthWest => Self::from_qr(-1, 1),
            DiagonalDirection::South => Self::from_qr(0, 1),
            DiagonalDirection::SouthEast => Self::from_qr(1, 0),
        }
    }

    #[cfg(features = "use_flat_top")]
    pub fn from_direction(direction: Direction) -> Self {
        match direction {
            Direction::NorthEast => Self::from_qr(1, -1),
            Direction::North => Self::from_qr(0, -1),
            Direction::NorthWest => Self::from_qr(-1, 0),
            Direction::SouthWest => Self::from_qr(-1, 1),
            Direction::South => Self::from_qr(0, 1),
            Direction::SouthEast => Self::from_qr(1, 0),
        }
    }

    #[cfg(features = "use_flat_top")]
    pub fn from_diagonal_direction(direction: DiagonalDirection) -> Self {
        match direction {
            DiagonalDirection::East => Self::from_qr(1, 0),
            DiagonalDirection::NorthEast => Self::from_qr(1, -1),
            DiagonalDirection::NorthWest => Self::from_qr(0, -1),
            DiagonalDirection::West => Self::from_qr(-1, 0),
            DiagonalDirection::SouthWest => Self::from_qr(-1, 1),
            DiagonalDirection::SouthEast => Self::from_qr(0, 1),
        }
    }

    #[cfg(not(features = "use_flat_top"))]
    pub fn from_offset_oddr(col: i32, row: i32) -> Self {
        let q = col - (row - (if row & 1 == 0 { 0 } else { 1 })) / 2;
        let r = row;
        Self::from_qr(q, r)
    }

    #[cfg(not(features = "use_flat_top"))]
    pub fn from_offset_evenr(col: i32, row: i32) -> Self {
        let q = col - (row + (if row & 1 == 0 { 0 } else { 1 })) / 2;
        let r = row;
        Self::from_qr(q, r)
    }

    #[cfg(features = "use_flat_top")]
    pub fn from_offset_oddq(col: i32, row: i32) -> Self {
        let q = col;
        let r = row - (col - (if row.is_even() { 0 } else { 1 })) / 2;
        Self::from_qr(q, r)
    }

    #[cfg(features = "use_flat_top")]
    pub fn from_offset_evenq(col: i32, row: i32) -> Self {
        let q = col;
        let r = row - (col + (if row.is_even() { 0 } else { 1 })) / 2;
        Self::from_qr(q, r)
    }

    pub fn is_valid(&self) -> bool {
        self.data[0] + self.data[1] + self.data[2] == 0
    }

    pub fn q(&self) -> i32 {
        self.data[0]
    }

    pub fn r(&self) -> i32 {
        self.data[1]
    }

    pub fn s(&self) -> i32 {
        self.data[2]
    }

    pub fn q_mut(&mut self) -> &mut i32 {
        &mut self.data[0]
    }

    pub fn r_mut(&mut self) -> &mut i32 {
        &mut self.data[1]
    }

    pub fn s_mut(&mut self) -> &mut i32 {
        &mut self.data[2]
    }

    pub fn distance_to(&self, rhs: Self) -> i32 {
        std::cmp::max(
            std::cmp::max(
                (self.data[0] - rhs.data[0]).abs(),
                (self.data[1] - rhs.data[1]).abs()
            ),
            (self.data[2] - rhs.data[2]).abs()
        )
    }

    pub fn neighbor(&self, direction: Direction) -> Coordinates {
        self + Self::from_direction(direction)
    }

    pub fn is_neighbor(&self, rhs: Self) -> bool {
        self.distance_to(rhs) == 1
    }

    pub fn diagonal(&self, direction: DiagonalDirection) -> Coordinates {
        self + Self::from_diagonal_direction(direction)
    }

    pub fn line_to(&self, rhs: Self) -> Vec<Self> {
        let n = max(self.distance_to(rhs), 1);
        let step = 1. / n as f64;

        let mut results = vec![];
        for i in 0..n {
            results.push(lerp(*self, rhs, step * i));
        }
        results
    }

}

impl Index<usize> for Coordinates {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Coordinates {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
            ],
        }
    }
}

forward_ref_binop!(impl Add, add for Coordinates, Coordinates);

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] = self.data[0] + rhs.data[0];
        self.data[1] = self.data[1] + rhs.data[1];
        self.data[2] = self.data[2] + rhs.data[2];
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for Coordinates, Coordinates);

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
            ],
        }
    }
}

forward_ref_binop!(impl Sub, sub for Coordinates, Coordinates);

impl SubAssign for Coordinates {
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0] = self.data[0] - rhs.data[0];
        self.data[1] = self.data[1] - rhs.data[1];
        self.data[2] = self.data[2] - rhs.data[2];
    }
}

forward_ref_op_assign!(impl SubAssign, sub_assign for Coordinates, Coordinates);

impl Mul<i32> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs,
            ],
        }
    }
}

forward_ref_binop!(impl Mul, mul for Coordinates, i32);

impl MulAssign<i32> for Coordinates {
    fn mul_assign(&mut self, rhs: i32) {
        self.data[0] = self.data[0] * rhs;
        self.data[1] = self.data[1] * rhs;
        self.data[2] = self.data[2] * rhs;
    }
}

forward_ref_op_assign!(impl MulAssign, mul_assign for Coordinates, i32);

fn lerp_i32(lhs: i32, rhs: i32, t: f64) -> f64 {
    lhs as f64 * (1. - t) + rhs as f64 * t
}

pub fn lerp(lhs: Coordinates, rhs: Coordinates, t: f64) -> Coordinates {
    Coordinates::from_floating(
        lerp_i32(lhs.q(), rhs.q(), t),
        lerp_i32(lhs.r(), rhs.r(), t),
        lerp_i32(lhs.s(), rhs.s(), t)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let coords =  Coordinates::from_qr(2, 2);
        assert_eq!(coords.s(), -4);
        assert!(coords.is_valid());

        let coords =  Coordinates::from_qs(2, 2);
        assert_eq!(coords.r(), -4);
        assert!(coords.is_valid());

        let coords =  Coordinates::from_rs(2, 2);
        assert_eq!(coords.q(), -4);
        assert!(coords.is_valid());
    }
}
