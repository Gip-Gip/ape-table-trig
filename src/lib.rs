#![cfg_attr(not(test), no_std)]

pub use ape_table_trig_macros::*;

pub use core::f32::consts::PI as PI_F32;
pub use core::f64::consts::PI as PI_F64;

pub const QUART_CIRC_F32: f32 = 0.5 * PI_F32;
pub const QUART_CIRC_F64: f64 = 0.5 * PI_F64;

pub const HALF_CIRC_F32: f32 = PI_F32;
pub const HALF_CIRC_F64: f64 = PI_F64;

pub const FULL_CIRC_F32: f32 = 2.0 * PI_F32;
pub const FULL_CIRC_F64: f64 = 2.0 * PI_F64;

pub const GEN_LIMIT_F32: f32 = HALF_CIRC_F32;
pub const GEN_LIMIT_F64: f64 = HALF_CIRC_F64;

#[inline]
pub fn abs_32(float: f32) -> f32 {
    f32::from_ne_bytes(
        (u32::from_ne_bytes(
            float.to_ne_bytes()
        ) & (u32::MAX >> 1)).to_ne_bytes()
    )
}

pub struct TrigTableF32 {
    table: &'static [f32],
}

impl TrigTableF32 {
    pub fn new(table: &'static [f32]) -> Self {
        Self {
            table
        }
    }

    pub fn sin(&self, radians: f32) -> f32 {
        let is_negative = radians < 0.0;
        let radians = abs_32(radians);
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
    pub fn cos(&self, radians: f32) -> f32 {
        self.sin(radians + QUART_CIRC_F32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TABLE_F32: [f32; 2_000] = trig_table_gen_f32!(2000);

    #[test]
    fn test_sin() {
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
}
