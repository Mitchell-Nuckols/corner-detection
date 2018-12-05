extern crate arrayfire as af;
extern crate image;

use std::*;
use af::*;
use image::*;
use path::Path;

#[allow(unused_must_use)]
fn main() {
    println!("There are {:?} available backends", get_backend_count());
    let available = get_available_backends();

    if available.contains(&Backend::CUDA) {
        println!("Evaluating CUDA Backend...");
        set_backend(Backend::CUDA);
        println!("There are {} CUDA compute devices", device_count());
    }

    let mut img_color = load_image::<f32>("3-original_input.jpg".to_string(), true);
    let img = color_space(&img_color, ColorSpace::GRAY, ColorSpace::RGB);
    img_color = &img_color / constant(255.0f32, img_color.dims());

    let mut img_copy = image::open("3-original_input_copy.jpg").unwrap();

    //let mut wnd = Window::new(1920, 1080, "Harris Corner Detection".to_string());

    let (ix, iy) = gradient(&img);

    let mut ixx = &ix * &ix;
    let mut iyy = &iy * &iy;
    let mut ixy = &ix * &iy;

    let guassian = gaussian_kernel(5, 5, 2.0, 2.0);
    ixx = convolve2::<f32, f32>(&ixx, &guassian, ConvMode::DEFAULT, ConvDomain::AUTO);
    iyy = convolve2::<f32, f32>(&iyy, &guassian, ConvMode::DEFAULT, ConvDomain::AUTO);
    ixy = convolve2::<f32, f32>(&ixy, &guassian, ConvMode::DEFAULT, ConvDomain::AUTO);

    let itr = &ixx + &iyy;
    let idet = &ixx * &iyy - &ixy * &ixy;

    let k = 0.04f32;
    let itr_sqr = &itr * &itr;

    let response = idet - &itr_sqr * constant(k, img.dims());

    let mask_arr = constant(1.0f32, Dim4::new(&[3, 3, 1, 1]));
    let max_response = dilate(&response, &mask_arr);

    let thresh = 1e5f32;
    let corner_thresh = constant(thresh, response.dims());

    let mut icorners = gt(&response, &corner_thresh, true);
    icorners = icorners * response;
    icorners = eq(&icorners, &max_response, true) * &icorners;

    let length = icorners.elements() as usize;
    let mut h_corners: Vec<f32> = vec![0.0; length];

    icorners.host::<f32>(&mut h_corners);

    let cross_len = 3;

    let mut goodcorners: u32 = 0;

    let imgsize = (img.dims()[0] * img.dims()[1]) as usize;
    println!("{}", imgsize);
    let mut xvals: Vec<u64> = Vec::new();
    let mut yvals: Vec<u64> = Vec::new();

    for y in cross_len..(img_color.dims()[0]-cross_len) {
        for x in cross_len..(img_color.dims()[1]-cross_len) {
            if &h_corners[(x * icorners.dims()[0] + y) as usize] > &thresh {
                println!("Corner at {}, {}", x, y);

                for dy in 0..8 {
                    for dx in 0..8 {
                        img_copy.put_pixel((x-dx+4) as u32, (y-dy+4) as u32, image::Rgba([0, 255, 0, 255]));
                    }
                }

                goodcorners += 1;
            }
        }
    }

    println!("Found {} corners", goodcorners);

    img_copy.save("corners.jpg").unwrap();

    let iximg = &ix / constant(255.0f32, ix.dims());
    let iyimg = &iy / constant(255.0f32, iy.dims());
    let ixximg = &ixx / constant(255.0f32, ixx.dims());
    let iyyimg = &iyy / constant(255.0f32, iyy.dims());
    let ixyimg = &ixy / constant(255.0f32, ixy.dims());

    save_image::<f32>("DX.jpg".to_string(), &iximg);
    save_image::<f32>("DY.jpg".to_string(), &iyimg);
    save_image::<f32>("DXX.jpg".to_string(), &ixximg);
    save_image::<f32>("DYY.jpg".to_string(), &iyyimg);
    save_image::<f32>("DXY.jpg".to_string(), &ixyimg);

    /*let xarr = Array::new(&xvals, Dim4::new(&[1, (xvals.len() as u64), 1, 1]));
    let yarr = Array::new(&yvals, Dim4::new(&[(yvals.len() as u64), 1, 1, 1]));

    loop {
        wnd.grid(2, 1);

        wnd.set_view(0, 0);
        wnd.draw_image(&img_color, Some("Input Image".to_string()));

        wnd.set_view(1, 0);
        wnd.draw_scatter2(&xarr, &yarr, MarkerType::POINT, Some("Corners".to_string()));

        wnd.show();

        if wnd.is_closed() == true { break; }
    }*/
}