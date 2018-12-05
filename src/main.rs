extern crate image;

use image::*;
use std::num::*;

pub mod ops {
    pub fn convolve_focus(mat: &[f32; 9], img: &image::GrayImage) -> image::GrayImage {
        let mut new_img: image::GrayImage = image::GrayImage::new(img.width(), img.height());
        
        for (x, y, p) in new_img.enumerate_pixels_mut() {
            let mut acc: f32 = 0.0;
            let mut mat_sum: f32 = 0.0;

            for j in 0..3 {
                for i in 0..3 {
                    println!("{},{}: {},{}", x, y, i, j);
                    println!("{}", acc);
                    let c = mat[((j * 3) + i) as usize];
                    mat_sum += c;

                    println!("Checking");
                    if (x == 0 && i < 1) ||
                        (y == 0 && j < 1) ||
                        (c == 0f32) {continue;}
                    if (x * y == img.width() * img.height()) {break;}
                    println!("Checked");
                    acc += c * (img.get_pixel((x+i)-1, (y+j)-1).data[0] as f32);
                }
            }

            *p = image::Luma([acc as u8]);
        }

        new_img
    }

    pub const SOBELX: [f32; 9] = [
        1.0, 0.0, -1.0,
        2.0, 0.0, -2.0,
        1.0, 0.0, -1.0
    ];

    pub fn sobelx(img: &image::GrayImage) -> image::GrayImage {
        convolve_focus(&SOBELX, img)
    }

    pub const SOBELY: [f32; 9] = [
        1.0, 2.0, 1.0,
        0.0, 0.0, 0.0,
        -1.0, -2.0, -1.0
    ];

    pub fn sobely(img: &image::GrayImage) -> image::GrayImage {
        convolve_focus(&SOBELY, img)
    }

    pub fn rgb_to_gray(img: &image::RgbImage) -> image::GrayImage {
        let mut gray_image: image::GrayImage = image::GrayImage::new(img.width(), img.height());

        for (x, y, p) in gray_image.enumerate_pixels_mut() {
            let c = 0.30 * (img.get_pixel(x, y).data[0] as f32)
                    + 0.59 * (img.get_pixel(x, y).data[1] as f32)
                    + 0.11 * (img.get_pixel(x, y).data[2] as f32);

            *p = image::Luma([c as u8]);
        }

        gray_image
    }
}

use ops::*;

fn main() {
    let img = image::open("test.jpg").unwrap();//.to_rgb();
    let gray = img.grayscale();

    let ix = gray.filter3x3(&ops::SOBELX);
    let iy = gray.filter3x3(&ops::SOBELY);

    let ixx = {
        let mut temp = image::GrayImage::new(ix.width(), ix.height());
        let ixtemp = ix.to_luma();

        for (x, y, p) in temp.enumerate_pixels_mut() {
            *p = image::Luma([(ixtemp.get_pixel(x, y).data[0] as f32 * ixtemp.get_pixel(x, y).data[0] as f32) as u8]);
        }

        temp
    };

    ix.save("ix.jpg").unwrap();
    iy.save("iy.jpg").unwrap();
    ixx.save("ixx.jpg").unwrap();
}