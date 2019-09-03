use std::{cmp::Ordering, ops};
use termion::cursor;

pub static ZERO: Coords = Coords { x: 0, y: 0 };

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub const fn from_xy(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }

    pub const fn from_x(x: i32) -> Coords {
        Coords { x, y: 0 }
    }

    pub const fn from_y(y: i32) -> Coords {
        Coords { x: 0, y }
    }

    pub const fn to_x(self) -> Coords {
        Coords { x: self.x, y: 0 }
    }

    pub const fn to_y(self) -> Coords {
        Coords { x: 0, y: self.y }
    }

    pub const fn as_row_col(self) -> (u16, u16) {
        (self.y as u16 + 1, self.x as u16 + 1)
    }
}

impl From<(u16, u16)> for Coords {
    fn from((x, y): (u16, u16)) -> Self {
        Coords::from_xy(i32::from(x) - 1, i32::from(y) - 1)
    }
}

impl From<Coords> for (u16, u16) {
    fn from(coords: Coords) -> Self {
        (coords.x as u16 + 1, coords.y as u16 + 1)
    }
}

impl From<Coords> for cursor::Goto {
    fn from(coords: Coords) -> Self {
        cursor::Goto(coords.x as u16 + 1, coords.y as u16 + 1)
    }
}

impl PartialOrd for Coords {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.x < other.x && self.y < other.y {
            Some(Ordering::Less)
        } else if self.x > other.x && self.y > other.y {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl ops::Neg for Coords {
    type Output = Coords;

    fn neg(self) -> Self::Output {
        Coords {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add<Coords> for Coords {
    type Output = Coords;

    fn add(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign<Coords> for Coords {
    fn add_assign(&mut self, other: Coords) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub<Coords> for Coords {
    type Output = Coords;

    fn sub(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign<Coords> for Coords {
    fn sub_assign(&mut self, other: Coords) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Mul<i32> for Coords {
    type Output = Coords;

    fn mul(self, scalar: i32) -> Self::Output {
        Coords {
            x: scalar * self.x,
            y: scalar * self.y,
        }
    }
}

impl ops::MulAssign<i32> for Coords {
    fn mul_assign(&mut self, scalar: i32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl ops::Mul<Coords> for i32 {
    type Output = Coords;

    fn mul(self, coords: Coords) -> Self::Output {
        coords.mul(self)
    }
}

impl ops::Div<i32> for Coords {
    type Output = Coords;

    fn div(self, scalar: i32) -> Self::Output {
        Coords {
            x: scalar / self.x,
            y: scalar / self.y,
        }
    }
}

impl ops::DivAssign<i32> for Coords {
    fn div_assign(&mut self, scalar: i32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}