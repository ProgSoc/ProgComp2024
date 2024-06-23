use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position(pub f64);
#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub f64);
#[derive(Copy, Clone, Debug)]
pub struct Acceleration(pub f64);
#[derive(Copy, Clone, Debug)]
pub struct Time(pub f64);
#[derive(Copy, Clone, Debug)]
pub struct Mass(pub f64);
#[derive(Copy, Clone, Debug)]
pub struct Force(pub f64);
#[derive(Copy, Clone, Debug)]
pub struct SpringConstant(pub f64);

impl Mul<Time> for Acceleration {
    type Output = Velocity;

    fn mul(self, time: Time) -> Velocity {
        Velocity(self.0 * time.0)
    }
}

impl Mul<Time> for Velocity {
    type Output = Position;

    fn mul(self, time: Time) -> Position {
        Position(self.0 * time.0)
    }
}

impl Add<Velocity> for Velocity {
    type Output = Velocity;

    fn add(self, other: Velocity) -> Velocity {
        Velocity(self.0 + other.0)
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(self.0 + other.0)
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position(self.0 - other.0)
    }
}

impl Add<Force> for Force {
    type Output = Force;

    fn add(self, other: Force) -> Force {
        Force(self.0 + other.0)
    }
}

impl Mul<Force> for Force {
    type Output = Force;

    fn mul(self, other: Force) -> Force {
        Force(self.0 * other.0)
    }
}

impl Div<Mass> for Force {
    type Output = Acceleration;

    fn div(self, mass: Mass) -> Acceleration {
        Acceleration(self.0 / mass.0)
    }
}

impl Mul<Position> for SpringConstant {
    type Output = Force;

    fn mul(self, x: Position) -> Force {
        Force(self.0 * x.0)
    }
}

impl Add<Time> for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time(self.0 + other.0)
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
