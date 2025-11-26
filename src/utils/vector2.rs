use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::Num;

use crate::utils::step::Step;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2 <T: Num = i32>{
    pub x: T,
    pub y: T
}

impl<T: Num> Vector2<T> {
    pub fn new() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }

    pub fn zero() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }

    pub fn one() -> Self {
        Self { x: T::one(), y: T::one() }
    }

    pub fn from(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

pub trait Move {
    fn move_up(&mut self) -> bool;
    fn move_down(&mut self) -> bool;
    fn move_left(&mut self) -> bool;
    fn move_right(&mut self) -> bool;
}

impl<T: Num + Step> Move for Vector2<T> {
    fn move_up(&mut self) -> bool {
        let (y, overflow) = self.y.sub_step();
        if !overflow {
            self.y = y;
        }
        overflow
    }

    fn move_down(&mut self) -> bool {
        let (y, overflow) = self.y.add_step();
        if !overflow {
            self.y = y;
        }
        overflow
    }

    fn move_left(&mut self) -> bool {
        let (x, overflow) = self.x.sub_step();
        if !overflow {
            self.x = x;
        }
        overflow
    }

    fn move_right(&mut self) -> bool {
        let (x, overflow) = self.x.add_step();
        if !overflow {
            self.x = x;
        }
        overflow
    }
}

impl<T: Num> Add<Self> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Num + Copy> Add<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vector2::<T> {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl<T: Num + AddAssign> AddAssign<Self> for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num + Copy + AddAssign> AddAssign<T> for Vector2<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub<Output = T> + Num,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Sub<T> for Vector2<T>
where
    T: Copy + Sub<Output = T> + Num,
{
    type Output = Vector2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vector2::<T> {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T> SubAssign for Vector2<T>
where
    T: SubAssign + Num,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> SubAssign<T> for Vector2<T>
where
    T: Copy + SubAssign + Num,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Copy + Mul<Output = T> + Num,
{
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector2<T>
where
    T: Copy + MulAssign + Num,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Copy + Div<Output = T> + Num,
{
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::<T> {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector2<T>
where
    T: Copy + DivAssign + Num,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}