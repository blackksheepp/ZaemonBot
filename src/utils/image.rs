use std::{env::current_dir, fs};

use magick_rust::{
    bindings::{GravityType_CenterGravity, GravityType_NorthGravity},
    DrawingWand, PixelWand,
};

pub fn write_text(text: &str, position: &str, filename: &str) -> Result<String, ()> {
    let cd = std::env::current_dir().expect("Failed to get current directory");

    let image_path = cd.join(filename);
    let mut image = magick_rust::MagickWand::new();
    image
        .read_image(image_path.to_str().unwrap())
        .expect("Failed to open image");

    let mut draw = DrawingWand::new();

    let font_path = cd.join("fonts/ProtestStrike-Regular.ttf").to_string_lossy().into_owned();
    draw.set_font(&font_path).unwrap();

    let image_width = image.get_image_width();
    let font_size = (image_width as f64 * 0.1) as f64;
    draw.set_font_size(font_size);

    let mut fill_color = PixelWand::new();
    fill_color.set_color("#FFFFFF").unwrap();
    let mut stroke_color = PixelWand::new();
    stroke_color.set_color("#000000").unwrap();

    draw.set_fill_color(&fill_color);
    draw.set_stroke_color(&stroke_color);


    draw.set_stroke_width(2.0);
    
    draw.set_gravity(GravityType_NorthGravity);
    draw.set_text_alignment(GravityType_CenterGravity);

    let w = image.get_image_width();
    let h = image.get_image_height();

    let wrap_text = textwrap::wrap(text, 20);
    let mut y_axis = 10.0;
    if position == "bottom" {
        y_axis = (h - (wrap_text.len() * 75)) as f64;
    }
    let text = wrap_text.join("\n");

    image
        .annotate_image(&draw, 0.0, y_axis, 0.0, &text)
        .unwrap();

    let mut nw = 512;
    let mut nh = 512;

    if w > h {
        let scale = nw / w;
        nh = h * scale;
    } else {
        let scale = nh / h;
        nw = w * scale;
    }

    image.resize_image(nw, nh, 0);
    let output_path = cd.join(&filename);
    image
        .write_image(output_path.to_str().unwrap())
        .expect("Failed to save image");

    Result::Ok(filename.to_string())
}

pub fn position_text(user: &u64, text: &str, pic: &str) -> Result<String, ()> {
    let filename = format!("temp/{0}-{1}.png", user, pic);
    let cd = current_dir().unwrap();
	let from = cd.join(format!("templates/{0}.png", &pic));
    let to = cd.join(&filename);
	println!("is_file: {}", &from.is_file());
	
    match fs::copy(&from, &to) {
        Ok(_) => {}
        Err(e) => log::error!("[{}] Failed to copy File. Error: {}\nFrom: {}\nTo: {}", user, e, &from.to_str().unwrap(), &to.to_str().unwrap()),
    }
    if let Some(dot) = text.find('.') {
        let (upper, bottom) = text.split_at(dot + 1);
        let upper = &upper[..upper.len()-1];

        write_text(&upper, "top", &filename).map_err(|_| log::error!("Failed to write text")).unwrap();
        write_text(&bottom, "bottom", &filename)
    } else {
        write_text(&text, "top", &filename)
    }
}
