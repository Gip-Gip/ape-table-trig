//! ## *Implementations of sin, cos, and tan using precalculated tables.*
//!
//! Using these functions can significantly improve performance on systems with
//! limited to no naitive floating point support, like the RP2040. Designed to
//! be no_std compatible out of the box.
//!
//! # Example:
//!
//! ```rust
//! use ape_table_trig::*;
//!
//! static TABLE: [f32; 1000] = trig_table_gen_f32!(1000);
//!
//! fn main() {
//!     let table = TrigTableF32::new(&TABLE);
//!
//!     // Calculate the sine of 1π radians
//!     let sine = table.sin(PI_F32);
//! }

#![cfg_attr(not(test), no_std)]

pub use ape_table_trig_macros::*;

pub use core::f32::consts::PI as PI_F32;
pub use core::f64::consts::PI as PI_F64;

/// Quarter circumference in radians, equal to ½π. F32.
pub const QUART_CIRC_F32: f32 = 0.5 * PI_F32;
/// Quarter circumference in radians, equal to ½π. F64.
pub const QUART_CIRC_F64: f64 = 0.5 * PI_F64;

/// Half circumference in radians, equal to 1π. F32.
pub const HALF_CIRC_F32: f32 = PI_F32;
/// Half circumference in radians, equal to 1π. F64.
pub const HALF_CIRC_F64: f64 = PI_F64;

/// Full circumference in radians, equal to 2π. F32.
pub const FULL_CIRC_F32: f32 = 2.0 * PI_F32;
/// Full circumference in radians, equal to 2π. F32.
pub const FULL_CIRC_F64: f64 = 2.0 * PI_F64;

/// Generation limit for the trig table. Currently the table generation generates
/// sin(0)..sin(1π), and then uses some math to finesse the table to work for all
/// other values. F32.
pub const GEN_LIMIT_F32: f32 = HALF_CIRC_F32;
/// Generation limit for the trig table. F64
pub const GEN_LIMIT_F64: f64 = HALF_CIRC_F64;

#[inline]
/// Get the absolute value of a float. F32.
pub fn abs_f32(float: f32) -> f32 {
    // Ungodly ugly, but it optimises down really well
    f32::from_ne_bytes(
        (u32::from_ne_bytes(
            float.to_ne_bytes()
        ) & (u32::MAX >> 1)).to_ne_bytes()
    )
}

#[inline]
/// Get the absolute value of a float. F64.
pub fn abs_f64(float: f64) -> f64 {
    // Ungodly ugly, but it optimises down really well
    f64::from_ne_bytes(
        (u64::from_ne_bytes(
            float.to_ne_bytes()
        ) & (u64::MAX >> 1)).to_ne_bytes()
    )
}

/// Used to perform sin, cos, and tan functions on trig tables. F32.
pub struct TrigTableF32 {
    table: &'static [f32],
}

impl TrigTableF32 {
    /// Create a table struct with a reference to a static table.
    pub fn new(table: &'static [f32]) -> Self {
        Self {
            table
        }
    }

    /// Calculate the approximate sine of the radians provided.
    pub fn sin(&self, radians: f32) -> f32 {
        let is_negative = radians < 0.0;
        let radians = abs_f32(radians);
        let rad_mod = radians % GEN_LIMIT_F32;

        let table_len = self.table.len();

        // Add 0.5 to do a quick round...
        let index = (((rad_mod / GEN_LIMIT_F32) * (table_len as f32)) + 0.5) as usize;

        let sin = self.table[index % table_len];

        let is_negative = is_negative ^ (((radians / GEN_LIMIT_F32) as u32) % 2 == 1);

        match is_negative {
            true => -sin,
            false => sin,
        }
    }

    #[inline]
    /// Calculate the approximate cosine of the radians provided.
    pub fn cos(&self, radians: f32) -> f32 {
        self.sin(radians + QUART_CIRC_F32)
    }

    #[inline]
    /// Calculate the approximate tangent of the radians provided.
    pub fn tan(&self, radians: f32) -> f32 {
        self.sin(radians) / self.cos(radians)
    }
}

/// Used to perform sin, cos, and tan functions on trig tables. F64.
pub struct TrigTableF64 {
    table: &'static [f64],
}

impl TrigTableF64 {
    /// Create a table struct with a reference to a static table.
    pub fn new(table: &'static [f64]) -> Self {
        Self {
            table
        }
    }

    /// Calculate the approximate sine of the radians provided.
    pub fn sin(&self, radians: f64) -> f64 {
        let is_negative = radians < 0.0;
        let radians = abs_f64(radians);
        let rad_mod = radians % GEN_LIMIT_F64;

        let table_len = self.table.len();

        // Add 0.5 to do a quick round...
        let index = (((rad_mod / GEN_LIMIT_F64) * (table_len as f64)) + 0.5) as usize;

        let sin = self.table[index % table_len];

        let is_negative = is_negative ^ (((radians / GEN_LIMIT_F64) as u64) % 2 == 1);

        match is_negative {
            true => -sin,
            false => sin,
        }
    }

    #[inline]
    /// Calculate the approximate cosine of the radians provided.
    pub fn cos(&self, radians: f64) -> f64 {
        self.sin(radians + QUART_CIRC_F64)
    }

    #[inline]
    /// Calculate the approximate tangent of the radians provided.
    pub fn tan(&self, radians: f64) -> f64 {
        self.sin(radians) / self.cos(radians)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TABLE_F32: [f32; 2_000] = trig_table_gen_f32!(2000);
    static TABLE_F64: [f64; 2_000_000] = trig_table_gen_f64!(2000000);

    #[test]
    fn test_sin_f32() {
        let table = TrigTableF32::new(&TABLE_F32);

        // We can only test half the table since floating point innacuracy
        // kicks in past the GEN_LIMIT
        for i in 0..2000 {
            // Go through the table and verify everything
            let radians = (i as f32)/4000.0 * FULL_CIRC_F32;

            let sin1 = radians.sin();
            let sin2 = table.sin(radians);

            assert_eq!(sin1, sin2, "\ni={}", i);
        }

        // Ensure the quadrants work as expected
        assert_eq!(table.sin(QUART_CIRC_F32), 1.0);
        assert_eq!(table.sin(HALF_CIRC_F32), 0.0);
        assert_eq!(table.sin(HALF_CIRC_F32 + QUART_CIRC_F32), -1.0);
        assert_eq!(table.sin(FULL_CIRC_F32), 0.0);
        
        assert_eq!(table.sin(-QUART_CIRC_F32), -1.0);
        assert_eq!(table.sin(-HALF_CIRC_F32), 0.0);
        assert_eq!(table.sin(-HALF_CIRC_F32 - QUART_CIRC_F32), 1.0);
        assert_eq!(table.sin(-FULL_CIRC_F32), 0.0);
    }
    
    #[test]
    fn test_sin_f64() {
        let table = TrigTableF64::new(&TABLE_F64);

        // We can only test half the table since floating point innacuracy
        // kicks in past the GEN_LIMIT
        for i in 0..2_000_000 {
            // Go through the table and verify everything
            let radians = (i as f64)/4_000_000.0 * FULL_CIRC_F64;

            let sin1 = radians.sin();
            let sin2 = table.sin(radians);

            assert_eq!(sin1, sin2, "\ni={}", i);
        }

        // Ensure the quadrants work as expected
        assert_eq!(table.sin(QUART_CIRC_F64), 1.0);
        assert_eq!(table.sin(HALF_CIRC_F64), 0.0);
        assert_eq!(table.sin(HALF_CIRC_F64 + QUART_CIRC_F64), -1.0);
        assert_eq!(table.sin(FULL_CIRC_F64), 0.0);
        
        assert_eq!(table.sin(-QUART_CIRC_F64), -1.0);
        assert_eq!(table.sin(-HALF_CIRC_F64), 0.0);
        assert_eq!(table.sin(-HALF_CIRC_F64 - QUART_CIRC_F64), 1.0);
        assert_eq!(table.sin(-FULL_CIRC_F64), 0.0);
    }
}
