use ape_table_trig::*;
use libm::{sin as sinf2, sinf};
use criterion::*;

static TABLE_F32: [f32; 2_000] = trig_table_gen_f32!(2000);
static TABLE_F64: [f64; 2_000_000] = trig_table_gen_f64!(2000000);

pub fn sin_benchmark(c: &mut Criterion) {
    let table = TrigTableF32::new(&TABLE_F32);

    const RANGE: f32 = 4_000.0;

    c.bench_function("Libm sin 0..4_000", |b: &mut Bencher| b.iter(|| {
        for i in 0..(RANGE as u32) {
            let radian = (i as f32 / RANGE) * FULL_CIRC_F32;
            sinf(radian);
        }
    }));
    
    c.bench_function("Table sin 0..4_000", |b: &mut Bencher| b.iter(|| {
        for i in 0..(RANGE as u32) {
            let radian = (i as f32 / RANGE) * FULL_CIRC_F32;
            table.sin(radian);
        }
    }));
    
    let table = TrigTableF64::new(&TABLE_F64);

    const RANGE_2: f64 = 4_000_000.0;

    c.bench_function("Libm sin 0..4_000_000", |b: &mut Bencher| b.iter(|| {
        for i in 0..(RANGE_2 as u32) {
            let radian = (i as f64 / RANGE_2) * FULL_CIRC_F64;
            sinf2(radian);
        }
    }));
    
    c.bench_function("Table sin 0..4_000_000", |b: &mut Bencher| b.iter(|| {
        for i in 0..(RANGE_2 as u32) {
            let radian = (i as f64 / RANGE_2) * FULL_CIRC_F64;
            table.sin(radian);
        }
    }));
}

criterion_group!(sin, sin_benchmark);
criterion_main!(sin);
