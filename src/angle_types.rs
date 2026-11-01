use num_traits::{Euclid, Float, FloatConst};
use serde::Serialize;

#[derive(Copy, Clone, Debug, Serialize)]
pub enum Angle<T> {
    Degrees(T),
    Radians(T),
    Turns(T),
}

impl<T: Float + FloatConst + Euclid + Copy> Default for Angle<T> {
    fn default() -> Self {
        Self::Radians(T::zero())
    }
}

impl<T: Float + FloatConst + Euclid + Copy> Angle<T> {
    //Convert between angle types
    pub fn to_radians(self) -> Self {
        Self::Radians(self.in_radians())
    }

    pub fn to_degrees(self) -> Self {
        Self::Degrees(self.in_degrees())
    }

    pub fn to_turns(self) -> Self {
        Self::Turns(self.in_turns())
    }

    //Convert the value between angle types
    pub fn in_radians(self) -> T {
        match self {
            Self::Degrees(v) => v * T::PI() / T::from(180.).unwrap(),
            Self::Radians(v) => v,
            Self::Turns(v) => v * T::TAU(),
        }
    }
    
    pub fn in_degrees(self) -> T {
        match self {
            Self::Degrees(v) => v,
            Self::Radians(v) => v * T::from(180.).unwrap() * T::FRAC_1_PI(),
            Self::Turns(v) => v * T::from(360.).unwrap(),
        }
    }
    
    pub fn in_turns(self) -> T {
        match self {
            Self::Degrees(v) => v * T::from(1./360.).unwrap(),
            Self::Radians(v) => v * T::one() / T::TAU(),
            Self::Turns(v) => v,
        }
    }

    pub fn wrapped(self) -> Self{
        match self {
            Self::Degrees(v) => Self::Degrees(v.rem_euclid(&T::from(360.0).unwrap())),
            Self::Radians(v) => Self::Radians(v.rem_euclid(&T::TAU())),
            Self::Turns(v) => Self::Turns(v.rem_euclid(&T::one())),
        }
    }

    pub fn add(&mut self, angle: Angle<T>) -> Angle<T> {
        match self {
            Self::Degrees(v) => *v =  *v + angle.in_degrees(),
            Self::Radians(v) => *v = *v + angle.in_radians(),
            Self::Turns(v) => *v =  *v + angle.in_turns(),
        }
        *self
    }

        pub fn multiply(&mut self, operand: T) -> Angle<T> {
        match self {
            Self::Degrees(v) => *v =  *v * operand,
            Self::Radians(v) => *v = *v * operand,
            Self::Turns(v) => *v =  *v * operand,
        }
        *self
    }

    //Trig ratios
    pub fn sin(self) -> T {
        self.in_radians().sin()
    }
    
    pub fn cos(self) -> T {
        self.in_radians().cos()
    }
    
    pub fn tan(self) -> T {
        self.in_radians().tan()
    }

    pub fn sinh(self) -> T {
        self.in_radians().sinh()
    }

    pub fn cosh(self) -> T {
        self.in_radians().cosh()
    }

    pub fn tanh(self) -> T {
        self.in_radians().tanh()
    }

    //Trig angles
    pub fn asin(v: T) -> Self {
        Self::Radians(v.asin())
    }

    pub fn acos(v: T) -> Self {
        Self::Radians(v.acos())
    }

    pub fn atan(v: T) -> Self {
        Self::Radians(v.atan())
    }

    pub fn atan2(y: T, v: T) -> Self {
        Self::Radians(y.atan2(v))
    }

    pub fn asinh(v: T) -> Self {
        Self::Radians(v.asinh())
    }    
    
    pub fn acosh(v: T) -> Self {
        Self::Radians(v.acosh())
    }

    pub fn atanh(v: T) -> Self {
        Self::Radians(v.atanh())
    }
    
}


