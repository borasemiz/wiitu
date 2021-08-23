use imageproc::drawing::draw_text;
use rusttype::{Font, Scale, Point};
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, Rgba};
use std::io::Result;
use std::str::FromStr;

fn get_text_width(font: &Font, text: &str, scale: Scale) -> u32 {
  let layout_iter = font.layout(text, scale, Point{ x: 0.0, y: 0.0 });
  let mut finish: u32 = 0;

  for glyph in layout_iter {
    finish = glyph.position().x as u32;

    if let Some(bounding_box) = glyph.pixel_bounding_box() {
      finish += bounding_box.width() as u32;
    }
  }

  finish
}

fn get_text_lines(text: &str, font: &Font, scale: Scale, img_width: u32) -> Vec<String> {
  let layout_iter = font.layout(text, scale, Point{x: 0.0, y: 0.0});
  let mut lines = vec![];
  let mut finish: u32 = 0;
  let mut last_chr_index: usize = 0;

  for (index, glyph) in layout_iter.enumerate() {
    finish = glyph.position().x as u32;

    match glyph.pixel_bounding_box() {
        Some(bounding_box ) => finish += bounding_box.width() as u32,
        None => {
          if finish > img_width {
            let mut splitted = String::from_str(text).unwrap();
            splitted.truncate(index);
            let splitted = splitted.split_off(last_chr_index);
            lines.push(splitted);
            last_chr_index = index;
            finish = 0;
          }
        }
    }
  }

  lines
}

fn main() -> Result<()> {
  let mut img = ImageReader::open("wiitu.jpeg")?.decode().unwrap();
  let font_data = include_bytes!("font.ttf");
  let font = Font::try_from_bytes(font_data).unwrap();
  let scale = Scale{ x: 40.0, y: 40.0 };
  let color: Rgba<u8> = Rgba([255, 255, 255, 1]);

  let finish = get_text_width(&font, "WHAT IF I TOLD YOU", scale);
  let x = (img.width() / 2) - (finish / 2);
  let mut out = draw_text(
    &mut img,
    color, 
    x,
    10,
    scale,
    &font,
    "WHAT IF I TOLD YOU"
  );

  let lines = get_text_lines("WHAT IF I TOLD YOU THAT YOU CAN SLEEP WITHOUT SNORING QWDQ QWDQWDQ QWD QWQWD QQWEQ WE", &font, scale, img.width());
  println!("{:?}", lines);

  let finish: u32 = get_text_width(&font, "THAT YOU CAN SLEEP WITHOUT SNORING", scale);
  let x = (img.width() / 2) - (finish / 2);
  let out = draw_text(
    &mut out,
    color,
    x,
    img.height() - 70,
    scale,
    &font,
    "THAT YOU CAN SLEEP WITHOUT SNORING"
  );

  out.save("wiitu_out.jpeg").unwrap();

  Ok(())
}
