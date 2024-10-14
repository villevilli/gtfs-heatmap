use core::f64;
use std::{collections::HashMap, i64, time::Instant};
use time::Duration;

use image::{ColorType, GrayImage, RgbImage};

use crate::coords::TileNumbers;

use super::{dijkstras::StopWithDuration, GtfsGraph};

const TILE_RESOLUTION: u32 = 256;
const WALKING_SPEED: f64 = 1.0;
const MAX_WALKING_TIME: i64 = Duration::minutes(45).whole_seconds();

enum Rgb {
    R,
    G,
    B,
}

impl GtfsGraph {
    pub fn generate_heatmap_tile(
        &self,
        zoom: u32,
        tile_x: u32,
        tile_y: u32,
        stop_times: &HashMap<String, StopWithDuration>,
    ) -> (GrayImage, String) {
        let mut buf = GrayImage::new(TILE_RESOLUTION, TILE_RESOLUTION);
        let tile = TileNumbers {
            zoom,
            x: tile_x,
            y: tile_y,
        };

        let start = Instant::now();

        let max_time = stop_times.values().fold(i64::MIN, |acc, stop_duration| {
            stop_duration.duration.whole_seconds().max(acc)
        }) + MAX_WALKING_TIME;

        let max_time_time = start.elapsed();

        buf.enumerate_pixels_mut()
            .for_each(|(pixel_x, pixel_y, mut pixel)| {
                pixel.0 = [calculate_pixel_brightness(
                    pixel_x, pixel_y, &tile, stop_times, max_time,
                )]
            });

        let img_gen_time = start.elapsed() - max_time_time;

        (
            buf,
            format!(
                "Time used to generate max time : {:?}\nTime used to draw image: {:?}\nMax time: {:?}",
                max_time_time, img_gen_time, max_time
            ),
        )
    }
}

fn calculate_pixel_brightness(
    pixel_x: u32,
    pixel_y: u32,
    tile: &TileNumbers,
    stops: &HashMap<String, StopWithDuration>,
    max_time_sec: i64,
) -> u8 {
    let pixel_coords = tile.get_pixel_coordinates(pixel_x, pixel_y);

    let time: Duration = stops.values().fold(Duration::MAX, |acc, stop| {
        (stop.duration
            + Duration::seconds_f64(
                stop.stop
                    .read()
                    .unwrap()
                    .coordinates
                    .haversine_distance(&pixel_coords)
                    * WALKING_SPEED,
            ))
        .min(acc)
    });

    let brightness = (time.whole_seconds() * 255) / (max_time_sec);
    if brightness > u8::MAX as i64 {
        u8::MAX
    } else {
        brightness as u8
    }
}

/*

fn hue_to_rgb(hue: u8) -> [u8; 3] {
    use Rgb::*;

    [hue_math(R, hue), hue_math(G, hue), hue_math(B, hue)]
}

fn hue_math(rgb: Rgb, hue: u8) -> u8 {
    let n = hue / 46;

    match rgb {
        Rgb::R => {}
        Rgb::G => {}
        Rgb::B => {}
    }
}

*/
