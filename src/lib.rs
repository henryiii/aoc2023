#![doc = include_str!("../README.md")]

/*!

## aoc2023 crate


A few problems use repeated items, so those are provided here.

*/

/*/
This has helpers for 2D problems.
*/
pub mod grid_helper {
    use core::ops::Add;

    use derive_more::Constructor;
    use grid::Grid;
    use strum::EnumIter;

    /// This is a direction. Can be converted to a bitflags-like u8.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
    #[repr(u8)]
    pub enum Direction {
        Up = 0x01,
        Down = 0x02,
        Left = 0x04,
        Right = 0x08,
    }

    impl Direction {
        /// This rotates the direction clockwise.
        #[must_use]
        pub const fn clockwise(&self) -> Self {
            use Direction::{Down, Left, Right, Up};

            match self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            }
        }

        /// This rotates the direction counter-clockwise.
        #[must_use]
        pub const fn counter_clockwise(&self) -> Self {
            use Direction::{Down, Left, Right, Up};

            match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            }
        }
    }

    /// This is a helper for a signed position. You can add a direction to step
    /// in that direction. You can try convert to a classic (usize, usize)
    /// position.
    #[derive(Debug, Constructor, Copy, Clone)]
    pub struct Position(isize, isize);

    impl Add<Direction> for Position {
        type Output = Self;

        fn add(self, dir: Direction) -> Self {
            use Direction::{Down, Left, Right, Up};

            match dir {
                Up => Self(self.0 - 1, self.1),
                Down => Self(self.0 + 1, self.1),
                Left => Self(self.0, self.1 - 1),
                Right => Self(self.0, self.1 + 1),
            }
        }
    }

    impl TryFrom<Position> for (usize, usize) {
        type Error = std::num::TryFromIntError;

        fn try_from(pos: Position) -> Result<Self, Self::Error> {
            Ok((usize::try_from(pos.0)?, usize::try_from(pos.1)?))
        }
    }

    impl From<Position> for (isize, isize) {
        fn from(pos: Position) -> Self {
            (pos.0, pos.1)
        }
    }

    /// This trait is for getting a value from a grid, but returning None if the
    /// position is out of bounds. Unlike `grid::Grid::get`, this takes signed
    /// integers. This might be fixed upstream soon in
    /// <https://github.com/becheran/grid/pull/41>.
    pub trait CheckedGet<T> {
        fn checked_get(&self, pos: impl TryInto<(isize, isize)>) -> Option<&T>;
    }

    impl<T> CheckedGet<T> for Grid<T> {
        fn checked_get(&self, pos: impl TryInto<(isize, isize)>) -> Option<&T> {
            let pos = pos.try_into().ok()?;
            let y = usize::try_from(pos.0).ok()?;
            let x = usize::try_from(pos.1).ok()?;
            self.get(y, x)
        }
    }
}
