use std::mem;
use std::ops::{Add, Sub};
use std::cmp::{PartialOrd, Ord, Ordering};

use serde::{Deserialize, Serialize};

use super::time::BpmEvent;

pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        a %= b;
        mem::swap(&mut a, &mut b);
    }

    a
}

#[derive(
    Debug, Default,
    Clone, Copy,
    PartialEq, Eq,
    Deserialize, Serialize,
)]
pub struct Beat(u32, u32, u32);

impl Beat {
    #[inline]
    pub fn new(
        mut int: u32,
        mut num: u32,
        mut deno: u32
    ) -> Self {
        assert_ne!(deno, 0);

        let gcd = gcd(num, deno);
        num /= gcd;
        deno /= gcd;

        int += num / deno;
        num %= deno;

        Self(int, num, deno)
    }

    #[inline]
    pub const fn integer(&self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn numerator(&self) -> u32 {
        self.1
    }

    #[inline]
    pub const fn denominator(&self) -> u32 {
        self.2
    }

    #[inline]
    pub fn as_time(self, base_bpm: f64) -> f64 {
        f64::from(self) / base_bpm * 60.0
    }

    #[inline]
    pub fn as_time_sv(self, sv: &[BpmEvent]) -> f64 {
        let mut time = 0.0;
        let mut cutoff = Beat::new(0, 0, 1);
        for s in sv.iter() {
            if s.beat > self {
                time += (self - cutoff).as_time(s.bpm);
                break;
            } else {
                time += (s.beat - cutoff).as_time(s.bpm);
                cutoff = s.beat;
            }
        }
        
        time
    }
}

impl From<Beat> for f64 {
    fn from(value: Beat) -> f64 {
        value.0 as f64 + value.1 as f64 / value.2 as f64
    }
}

impl Add for Beat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let denominator = self.denominator().strict_mul(rhs.denominator());
        let numerator = self.numerator()
            .strict_mul(rhs.denominator())
            .strict_add(rhs.numerator().strict_mul(self.denominator()));
        let integer = self.integer().strict_add(rhs.integer());
        
        Beat::new(integer, numerator, denominator)
    }
}

impl Sub for Beat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let denominator = self.denominator().strict_mul(rhs.denominator());

        let lhs_num = self.numerator().strict_mul(rhs.denominator());
        let rhs_num = rhs.numerator().strict_mul(self.denominator());
        let rhs_num_greater = rhs_num > lhs_num;
        let numerator = if rhs_num_greater {
            rhs_num.strict_sub(lhs_num)
        } else {
            lhs_num.strict_sub(rhs_num)
        };

        let mut integer = self.integer().strict_sub(rhs.integer());
        if rhs_num_greater {
            integer = integer.strict_sub(1);
        }

        Beat::new(integer, numerator, denominator)
    }
}

impl PartialOrd for Beat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Beat {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.integer().cmp(&other.integer()) {
            Ordering::Equal => self.numerator()
                .strict_mul(other.denominator())
                .cmp(&other.numerator().strict_mul(self.denominator())),
            other => other
        }
    }
}
