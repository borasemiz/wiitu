use imageproc::drawing::draw_text;
use imageproc::definitions::Image;
use rusttype::{Font, Scale, Point};
use image::io::Reader as ImageReader;
use image::{GenericImageView, Rgba};
use std::io::{Result, Cursor};
use std::str::FromStr;
use clap::{Arg, App};

const COLOR: Rgba<u8> = Rgba([255, 255, 255, 1]);

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
  let text: String = String::from_str(text.trim()).unwrap();
  let layout_iter = font.layout(&text, scale, Point{x: 0.0, y: 0.0});
  let mut lines = vec![];
  let mut current_word_start = 0;
  let mut current_word_length = 0;
  let mut finish: u32 = 0;
  let mut finish_previous_line: u32 = 0;
  let mut last_segment_index: usize = 0;

  for (index, glyph) in layout_iter.enumerate() {
    match glyph.pixel_bounding_box() {
      Some(bounding_box ) => {
        current_word_length += bounding_box.width() as u32;
      },
      None => {
        //finish = glyph.position().x as u32 - finish_previous_line;
        if (finish + current_word_length) > img_width {
          let mut split = text.clone();
          split.truncate(current_word_start - 1);
          let split = split.split_off(last_segment_index);
          lines.push(split);
          last_segment_index = current_word_start - 1;
          finish_previous_line += finish;
          finish = 0;
        } else {
          finish = current_word_length + (glyph.position().x as u32 - finish_previous_line);
          current_word_start = index + 1;
          current_word_length = 0;
        }
      }
    }

    if index == text.len() - 1 {
      let mut split = text.clone();
      split.truncate(text.len());
      let split = split.split_off(last_segment_index);
      lines.push(split);
    }
  }

  lines
}

fn get_base_image_template(font: &Font, scale: Scale) -> Image<Rgba<u8>> {
  let mut img = ImageReader::new(
    Cursor::new(include_bytes!("wiitu.jpeg"))
  ).with_guessed_format().unwrap().decode().unwrap();

  let finish = get_text_width(&font, "WHAT IF I TOLD YOU", scale);
  let x = (img.width() / 2) - (finish / 2);
  let out = draw_text(
    &mut img,
    COLOR,
    x,
    10,
    scale,
    &font,
    "WHAT IF I TOLD YOU"
  );

  out
}

fn print_text_into_image(img: Image<Rgba<u8>>, text: &str, font: &Font, scale: Scale) -> Image<Rgba<u8>> {
  let lines = get_text_lines(text, font, scale, img.width());
  let v_metrics = font.v_metrics(scale);
  let mut out = img;

  for (index, line) in lines.into_iter().rev().enumerate() {
    let finish = get_text_width(&font, &line, scale);
    let line_height = (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap) as u32;
    let x = (out.width() / 2) - (finish / 2);
    let y = out.height() - 70 - (line_height * index as u32);

    out = draw_text(
      &mut out,
      COLOR,
      x,
      y,
      scale,
      &font,
      &line
    );
  }

  out
}

fn main() -> Result<()> {
  let matches = App::new("wiitu")
      .version("0.1.0")
      .author("Some Guy <someguy@smail.com>")
      .about("Generates a \"What if I told you that ...\" meme featuring Morpheus in the movie The Matrix")
      .arg(
        Arg::with_name("text")
            .short("t")
            .long("text")
            .value_name("TEXT")
            .help("Enter the text to be shown")
            .required(true)
      )
      .get_matches();

  let text = matches.value_of("text");
  if text == None {
    eprintln!("Error - no text is provided");
    std::process::exit(1);
  }

  let text = text.unwrap();
  let font = Font::try_from_bytes(include_bytes!("font.ttf")).unwrap();
  let scale = Scale{ x: 40.0, y: 40.0 };

  let img = get_base_image_template(&font, scale);
  let img = print_text_into_image(img, text, &font, scale);

  img.save("wiitu_out.jpeg").unwrap();

  Ok(())
}
