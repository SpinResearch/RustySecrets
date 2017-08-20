//! This module provides the Gf256 type which is used to represent
//! elements of a finite field with 256 elements.

use std::ops::{Add, Div, Mul, Sub};

include!(concat!(env!("OUT_DIR"), "/nothinghardcoded.rs"));

fn get_tables() -> &'static Tables {
    &TABLES
}

/// Type for elements of a finite field with 256 elements
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Gf256 {
    pub poly: u8,
}

impl Gf256 {
    /// returns the additive neutral element of the field
    #[inline]
    pub fn zero() -> Gf256 {
        Gf256 { poly: 0 }
    }
    /// returns the multiplicative neutral element of the field
    #[inline]
    pub fn one() -> Gf256 {
        Gf256 { poly: 1 }
    }
    #[inline]
    pub fn from_byte(b: u8) -> Gf256 {
        Gf256 { poly: b }
    }
    #[inline]
    pub fn to_byte(&self) -> u8 {
        self.poly
    }
    pub fn exp(power: u8) -> Gf256 {
        let tabs = get_tables();
        Gf256::from_byte(tabs.exp[power as usize])
    }
    pub fn log(&self) -> Option<u8> {
        if self.poly == 0 {
            None
        } else {
            let tabs = get_tables();
            Some(tabs.log[self.poly as usize])
        }
    }
    pub fn pow(&self, mut exp: u8) -> Gf256 {
        let mut base = *self;
        let mut acc = Self::one();

        while exp > 1 {
            if (exp & 1) == 1 {
                acc = acc * base;
            }
            exp /= 2;
            base = base * base;
        }

        if exp == 1 {
            acc = acc * base;
        }

        acc
    }
}

impl Add<Gf256> for Gf256 {
    type Output = Gf256;
    #[inline]
    fn add(self, rhs: Gf256) -> Gf256 {
        Gf256::from_byte(self.poly ^ rhs.poly)
    }
}

impl Sub<Gf256> for Gf256 {
    type Output = Gf256;
    #[inline]
    fn sub(self, rhs: Gf256) -> Gf256 {
        Gf256::from_byte(self.poly ^ rhs.poly)
    }
}

impl Mul<Gf256> for Gf256 {
    type Output = Gf256;
    fn mul(self, rhs: Gf256) -> Gf256 {
        if let (Some(l1), Some(l2)) = (self.log(), rhs.log()) {
            let tmp = ((l1 as u16) + (l2 as u16)) % 255;
            Gf256::exp(tmp as u8)
        } else {
            Gf256 { poly: 0 }
        }
    }
}

impl Div<Gf256> for Gf256 {
    type Output = Gf256;
    fn div(self, rhs: Gf256) -> Gf256 {
        let l2 = rhs.log().expect("division by zero");
        if let Some(l1) = self.log() {
            let tmp = ((l1 as u16) + 255 - (l2 as u16)) % 255;
            Gf256::exp(tmp as u8)
        } else {
            Gf256 { poly: 0 }
        }
    }
}
