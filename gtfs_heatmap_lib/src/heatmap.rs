use std::sync::WaitTimeoutResult;

use image::{GenericImage, GrayAlphaImage, GrayImage, ImageBuffer, Luma, LumaA, Pixel};
use rusqlite::Connection;

use crate::{
    coords::{self, Coordinates, TileNumbers},
    get_nearby_stops,
    gtfs_types::Stop,
};

const WALKING_SPEED: f64 = 1.4;

pub fn generate_heatmap_tile(
    zoom: u32,
    tile_x: u32,
    tile_y: u32,
    connection: Connection,
) -> GrayAlphaImage {
    let mut image = GrayAlphaImage::new(256, 256);

    let mut nearby_stops: Vec<Stop> = Vec::new();

    let search_radius = 0.026;

    for x in 0..image.width() {
        for y in 0..image.height() {
            if (x % 16 == 0) && (y % 16 == 0) {
                nearby_stops = get_nearby_stops(
                    &connection,
                    TileNumbers::get_pixel_coordinates(
                        &TileNumbers {
                            zoom,
                            x: tile_x,
                            y: tile_y,
                        },
                        x + 16,
                        y + 16,
                    ),
                    search_radius,
                )
                .expect("Database Borky");
            }

            image.put_pixel(
                x,
                y,
                LumaA([
                    0,
                    get_pixel_brightenss(
                        x,
                        y,
                        &nearby_stops,
                        TileNumbers {
                            zoom,
                            x: tile_x,
                            y: tile_y,
                        },
                    ),
                ]),
            )
        }
    }

    image
}

fn get_pixel_brightenss(x: u32, y: u32, nearby_stops: &Vec<Stop>, current_tile: TileNumbers) -> u8 {
    let closest_distance = nearby_stops.iter().fold(f64::INFINITY, |acc, stop| -> f64 {
        let distance = stop
            .coordinates
            .distance(current_tile.get_pixel_coordinates(x, y));

        if distance < acc {
            return distance;
        } else {
            acc
        }
    });

    let alpha = (closest_distance / WALKING_SPEED) as u8;
    alpha
}
