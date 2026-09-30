use ape_table_trig::*;
use criterion::*;
use libm::{sin as sinf2, sinf};

static TABLE_F32: TrigTableF32 = ape_table_trig_macros::trig_table_gen_f32!(1000);
static TABLE_F64: TrigTableF64 = ape_table_trig_macros::trig_table_gen_f64!(1000000);

pub fn sin_benchmark(c: &mut Criterion) {
    const RANGE: f32 = 4_000.0;

    c.bench_function("Libm sin 0..4_000", |b: &mut Bencher| {
        b.iter(|| {
            for i in 0..(RANGE as u32) {
                let radian = (i as f32 / RANGE) * FULL_CIRC_F32;
                sinf(radian);
            }
        })
    });

    c.bench_function("Table sin 0..4_000", |b: &mut Bencher| {
        b.iter(|| {
            for i in 0..(RANGE as u32) {
                let radian = (i as f32 / RANGE) * FULL_CIRC_F32;
                TABLE_F32.sin(radian);
            }
        })
    });

    const RANGE_2: f64 = 4_000_000.0;

    c.bench_function("Libm sin 0..4_000_000", |b: &mut Bencher| {
        b.iter(|| {
            for i in 0..(RANGE_2 as u32) {
                let radian = (i as f64 / RANGE_2) * FULL_CIRC_F64;
                sinf2(radian);
            }
        })
    });

    c.bench_function("Table sin 0..4_000_000", |b: &mut Bencher| {
        b.iter(|| {
            for i in 0..(RANGE_2 as u32) {
                let radian = (i as f64 / RANGE_2) * FULL_CIRC_F64;
                TABLE_F64.sin(radian);
            }
        })
    });
}

criterion_group!(sin, sin_benchmark);
criterion_main!(sin);
