mod synth;

use image::RgbaImage;
use rodio::{OutputStream, Sink, Source};
use std::{error::Error, time::Duration};

use crate::synth::WavetableOscillator;

struct ImageData {
    width: u32,
    height: u32,
    rows: Vec<Vec<[u8; 4]>>,
}

impl ImageData {
    fn new(image: RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let mut rows: Vec<Vec<[u8; 4]>> = Vec::with_capacity(height as usize);

        for y in 0..height {
            let mut row_data: Vec<[u8; 4]> = Vec::with_capacity(width as usize);
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                row_data.push(pixel.0);
            }
            rows.push(row_data);
        }

        ImageData {
            width,
            height,
            rows,
        }
    }

    fn get_row_averages(&self) -> Vec<[f32; 4]> {
        self.rows
            .iter()
            .map(|row| {
                let [r, g, b, a] = row.iter().fold([0 as u32; 4], |mut acc, p| {
                    acc[0] += p[0] as u32;
                    acc[1] += p[1] as u32;
                    acc[2] += p[2] as u32;
                    acc[3] += p[3] as u32;
                    acc
                });

                [
                    (r / self.width) as f32,
                    (g / self.width) as f32,
                    (b / self.width) as f32,
                    (a / self.width) as f32,
                ]
            })
            .collect()
    }
}

pub fn run(image_file_path: &str) -> Result<(), Box<dyn Error>> {
    let img = image::open(image_file_path)
        .expect("Failed to open image")
        .to_rgba8();

    let img = ImageData::new(img);

    let average_rgba = img.get_row_averages();

    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        wave_table.push((2.0 * std::f32::consts::PI * n as f32 / wave_table_size as f32).sin());
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for rgba in average_rgba {
        let [r, g, b, _a] = rgba;

        let oscillator = WavetableOscillator::new(
            2,
            44100,
            vec![
                (wave_table.clone(), r),
                (wave_table.clone(), g),
                (wave_table.clone(), b),
            ],
        );

        sink.append(oscillator.take_duration(Duration::from_millis(30)));
    }

    sink.sleep_until_end();

    Ok(())
}
