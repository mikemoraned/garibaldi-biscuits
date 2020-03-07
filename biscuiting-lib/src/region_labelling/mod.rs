use image::GenericImage;
use std::collections::HashSet;
mod turtle;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point<T: Copy + PartialEq + Eq> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + PartialEq + Eq> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point::<T> { x, y }
    }
}

pub fn find_contours<I>(background_color: I::Pixel, image: &I) -> Vec<Vec<Point<u32>>>
where
    I: GenericImage,
    I::Pixel: std::cmp::Eq + std::hash::Hash,
{
    let mut colors_seen = HashSet::new();
    let mut points_seen = HashSet::new();
    let mut contours = Vec::new();
    let mut turtle = turtle::Turtle::new(0, 0);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image.get_pixel(x, y);
            if !colors_seen.contains(&color) && color != background_color {
                colors_seen.insert(color);
                points_seen.clear();
                let mut contour = Vec::new();
                turtle.reset(x, y);
                trace_contour(&mut turtle, image, color, &mut contour, &mut points_seen);
                contours.push(contour);
            }
        }
    }
    contours
}

use bit_set::BitSet;
use image::Luma;
use imageproc::definitions::Image;

pub fn find_contours_in_luma(
    background_color: Luma<u32>,
    image: &Image<Luma<u32>>,
) -> Vec<Vec<Point<u32>>> {
    let mut colors_seen = BitSet::new();
    let mut points_seen = HashSet::new();
    let mut contours = Vec::new();
    let mut turtle = turtle::Turtle::new(0, 0);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image.get_pixel(x, y);
            if !colors_seen.contains(color[0] as usize) && *color != background_color {
                colors_seen.insert(color[0] as usize);
                points_seen.clear();
                let mut contour = Vec::new();
                turtle.reset(x, y);
                trace_contour_luma(&mut turtle, image, *color, &mut contour, &mut points_seen);
                contours.push(contour);
            }
        }
    }
    contours
}

pub fn find_contour<I>(foreground_color: I::Pixel, image: &I) -> Option<Vec<Point<u32>>>
where
    I: GenericImage,
    I::Pixel: std::cmp::Eq,
{
    match find_first_foreground_pixel(foreground_color, image) {
        Some(start) => {
            let mut contour = Vec::new();
            trace_contour(
                &mut turtle::Turtle::new(start.x, start.y),
                image,
                foreground_color,
                &mut contour,
                &mut HashSet::new(),
            );
            Some(contour)
        }
        None => None,
    }
}

fn find_first_foreground_pixel<I>(color: I::Pixel, image: &I) -> Option<Point<u32>>
where
    I: GenericImage,
    I::Pixel: std::cmp::Eq,
{
    for y in 0..image.width() {
        for x in 0..image.height() {
            let p = image.get_pixel(x, y);
            if p == color {
                return Some(Point::new(x, y));
            }
        }
    }
    None
}

fn trace_contour<I>(
    start: &mut turtle::Turtle,
    image: &I,
    foreground_color: I::Pixel,
    points: &mut Vec<Point<u32>>,
    points_seen: &mut HashSet<Point<u32>>,
) where
    I: GenericImage,
    I::Pixel: std::cmp::Eq,
{
    let start_point = Point::new(start.x as u32, start.y as u32);
    points.push(start_point);
    points_seen.insert(start_point);

    let mut next = start.left();
    while next != *start {
        if is_in_bounds(next.x, next.y, image)
            && image.get_pixel(next.x as u32, next.y as u32) == foreground_color
        {
            let point = Point::new(next.x as u32, next.y as u32);
            if !points_seen.contains(&point) {
                points.push(point);
                points_seen.insert(point);
            }
            next = next.left();
        } else {
            next = next.right();
        }
    }
}

fn trace_contour_luma(
    start: &mut turtle::Turtle,
    image: &Image<Luma<u32>>,
    foreground_color: Luma<u32>,
    points: &mut Vec<Point<u32>>,
    points_seen: &mut HashSet<Point<u32>>,
) {
    let start_point = Point::new(start.x as u32, start.y as u32);
    points.push(start_point);
    points_seen.insert(start_point);

    let mut next = start.left();
    while next != *start {
        if is_in_bounds(next.x, next.y, image)
            && *image.get_pixel(next.x as u32, next.y as u32) == foreground_color
        {
            let point = Point::new(next.x as u32, next.y as u32);
            if !points_seen.contains(&point) {
                points.push(point);
                points_seen.insert(point);
            }
            next = next.left();
        } else {
            next = next.right();
        }
    }
}

fn is_in_bounds<I>(x: i32, y: i32, image: &I) -> bool
where
    I: GenericImage,
{
    (x >= 0 && x < image.width() as i32) && (y >= 0 && y < image.height() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Luma;

    #[test]
    fn test_with_single_pixel() {
        let mut image = gray_image!(type: u32,
            0,   0,  0;
            0, 255,  0;
            0,   0,  0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![Point::new(1, 1)];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_centered_square() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 0;
            0, 255, 255, 0;
            0, 255, 255, 0;
            0,   0,   0, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(2, 2),
            Point::new(1, 2),
        ];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_hole() {
        let mut image = gray_image!(type: u32,
            0,   0,   0,   0;
            0, 255, 255, 255;
            0, 255,   0, 255;
            0, 255, 255, 255);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(3, 2),
            Point::new(3, 3),
            Point::new(2, 3),
            Point::new(1, 3),
            Point::new(1, 2),
        ];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_l_shape_example_1() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 0;
            0, 255, 255, 0;
            0,   0, 255, 0;
            0,   0,   0, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![Point::new(1, 1), Point::new(2, 1), Point::new(2, 2)];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_l_shape_example_2() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 0;
            0, 255, 255, 0;
            0, 255,   0, 0;
            0,   0,   0, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![Point::new(1, 1), Point::new(2, 1), Point::new(1, 2)];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_l_shape_example_3() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 0;
            0, 255,   0, 0;
            0, 255, 255, 0;
            0,   0,   0, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![Point::new(1, 1), Point::new(1, 2), Point::new(2, 2)];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_l_shape_example_4() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 0;
            0,   0, 255, 0;
            0, 255, 255, 0;
            0,   0,   0, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![Point::new(2, 1), Point::new(2, 2), Point::new(1, 2)];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_x_offset_square() {
        let mut image = gray_image!(type: u32,
            0,   0,   0,   0;
            0,   0, 255, 255;
            0,   0, 255, 255;
            0,   0,   0,   0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(3, 2),
            Point::new(2, 2),
        ];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_y_offset_square() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 0;
            0,   0,   0, 0;
            0, 255, 255, 0;
            0, 255, 255, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(1, 3),
        ];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_with_single_pixel_in_top_left_corner() {
        let mut image = gray_image!(type: u32,
            255, 0;
              0, 0);

        let foreground_color = Luma([255u32; 1]);
        let background_color = Luma([0u32; 1]);

        let expected_contour = vec![Point::new(0, 0)];

        let sub_image = image.sub_image(0, 0, image.width(), image.height());

        let contour = find_contour(foreground_color, &sub_image);

        assert_eq!(Some(expected_contour.clone()), contour);

        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![expected_contour.clone()], contours);
    }

    #[test]
    fn test_find_contours_with_multiple_contours_in_large_example() {
        let mut image = gray_image!(type: u32,
            0,   0, 100, 100;
            0,   0, 100, 100;
            0, 255, 255, 0;
            0, 255, 255, 0);

        let background_color = Luma([0u32; 1]);

        let sub_image = image.sub_image(0, 0, image.width(), image.height());
        let contours = find_contours(background_color, &sub_image);

        assert_eq!(
            vec![
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(3, 1),
                Point::new(2, 1)
            ],
            contours[0]
        );

        assert_eq!(
            vec![
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(2, 3),
                Point::new(1, 3)
            ],
            contours[1]
        );

        assert_eq!(2, contours.len());
    }

    #[test]
    fn test_find_contours_with_multiple_contours_in_minimal_example() {
        let mut image = gray_image!(type: u32,
            0,   0,   0, 100;
            0,   0,   0, 0;
            0,   0,   0, 0;
            255, 0,   0, 0);

        let background_color = Luma([0u32; 1]);

        let sub_image = image.sub_image(0, 0, image.width(), image.height());
        let contours = find_contours(background_color, &sub_image);

        assert_eq!(vec![Point::new(3, 0)], contours[0]);
        assert_eq!(vec![Point::new(0, 3)], contours[1]);

        assert_eq!(2, contours.len());
    }
}
