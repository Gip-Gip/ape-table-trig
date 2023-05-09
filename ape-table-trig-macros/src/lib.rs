use std::f32::consts::PI as PI_F32;
use std::f64::consts::PI as PI_F64;

const QUART_CIRC_F32: f32 = 0.5 * PI_F32;
const QUART_CIRC_F64: f64 = 0.5 * PI_F64;

const GEN_LIMIT_F32: f32 = QUART_CIRC_F32;
const GEN_LIMIT_F64: f64 = QUART_CIRC_F64;

use proc_macro::*;

#[proc_macro]
/// Generate a trig table of F32s. Provide the size of the table in usize.
pub fn trig_table_gen_f32(tokens: TokenStream) -> TokenStream {
    let size_str = format!("{}", tokens);

    let size: usize = size_str.parse().expect("Please provide the size of the table as a usize!");

    let mut generated_code: String = "[".to_string();

    for i in 0..size {
        // We only need to calculate the sin up to 0.5π
        let radians = (i as f32)/(size as f32) * GEN_LIMIT_F32;

        generated_code.push_str(&format!("{:e},", radians.sin()));
    }

    generated_code.push(']');
    
    generated_code.parse().expect("Could not parse for some reason!")
}

#[proc_macro]
/// Generate a trig table of F64s. Provide the size of the table in usize.
pub fn trig_table_gen_f64(tokens: TokenStream) -> TokenStream {
    let size_str = format!("{}", tokens);

    let size: usize = size_str.parse().expect("Please provide the size of the table as a usize!");

    let mut generated_code: String = "[".to_string();

    for i in 0..size {
        // We only need to calculate the sin up to 0.5π
        let radians = (i as f64)/(size as f64) * GEN_LIMIT_F64;

        generated_code.push_str(&format!("{:e},", radians.sin()));
    }

    generated_code.push(']');
    
    generated_code.parse().expect("Could not parse for some reason!")
}
