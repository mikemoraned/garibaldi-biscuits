#[macro_use]
extern crate imageproc;
extern crate rand;

use image::GenericImage;
use image::ImageBuffer;
use image::Luma;
use std::time::Duration;

use biscuiting_lib::region_labelling::find_contours;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn tiny_random_image(c: &mut Criterion) {
    let mut image = random_test_image(3, 3, 0u64);

    let background_color = Luma([0u32; 1]);

    let sub_image = image.sub_image(0, 0, image.width(), image.height());
    c.bench_function("tiny_random", |b| {
        b.iter(|| find_contours(black_box(background_color), &sub_image))
    });
}

fn medium_random_image(c: &mut Criterion) {
    let mut image = random_test_image(100, 100, 0u64);

    let background_color = Luma([0u32; 1]);

    let sub_image = image.sub_image(0, 0, image.width(), image.height());

    let mut group = c.benchmark_group("custom_group");
    group.measurement_time(Duration::from_secs(7000));
    group.sample_size(500);

    group.bench_function("medium_random", |b| {
        b.iter(|| find_contours(black_box(background_color), &sub_image))
    });

    group.finish();
}

type TestImage = ImageBuffer<Luma<u32>, Vec<u32>>;

fn random_test_image(width: u32, height: u32, seed: u64) -> TestImage {
    use rand::prelude::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut image = TestImage::new(width, height);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let intensity = rng.gen::<u32>();
            image.put_pixel(x, y, Luma::<u32>([intensity; 1]));
        }
    }

    image
}

criterion_group!(
    benches,
    tiny_random_image,
    medium_random_image,
    // large_random_image
);
criterion_main!(benches);
