#[macro_use]
extern crate imageproc;

use image::GenericImage;
use image::ImageBuffer;
use image::Luma;
use imageproc::utils::gray_bench_image;

use biscuiting_lib::region_labelling::find_contours;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

type TestImage = ImageBuffer<Luma<u32>, Vec<u32>>;

fn large_example_benchmark(c: &mut Criterion) {
    let mut image = TestImage::new(1000, 1000);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let intensity = (x % 7 + y % 6) as u32;
            image.put_pixel(x, y, Luma::<u32>([intensity; 1]));
        }
    }

    let background_color = Luma([0u32; 1]);

    let sub_image = image.sub_image(0, 0, image.width(), image.height());
    let contours = c.bench_function("large_example_benchmark", |b| {
        b.iter(|| find_contours(background_color, &sub_image))
    });
}

criterion_group!(benches, tiny_example_benchmark, large_example_benchmark);
criterion_main!(benches);
