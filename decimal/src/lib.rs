#![allow(unused)]

use bigdecimal::BigDecimal;
use num_bigint::{BigInt, BigUint, Sign};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Eq)]
pub struct Decimal {
    sign: Sign,
    integer_part: BigUint, // 整数部分
    frac_part: BigUint,    // 小数部分
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (left, right) = input.split_once(".")?;
        let integer_part = BigUint::from_str(if left.is_empty() { "0" } else { left }).ok()?;
        let frac_part = BigUint::from_str(if right.is_empty() { "0" } else { right }).ok()?;

        Some(Decimal {
            sign: if left.starts_with("-") {
                Sign::Minus
            } else {
                Sign::Plus
            },
            integer_part,
            frac_part,
        })
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign
            && self.integer_part == other.integer_part
            && self.frac_part == other.frac_part
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sign
            .partial_cmp(&other.sign)
            .partial_cmp(&self.integer_part.partial_cmp(&other.integer_part))
            .partial_cmp(&self.frac_part.partial_cmp(&other.frac_part))
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(mut self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Decimal {
    type Output = ();

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for Decimal {
    type Output = Decimal;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}
