#[macro_use]
extern crate imageproc;

use image::GenericImage;

use biscuiting_lib::region_labelling::find_contours;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::Luma;

fn tiny_example_benchmark(c: &mut Criterion) {
    let mut image = gray_image!(type: u32,
        0,   0, 100, 100;
        0,   0, 100, 100;
        0, 255, 255, 0;
        0, 255, 255, 0);

    let background_color = Luma([0u32; 1]);

    let sub_image = image.sub_image(0, 0, image.width(), image.height());
    let contours = c.bench_function("tiny_example_benchmark", |b| {
        b.iter(|| find_contours(background_color, &sub_image))
    });
}

criterion_group!(benches, tiny_example_benchmark);
criterion_main!(benches);
