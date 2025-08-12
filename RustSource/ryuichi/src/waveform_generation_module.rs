use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use hound;
use image::{RgbImage, Rgb};

#[no_mangle]
pub extern "C" fn rust_sound_transform(path: *const c_char, name: *const c_char) -> *const c_char {
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = c_str.to_str().expect("Failed to convert C string to Rust string");

    let c_str_name = unsafe { CStr::from_ptr(name) };
    let name_str = c_str_name.to_str().expect("Failed to convert C string to Rust string");

    let filename = Path::new(name_str)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let output_path = format!("C:/Ryuichi/WaveformImg/{}.png", filename);

    let reader = hound::WavReader::open(path_str).unwrap();
    let samples: Vec<i16> = reader
        .into_samples()
        .filter_map(Result::ok)
        .collect();

    let width: usize = 100;
    let height: u32 = 110;
    let mut img = RgbImage::new(width as u32, height);
    let step = samples.len() / width.max(1);

    for x in 0..width {
        let i = x * step;
        if i < samples.len() {
            let sample = samples[i] as f32 / i16::MAX as f32;
            let y = ((sample + 1.0) / 2.0 * height as f32) as u32;
            img.put_pixel(x as u32, height - y.min(height - 1), Rgb([0, 255, 0]));
        }
    }

    img.save(&output_path).expect("Failed to save image");

    CString::new(output_path)
        .expect("Failed to create CString")
        .into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free_string(s: *mut c_char) {
    unsafe{
        if !s.is_null() {
       drop(CString::from_raw(s));
    }
}
}