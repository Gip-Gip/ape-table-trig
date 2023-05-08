use ape_table_trig::*;
use libm::sinf;
use criterion::*;

static TABLE_F32: [f32; 1_000] = trig_table_gen_f32!(1000);

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
}

criterion_group!(sin, sin_benchmark);
criterion_main!(sin);
